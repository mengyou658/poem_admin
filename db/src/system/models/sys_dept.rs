use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub dept_id: Option<String>,
    pub dept_name: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct DeleteReq {
    pub dept_id: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EditReq {
    pub dept_id: String,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, FromQueryResult, Default, Deserialize)]
pub struct DeptResp {
    pub dept_id: String,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct RespTree {
    #[serde(flatten)]
    pub data: DeptResp,
    pub children: Option<Vec<RespTree>>,
}
