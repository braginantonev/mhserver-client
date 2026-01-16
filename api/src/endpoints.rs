// mhserver api version: 1.3.0 (dev)

pub const API_URL: &str = "/api/v1";

pub const PING: &str = "/ping";

pub mod auth {
    pub const LOGIN: &str = "/users/login";
    pub const REGISTRATION: &str = "/users/register";
}

pub mod data {
    pub const CREATE_CONNECTION: &str = "/files/connect";
    pub const SAVE_FILE: &str = "/files/{uuid}/save";
    pub const GET_FILE: &str = "/files/{uuid}/get";
    pub const GET_SUM: &str = "/files/{uuid}/sum";
    pub const GET_FILES_LIST: &str = "/files/list";
    pub const GET_AVAILABLE_LIST: &str = "/files/space";
}