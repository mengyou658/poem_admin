// use core::time::Duration;
// use std::time::Instant;
//
// use chrono::Local;
// use configs::CFG;
// use db::{
//     common::{ctx::ReqCtx, res::ResJsonString},
//     db_conn,
//     system::entities::{prelude::SysOperLog, sys_oper_log},
//     DB,
// };
// use sea_orm::{EntityTrait, Set};
//
// use crate::{apps::system::check_user_online, utils::api_utils::ALL_APIS};
//
// use anyhow::Result;
// use std::future::{ready, Ready};
//
// use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
// use actix_web::body::BoxBody;
// use actix_web::http::StatusCode;
// use futures::future::LocalBoxFuture;
//
// // There are two steps in middleware processing.
// // 1. Middleware initialization, middleware factory gets called with
// //    next service in chain as parameter.
// // 2. Middleware's call method gets called with normal request.
// pub struct OperLog;
//
// // Middleware factory is `Transform` trait
// // `S` - type of the next service
// // `B` - type of response's body
// impl<S, B> Transform<S, ServiceRequest> for OperLog
//   where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//   type Response = ServiceResponse<B>;
//   type Error = Error;
//   type InitError = ();
//   type Transform = OperLogMiddleware<S>;
//   type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//   fn new_transform(&self, inner: S) -> Self::Future {
//     ready(Ok(OperLogMiddleware { inner }))
//   }
// }
//
// pub struct OperLogMiddleware<S> {
//   inner: S,
// }
//
// impl<S, B> Service<ServiceRequest> for OperLogMiddleware<S>
//   where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//   type Response = ServiceResponse<B>;
//   type Error = Error;
//   type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
//
//   forward_ready!(inner);
//
//   fn call(&self, req: ServiceRequest) -> Self::Future {
//     // 开始请求数据
//     let now = Instant::now();
//     let fut = self.inner.call(req);
//
//     let req_ctx = match req.extensions().get::<ReqCtx>() {
//       Some(x) => x.clone(),
//       None => {
//         return Box::pin(async move {
//           let res_end = fut.await?;
//           Ok(res_end)
//         });
//       }
//     };
//
//     Box::pin(async move {
//       let res_end: ServiceResponse<_> = fut.await?;
//       let (_, res) = &res_end.into_parts();
//       let duration = now.elapsed();
//       // 请求结束 记录日志
//       let code = res_end.status().clone();
//       let res_ctx = match res.extensions().get::<ResJsonString>() {
//         Some(x) => x.0.clone(),
//         None => "".to_string(),
//       };
//       match code {
//         StatusCode::OK => {
//           oper_log_add(req_ctx, res_ctx, "1".to_string(), "".to_string(), duration).await;
//         }
//         _ => {
//           oper_log_add(req_ctx, "".to_string(), "0".to_string(), res_ctx, duration).await;
//         }
//       }
//       Ok(res_end)
//     })
//   }
// }
//
// /// req上下文注入中间件 同时进行jwt授权验证 以及日志记录
//
// pub async fn oper_log_add(req: ReqCtx, res: String, status: String, err_msg: String, duration: Duration) {
//     tokio::spawn(async move {
//         match oper_log_add_fn(req, res, status, err_msg, duration).await {
//             Ok(_) => {}
//             Err(e) => {
//                 tracing::info!("日志添加失败：{}", e.to_string());
//             }
//         };
//     });
// }
//
// /// add 添加
// pub async fn oper_log_add_fn(req: ReqCtx, res: String, status: String, err_msg: String, duration: Duration) -> Result<()> {
//     if !CFG.log.enable_oper_log {
//         return Ok(());
//     }
//     let apis = ALL_APIS.lock().await;
//     let (api_name, is_log) = match apis.get(&req.path) {
//         Some(x) => (x.name.clone(), x.log_method.clone()),
//         None => ("".to_string(), "0".to_string()),
//     };
//     drop(apis);
//     let now = Local::now().naive_local();
//     // 打印日志
//     let req_data = req.clone();
//     let res_data = res.clone();
//     let err_msg_data = err_msg.clone();
//     let duration_data = duration;
//     match is_log.as_str() {
//         "1" => {
//             tokio::spawn(async move {
//                 file_log(req_data, now, duration_data, res_data, err_msg_data);
//             });
//         }
//         "2" => {
//             tokio::spawn(async move {
//                 match db_log(duration_data, req, now, api_name, res, status, err_msg).await {
//                     Ok(_) => {}
//                     Err(e) => {
//                         tracing::info!("日志添加失败：{}", e.to_string());
//                     }
//                 };
//             });
//         }
//         "3" => {
//             tokio::spawn(async move {
//                 file_log(req_data, now, duration_data, res_data, err_msg_data);
//                 match db_log(duration_data, req, now, api_name, res, status, err_msg).await {
//                     Ok(_) => {}
//                     Err(e) => {
//                         tracing::info!("日志添加失败：{}", e.to_string());
//                     }
//                 };
//             });
//         }
//         _ => return Ok(()),
//     }
//
//     Ok(())
// }
//
// async fn db_log(duration: Duration, req: ReqCtx, now: chrono::NaiveDateTime, api_name: String, res: String, status: String, err_msg: String) -> Result<()> {
//     let d = duration.as_micros() as i64;
//     let db = DB.get_or_init(db_conn).await;
//     let (_, m) = check_user_online(Some(db), req.user.token_id.clone()).await;
//     let user = match m {
//         Some(x) => x,
//         None => return Ok(()),
//     };
//     let operator_type = match req.method.as_str() {
//         "GET" => "1",    // 查询
//         "POST" => "2",   // 新增
//         "PUT" => "3",    // 修改
//         "DELETE" => "4", // 删除
//         _ => "0",        // 其他
//     };
//     let add_data = sys_oper_log::ActiveModel {
//         oper_id: Set(scru128::scru128_string()),
//         time_id: Set(now.timestamp()),
//         title: Set(api_name),
//         business_type: Set("".to_string()),
//         method: Set(req.path),
//         request_method: Set(req.method),
//         operator_type: Set(operator_type.to_string()),
//         oper_name: Set(req.user.name),
//         dept_name: Set(user.dept_name),
//         oper_url: Set(req.ori_uri),
//         oper_ip: Set(user.ipaddr),
//         oper_location: Set(user.login_location),
//         oper_param: Set(if req.data.len() > 10000 { "数据太长不保存".to_string() } else { req.data }),
//         json_result: Set(if res.len() > 100000 { "数据太长不保存".to_string() } else { res }),
//         path_param: Set(req.path_params),
//         status: Set(status),
//         error_msg: Set(err_msg),
//         duration: Set(d),
//         oper_time: Set(now),
//     };
//     SysOperLog::insert(add_data).exec(db).await.expect("oper_log_add error");
//     Ok(())
// }
//
// fn file_log(req_data: ReqCtx, now: chrono::NaiveDateTime, duration_data: Duration, res_data: String, err_msg_data: String) {
//     tracing::info!(
//         "\n请求路径:{:?}\n完成时间:{:?}\n消耗时间:{:?}微秒 | {:?}毫秒\n请求数据:{:?}\n响应数据:{}\n错误信息:{:?}\n",
//         req_data.path.clone(),
//         now,
//         duration_data.as_micros(),
//         duration_data.as_millis(),
//         req_data,
//         res_data,
//         err_msg_data,
//     );
// }
