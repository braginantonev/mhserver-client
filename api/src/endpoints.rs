// mhserver api version: 1.3.0

const API_URL: &str = "/api/v1";

mod auth {
    const LOGIN: &str = "/users/login";
    const REGISTRATION: &str = "/users/register";
}

mod data {
    const CREATE_CONNECTION: &str = "/files/connect";
    const SAVE_FILE: &str = "/files/{uuid}/save";
    const GET_FILE: &str = "/files/{uuid}/get";
    const GET_SUM: &str = "/files/{uuid}/sum";
}