use {
    super::{ AppStates, MainWindow, NotificationType, notification },
    std::sync::{ Arc, Mutex },
    reqwest::Client, 
    slint::Weak,
};

pub struct ServerConnection {
    client: Client,
    addr: String,
}

impl ServerConnection {
    pub fn new(client: Client, addr: &str) -> Self {
        Self { client: client, addr: addr.to_string() }
    }

    pub fn set_address(&mut self, new_addr: &str) {
        self.addr = new_addr.to_string()
    }

    pub fn get_address(&self) -> String {
        self.addr.clone()
    }

    // Return cloned client
    pub fn get_client(&self) -> Client {
        self.client.clone()
    }
}

pub async fn connect(win: Weak<MainWindow>, conn: Arc<Mutex<ServerConnection>>, current_state: AppStates) {
    let client = conn.lock().unwrap().get_client();
    let server_addr = conn.lock().unwrap().get_address();

    let resp = api::ping::ping(client, server_addr.as_str()).await;

    let _ = win.upgrade_in_event_loop(move |win| {
        match resp {
            Ok(res) => {
                if res { win.set_state(current_state.next()); }
            },
            Err(err) => { notification::show(win, err.to_str(), NotificationType::Error); }
        }
    });
}