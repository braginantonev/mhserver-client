// mhserver api version: 1.3.0 (dev)

pub const API_URL: &str = "/api/v1";

/// Method - POST
pub const PING: &str = "/ping";

/// Endpoints to auth service
pub mod auth {

    /// Method - GET
    pub const LOGIN: &str = "/users/login";

    /// Method - POST
    pub const REGISTRATION: &str = "/users/register";
}

/// Endpoints for data service
pub mod data {

    /// Method - OPTIONS
    pub const CREATE_CONNECTION: &str = "/files/connect";

    /// Method - POST
    pub const SAVE_FILE: &str = "/files/{uuid}/save";

    /// Method - GET
    pub const GET_FILE: &str = "/files/{uuid}/get";

    /// Method - GET
    pub const GET_SUM: &str = "/files/{uuid}/sum";

    /// Method - GET
    pub const GET_FILES_LIST: &str = "/files/list";

    /// Method - GET
    pub const GET_AVAILABLE_SPACE: &str = "/files/space";
}