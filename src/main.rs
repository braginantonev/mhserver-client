mod connection;
mod notification;
mod authorization;

use {
    std::sync::{Arc, Mutex},
    api::authorization::User,
};

slint::include_modules!();

impl AppStates {
    fn next(&self) -> AppStates {
        match self {
            AppStates::Connection => AppStates::Login,
            AppStates::Login => todo!(),
            AppStates::Register => todo!()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError>{
    let http_client = reqwest::Client::builder()
        .tls_info(true)
        .tls_backend_rustls()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::new(2, 0))
        .build()
        .expect("failed build http client");

    let server_conn = Arc::new(Mutex::new(connection::ServerConnection::new(http_client, "")));

    let main_window = MainWindow::new()?;
    let win_weak = main_window.as_weak();

    let call_win = win_weak.clone();
    let call_conn = server_conn.clone();
    main_window.on_connect(move |server_addr| {
        let win = call_win.clone();
        let conn = call_conn.clone();
        conn.lock().unwrap().set_address(server_addr.as_str());

        let current_state = win.upgrade().unwrap().get_state();

        tokio::spawn(connection::connect(win, conn, current_state));
    });

    let call_win = win_weak.clone();
    let call_conn = server_conn.clone();
    main_window.on_login(move |username, password| {
        let win = call_win.clone();
        let conn = call_conn.clone();

        tokio::spawn(authorization::login(win, conn, User::new(username.as_str(), password.as_str())));
    });

    let call_win = win_weak.clone();
    let call_conn = server_conn.clone();
    main_window.on_register(move |username, password, verify| {
        let win = call_win.clone();
        let conn = call_conn.clone();

        if password != verify {
            let _ = win.upgrade_in_event_loop(move |win| {
                notification::show(win, "Password and verify password not ident!", NotificationType::Error);
            });
            return
        }

        tokio::spawn(authorization::register(win, conn, User::new(username.as_str(), password.as_str())));
    });

    main_window.run()?;

    Ok(())
}