mod connection;
mod notification;

use {
    std::sync::{Arc, Mutex}
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

            tokio::spawn(async move {
                let client = conn.lock().unwrap().get_client();
                
                match connection::connect(client, server_addr.as_str(), current_state).await {
                    Ok(next_state) => {
                        win.upgrade_in_event_loop(move |main_window| {
                            main_window.set_state(next_state);
                        }).unwrap()
                    },
                    Err(err) => {
                        win.upgrade_in_event_loop(move |main_window| {
                            notification::show(main_window, err.as_str(), NotificationType::Error);
                        }).unwrap()
                    }
                };
            });
        }
    );

    main_window.run()?;

    Ok(())
}