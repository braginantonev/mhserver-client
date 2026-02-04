mod notification;
mod config;
mod http;

use {
    api::authorization::User, 
    config::Config, 
    std::{
        error::Error, 
        sync::Arc,
    }, 
    tokio::sync::Mutex,
};

slint::include_modules!();

enum UiActions {
    ChangeAppState(AppStates),
    ChangePreparingState(PreparingStates),
    ShowNotification(String, NotificationType)
}

impl UiActions {
    fn run(self, win: MainWindow) {
        match self {
            UiActions::ChangeAppState(next) => win.set_state(next),
            UiActions::ChangePreparingState(next) => win.set_prepare_state(next),
            UiActions::ShowNotification(desc, r#type) => {
                notification::show(win, desc.as_str(), r#type);
            }
        }
    }
}

struct ConnectionInfo {
    client: reqwest::Client,
    server_address: String,
}

impl ConnectionInfo {
    fn new(client: reqwest::Client, srv_addr: &str) -> Self { 
        Self { 
            client: client.clone(),
            server_address: srv_addr.to_string(), 
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

    let cfg = Arc::new(Mutex::new(match Config::from_file() {
        Ok(cfg) => cfg,
        Err(_) => Config::default()
    }));

    let (preparing_tx, preparing_rx) = tokio::sync::broadcast::channel::<String>(2);
    
    // Setup server connection and jwt
    main_window.on_start_preparing({
        let win = win_weak.clone();
        let cfg = cfg.clone();
        let client = http_client.clone();
        let rx = preparing_rx.resubscribe();

        move || {
            let win = win.clone();
            let cfg = cfg.clone();
            let client = client.clone(); 
            let mut rx = rx.resubscribe();

            tokio::spawn(async move {
                let (server_address, jwt) = {
                    let guard = cfg.lock().await;
                    (guard.get_server_address(), guard.get_user_jwt()) 
                };

                println!("ping server with address = {}", server_address);

                // Ping server
                match api::ping::ping(client.clone(), server_address.as_str()).await {
                    Err(_) => {
                        let _ = win.upgrade_in_event_loop(move |win| {
                            win.set_prepare_state(PreparingStates::Connection);
                        });

                        // Wait jwt from login callback
                        let server_address = rx.recv().await.expect("failed receive address from chanel");
                        cfg.lock().await.set_address(server_address.as_str());
                    },
                    _ => {}
                };

                if jwt == "" {
                    let _ = win.upgrade_in_event_loop(|win| {
                        win.set_prepare_state(PreparingStates::Login);
                    });
                }

                // Wait jwt from login callback
                let jwt = rx.recv().await.expect("failed receive jwt from channel");
                {
                    let mut guard = cfg.lock().await;
                    guard.set_user_jwt(jwt.as_str());
                    guard.save_to_file();
                }

                let _ = win.upgrade_in_event_loop(|_win| {
                    println!("end of preparing");
                    //win.set_state(AppStates::Main);
                });
            });
        }
    });

    // Used to send new server address to prepare callback
    main_window.on_connect({
        let win = win_weak.clone();
        let client = http_client.clone();
        let tx = preparing_tx.clone(); // Sender for preparing callback

        move |server_addr| {
            let win = win.clone();
            let client = client.clone();
            let tx = tx.clone();

            tokio::spawn(async move {
                if let Some(act) = match api::ping::ping(client, server_addr.as_str()).await {
                    Ok(_) => {
                        match tx.send(server_addr.to_string()) {
                            Err(err) => Some(UiActions::ShowNotification(err.0, NotificationType::Error)),
                            _ => None
                        }
                    },
                    Err(err) => Some(UiActions::ShowNotification(err.to_string(), NotificationType::Error))
                } {
                    let _ = win.upgrade_in_event_loop(|win| {
                        act.run(win);
                    });
                };
            });
        }
    });
    
    // Used to send new jwt to preparing callback
    main_window.on_login({
        let win = win_weak.clone();
        let cfg = cfg.clone();
        let client = http_client.clone();
        let tx = preparing_tx.clone();

        move |username, password| {
            let win = win.clone();
            let cfg = cfg.clone();
            let http_client = client.clone();
            let tx = tx.clone();

            tokio::spawn(async move {
                let server_address = cfg.lock().await.get_server_address();

                if let Some(jwt) = http::authorization::login(
                    win,
                    ConnectionInfo::new(http_client, server_address.as_str()), 
                    User::new(username.as_str(), password.as_str())
                ).await {
                    tx.send(jwt).expect("failed to send jwt to channel");
                }
            });
        }
    });
    
    main_window.on_register({
        let win = win_weak.clone();
        let cfg = cfg.clone();
        let client = http_client.clone();

        move |username, password, verify| {
            let win = win.clone();
            let cfg = cfg.clone();
            let http_client = client.clone();

            if password != verify {
                let _ = win.upgrade_in_event_loop(move |win| {
                    notification::show(win, "Password and verify password not ident!", NotificationType::Error);
                });
                return
            }

            tokio::spawn(async move {
                let server_address = cfg.lock().await.get_server_address();

                http::authorization::register(
                    win,
                    ConnectionInfo::new(http_client, server_address.as_str()),
                    User::new(username.as_str(), password.as_str())
                )
            });
        }
    });

    main_window.run()?;

    Ok(())
}