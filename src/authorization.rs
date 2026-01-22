use {
    super::{ 
        connection::ServerConnection,
        AppStates, MainWindow, NotificationType, UiActions,
        notification, 
    },
    api::authorization::{ User, login_v1, register_v1 },
    slint::Weak, 
    std::sync::{ Arc, Mutex }
};

/// Return JWT token
pub async fn login(win: Weak<MainWindow>, conn: Arc<Mutex<ServerConnection>>, user: User) -> Option<String> {
    let (client, server_addr) = {
        let guard = conn.lock().unwrap();
        (guard.get_client(), guard.get_address())
    };

    let action = match login_v1(client, server_addr.as_str(), user).await {
        Ok(jwt) => return Some(jwt),
        Err(err) => UiActions::ShowNotification(err.to_string(), NotificationType::Error)
    };

    let _ = win.upgrade_in_event_loop(move |win| {
        match action {
            UiActions::ShowNotification(err, r#type) => {
                notification::show(win, err.as_str(), r#type);
            }
            _ => (),
        };   
    });
    
    None
}

pub async fn register(win: Weak<MainWindow>, conn: Arc<Mutex<ServerConnection>>, user: User) {
    let (client, server_addr) = {
        let guard = conn.lock().unwrap();
        (guard.get_client(), guard.get_address())
    };

    let action = match register_v1(client, server_addr.as_str(), user).await {
        Ok(_) => UiActions::ChangeState(AppStates::Login),
        Err(err) => UiActions::ShowNotification(err.to_string(), NotificationType::Error)
    };

    let _ = win.upgrade_in_event_loop(move |win| {
        match action {
            UiActions::ChangeState(next) => { win.set_state(next); }
            UiActions::ShowNotification(err, r#type) => { 
                notification::show(win, err.as_str(), r#type);
            }
        };
    });
}