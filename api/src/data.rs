//* ------------------------------- *//
//*                                 *//
//*   Mhserver API version: 1.2.x   *//
//*                                 *//
//* ------------------------------- *//

use {
    super::{AuthorizedRequest, ServerError, endpoints::{self, data}},
    chrono::{DateTime, TimeZone},
    chrono_tz::Tz, 
    reqwest::StatusCode,
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    name: String,

    #[serde(default)]
    size: u64,

    #[serde(rename = "isDir", default)]
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
pub async fn get_files_v1(req: AuthorizedRequest, target_dir: &str) -> Result<Vec<FileInfo>, ServerError> {
    match req.client.get(endpoints::build_url(req.srv_addr_str(), endpoints::API_V1,data::GET_FILES, Some(&[("dir", target_dir)])).unwrap())
    .bearer_auth(req.jwt)
    .send().await {
        Ok(resp) => {
            if resp.status() != 200 {
                let st = resp.status();
                Err(ServerError::new(resp.text().await.unwrap().as_str()).with_status(st)) 
            }
            else { 
                //println!("{}", resp.text().await.unwrap());
                match resp.json::<Vec<FileInfo>>().await {
                    Ok(res) => Ok(res),
                    Err(_) => Err(ServerError::new("server have another json structure").with_status(StatusCode::BAD_REQUEST))
                }
            }
        },
        Err(err) => Err(ServerError::from(err))
    }
}