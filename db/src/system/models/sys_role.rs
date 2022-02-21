use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct SearchReq {
    pub role_id: Option<String>,
    pub role_ids: Option<Vec<String>>,
    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct AddReq {
    pub role_name: String,
    pub role_key: String,
    pub list_order: i32,
    pub data_scope: Option<String>,
    pub status: Option<String>,
    #[validate(length(min = 1))]
    pub remark: Option<String>,
    pub menu_ids: Vec<String>,
}

#[derive(Deserialize, Validate)]
pub struct DeleteReq {
    pub role_ids: Vec<String>,
}
#[derive(Deserialize, Validate)]
pub struct DataScopeReq {
    pub role_id: String,
    pub data_scope: String,
    pub dept_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct EditReq {
    pub role_id: String,
    pub role_name: String,
    pub role_key: String,
    pub list_order: i32,
    pub data_scope: String,
    pub status: String,
    pub remark: String,
    pub menu_ids: Vec<String>,
}
#[derive(Deserialize, Clone, Validate)]
pub struct StatusReq {
    pub role_id: String,
    pub status: String,
}
#[derive(Deserialize, Clone)]
pub struct UpdateAuthRoleReq {
    pub user_id: String,
    pub role_ids: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct AddOrCancelAuthRoleReq {
    pub user_ids: Vec<String>,
    pub role_id: String,
}

#[derive(Debug, Serialize, Validate, FromQueryResult, Clone)]
pub struct Resp {
    pub role_id: String,
    pub role_name: String,
    pub role_key: String,
    pub status: String,
    pub list_order: i32,
    pub remark: String,
    pub data_scope: String,
}
