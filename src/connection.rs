use {
    super::{ AppStates, MainWindow, NotificationType, UiActions, notification },
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
    let (client, server_addr) = {
        let guard = conn.lock().unwrap();
        (guard.get_client(), guard.get_address())
    };

    let action = match api::ping::ping(client, server_addr.as_str()).await {
        Ok(_) => UiActions::ChangeState(current_state.next()),
        Err(err) => UiActions::ShowNotification(err.to_string(), NotificationType::Error)
    };

    let _ = win.upgrade_in_event_loop(move |win| {
        match action {
            UiActions::ChangeState(next) => win.set_state(next),
            UiActions::ShowNotification(err, r#type) => { notification::show(win, err.as_str(), r#type); }
        }
    });
}