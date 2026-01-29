use {
    crate::{ 
        ConnectionInfo,
        PreparingStates, MainWindow, NotificationType, UiActions,
    },
    api::authorization::{ User, login_v1, register_v1 },
    slint::Weak, 
};

/// Return JWT token
pub async fn login(win: Weak<MainWindow>, conn: ConnectionInfo, user: User) -> Option<String> {
    let action = match login_v1(conn.client, conn.server_address.as_str(), user).await {
        Ok(jwt) => return Some(jwt),
        Err(err) => UiActions::ShowNotification(err.to_string(), NotificationType::Error)
    };

    let _ = win.upgrade_in_event_loop(|win| {
        action.run(win);
    });
    
    None
}

pub async fn register(win: Weak<MainWindow>, conn: ConnectionInfo, user: User) {
    let action = match register_v1(conn.client, conn.server_address.as_str(), user).await {
        Ok(_) => UiActions::ChangePreparingState(PreparingStates::Login),
        Err(err) => UiActions::ShowNotification(err.to_string(), NotificationType::Error)
    };

    let _ = win.upgrade_in_event_loop(|win| {
        action.run(win);
    });
}