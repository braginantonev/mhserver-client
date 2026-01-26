mod notification;
mod config;
mod http;

use {
    api::authorization::User, 
    std::{
        error::Error, 
        sync::Arc,
    },
    config::Config,
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

enum UiActions {
    ChangeState(AppStates),
    ShowNotification(String, NotificationType)
}

struct ConnectionInfo {
    client: reqwest::Client,
    server_address: String,
    user_jwt: Option<String>,
}

impl ConnectionInfo {
    fn new(client: reqwest::Client, srv_addr: &str, user_jwt: &str) -> Self { 
        Self { 
            client: client.clone(),
            server_address: srv_addr.to_string(), 
            user_jwt: if user_jwt != "" { Some(user_jwt.to_string()) } else { None },
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let main_window = MainWindow::new()?;
    let win_weak = main_window.as_weak();

    let http_client = reqwest::Client::builder()
        .tls_info(true)
        .tls_backend_rustls()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::new(2, 0))
        .build()?;

    let cfg = Arc::new(match Config::from_file() {
        Ok(cfg) => cfg,
        Err(_) => {
            main_window.set_state(AppStates::Connection); // Set start field
            Config::default()
        }
    });

    let cb_win = win_weak.clone();
    let cb_cfg = cfg.clone();
    let cb_client = http_client.clone();
    main_window.on_connect(move |server_addr| {
        let win = cb_win.clone();
        let http_client = cb_client.clone();
        let current_state = win.upgrade().unwrap().get_state();

        tokio::spawn(http::tools::ping(
            win,
            ConnectionInfo::new(http_client, cb_cfg.get_server_address().as_str(), cb_cfg.get_user_jwt().as_str()),
            current_state
        ));
    });

    let call_win = win_weak.clone();
    let call_cfg = cfg.clone();
    let cb_client = http_client.clone();
    main_window.on_login(move |username, password| {
        let win = call_win.clone();
        let http_client = cb_client.clone();

        tokio::spawn(http::authorization::login(
            win,
            ConnectionInfo::new(http_client, call_cfg.get_server_address().as_str(), call_cfg.get_user_jwt().as_str()), 
            User::new(username.as_str(), password.as_str())
        ));
    });

    let call_win = win_weak.clone();
    let call_cfg = cfg.clone();
    let cb_client = http_client.clone();
    main_window.on_register(move |username, password, verify| {
        let win = call_win.clone();
        let http_client = cb_client.clone();

        if password != verify {
            let _ = win.upgrade_in_event_loop(move |win| {
                notification::show(win, "Password and verify password not ident!", NotificationType::Error);
            });
            return
        }

        tokio::spawn(http::authorization::register(
            win,
            ConnectionInfo::new(http_client, call_cfg.get_server_address().as_str(), call_cfg.get_user_jwt().as_str()),
            User::new(username.as_str(), password.as_str())
        ));
    });

    main_window.run()?;

    Ok(())
}