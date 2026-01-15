// mhserver api version: 1.2.0

pub const API_URL: &str = "/api/v1";

pub mod auth {
    pub const LOGIN: &str = "/users/login";
    pub const REGISTRATION: &str = "/users/register";
}

pub mod data {
    pub const CREATE_CONNECTION: &str = "/files/connect";
    pub const SAVE_FILE: &str = "/files/{uuid}/save";
    pub const GET_FILE: &str = "/files/{uuid}/get";
    pub const GET_SUM: &str = "/files/{uuid}/sum";
}