use {
    super::{ AppStates, MainWindow, NotificationType, notification, connection::ServerConnection },
    std::sync::{ Arc, Mutex },
    slint::Weak,
    api::authorization::{ login_v1, register_v1, User },
};

/// Return JWT token
pub async fn login(win: Weak<MainWindow>, conn: Arc<Mutex<ServerConnection>>, user: User) -> Option<String> {
    let client = conn.lock().unwrap().get_client();
    let server_addr = conn.lock().unwrap().get_address();

    match login_v1(client, server_addr.as_str(), user).await {
        Ok(jwt) => Some(jwt),
        Err(err) => {
            let _ = win.upgrade_in_event_loop(move |win| {
                notification::show(win, err.to_str(), NotificationType::Error);
            });
            
            None
        }
    }
}

pub async fn register(win: Weak<MainWindow>, conn: Arc<Mutex<ServerConnection>>, user: User) {
    let client = conn.lock().unwrap().get_client();
    let server_addr = conn.lock().unwrap().get_address();

    match register_v1(client, server_addr.as_str(), user).await {
        Ok(_) => {},
        Err(err) => {
            let _ = win.upgrade_in_event_loop(move |win| {
                notification::show(win, err.to_str(), NotificationType::Error);
            });
        }
    };
}