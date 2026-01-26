use {
    crate::{ ConnectionInfo, AppStates, MainWindow, NotificationType, UiActions, notification },
    slint::Weak,
};

pub async fn ping(win: Weak<MainWindow>, conn: ConnectionInfo, current_state: AppStates) {
    let action = match api::ping::ping(conn.client, conn.server_address.as_str()).await {
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