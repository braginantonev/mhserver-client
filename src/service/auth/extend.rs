use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct Claims {
    exp: Option<u64>,
}

pub fn is_jwt_expired(token: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let payload_b64 = token.split('.').nth(1).ok_or("invalid jwt: no payload")?;

    let payload_bytes = URL_SAFE_NO_PAD.decode(payload_b64)?;
    let claims: Claims = serde_json::from_slice(&payload_bytes)?;

    match claims.exp {
        Some(exp) => {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            Ok(now >= exp)
        }
        None => Ok(false),
    }
}
