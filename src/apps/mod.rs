use poem::{get, post, EndpointExt, Route};

use crate::middleware::Auth;

pub mod system;

pub fn api() -> Route {
    Route::new()
        .nest("/comm", no_auth_api()) // 无需授权Api
        .nest("/system", system::system_api().with(Auth)) // 系统管理模块
}

//

pub fn no_auth_api() -> Route {
    Route::new()
        .at("/login", post(system::SysLogin)) // 登录
        .at("/get_captcha", get(system::get_captcha)) // 获取验证码
        .at("/log_out", post(system::log_out)) // 退出登录
}