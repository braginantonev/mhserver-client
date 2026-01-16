use {
    super::{ States, ServerConnection },
};

pub async fn connect(conn: ServerConnection, state: States) -> Result<States, String> {
    match api::ping::ping(conn.client.as_ref(), conn.addr.as_str()).await {
        Ok(res) => {
            if res { Ok(state.next()) }
            else { Err("Wrong address or server is off".to_string()) }
        },
        Err(err) => { Err(format!("Connection error: {}", err.to_string())) }
    }
}