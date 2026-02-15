// mhserver api version: 1.2.0 (dev)

pub const API_V1: &str = "/api/v1";

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
    pub const GET_FILES: &str = "/files";

    /// Method - GET
    pub const GET_AVAILABLE_SPACE: &str = "/files/space";

    /// Method - POST
    pub const CREATE_DIR: &str = "/files/mkdir";

    /// Method - POST
    pub const REMOVE_DIR: &str = "/files/rmdir";
}

/// Build a standard URL, with https.
/// 
/// **Arguments**:
/// 1. Server address (e.g. localhost:8080)
/// 2. API URL with version (e.g. /api/v1)
/// 3. Endpoint (e.g. /ping)
pub fn build_url(addr: &str, api: &str, endpoint: &str) -> String {
    format!("https://{}{}{}", addr, api, endpoint)
}