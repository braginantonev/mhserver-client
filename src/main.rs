mod connect;

use {
    reqwest::Client, slint::ToSharedString, std::fmt,
    std::sync::{Arc, Mutex},
};

slint::include_modules!();

#[derive(Default, Clone)]
struct ServerConnection {
    client: Arc<reqwest::Client>,
    addr: String,
}

impl ServerConnection {
    fn new(client: Client, addr: &str) -> Self {
        Self { client: Arc::new(client), addr: addr.to_string() }
    }

    fn clone(&self) -> Self {
        Self { client: Arc::clone(&self.client), addr: self.addr.clone() }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum States {
    Connection,
    Authorization
}

impl States {
    pub fn next(&self) -> Self {
        match self {
            States::Connection => States::Authorization,
            States::Authorization => todo!()
        }
    }
}

impl fmt::Display for States {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct AppState(States);

impl AppState {
    fn set_state(&mut self, new_state: States) {
        self.0 = new_state;
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

    let server_conn = ServerConnection::new(http_client, "");

    // Set first state
    let app_state = Arc::new(Mutex::new(AppState(States::Connection)));

    let main_window = MainWindow::new()?;
    let win_weak = main_window.as_weak();

    // Open first scene by state
    win_weak.upgrade().unwrap().set_scene(app_state.lock().unwrap().0.to_shared_string());

    main_window.on_connect(move |server_addr| {
        let win = win_weak.clone();

        let state = Arc::clone(&app_state);
        let mut conn = server_conn.clone();
        conn.addr = server_addr.to_string();

        tokio::spawn(async move {
            let current_state = state.lock().unwrap().0;
            match connect::connect(conn, current_state).await {
                Ok(next_state) => {
                    state.lock().unwrap().set_state(next_state);

                    win.upgrade_in_event_loop(move |main_window| {
                        main_window.set_scene(state.lock().unwrap().0.to_shared_string());
                    }).unwrap()
                },
                Err(err) => println!("{err}")
            };
        });
    });

    main_window.run()?;

    Ok(())
}