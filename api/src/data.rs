//* ------------------------------- *//
//*                                 *//
//*   Mhserver API version: 1.2.x   *//
//*                                 *//
//* ------------------------------- *//

use {
    super::{ServerError, endpoints::{self, data}},
    chrono::{DateTime, TimeZone},
    chrono_tz::Tz, 
    reqwest::{Client, StatusCode},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    name: String,
    size: u64,

    #[serde(rename = "isDir")]
    is_dir: bool,

    #[serde(rename = "modTime")]
    mod_time: i64
}

impl FileInfo {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    /// Return chrono::DateTime in your timezone
    pub fn mod_time(&self, tz: Tz) -> DateTime<Tz> {
        tz.timestamp_opt(self.mod_time, 0).unwrap()
    }

    pub fn mod_time_unix(&self) -> i64 {
        self.mod_time
    }
}

/// Return a vector of file infos from server (from internal dir)
pub async fn get_files_v1(http_client: Client, srv_addr: &str, target_dir: &str) -> Result<Vec<FileInfo>, ServerError> {
    match http_client.get(endpoints::build_url(srv_addr, endpoints::API_V1,data::GET_FILES, Some(&[("dir", target_dir)])).unwrap())
    .send().await {
        Ok(resp) => {
            if resp.status() != 200 {
                let st = resp.status();
                Err(ServerError::new(resp.text().await.unwrap().as_str(), st)) 
            }
            else { 
                match resp.json::<Vec<FileInfo>>().await {
                    Ok(res) => Ok(res),
                    Err(_) => Err(ServerError::new("server have another json structure", StatusCode::BAD_REQUEST))
                }
            }
        },
        Err(err) => Err(ServerError::from(err))
    }
}