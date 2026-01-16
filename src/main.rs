mod connect;

use {
    reqwest::Client, slint::ToSharedString, std::fmt,
    std::sync::{Arc, Mutex},
};

slint::include_modules!();

#[derive(Default, Clone)]
struct ServerConnection {
    client: Option<reqwest::Client>,
    addr: String,
}

impl ServerConnection {
    fn new(client: Client, addr: &str) -> Self {
        Self { client: Some(client), addr: addr.to_string() }
    }

    fn set_from(&mut self, conn: ServerConnection) {
        self.client = conn.client;
        self.addr = conn.addr;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ApplicationState {
    Connection,
    Authorization
}

impl ApplicationState {
    pub fn next(&self) -> Self {
        match self {
            ApplicationState::Connection => ApplicationState::Authorization,
            ApplicationState::Authorization => todo!()
        }
    }
}

impl fmt::Display for ApplicationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::builder()
        .tls_info(true)
        .tls_backend_rustls()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::new(2, 0))
        .build()
        .expect("failed build http client");

    let server_conn = Arc::new(Mutex::new(ServerConnection::default()));

    // Set first state
    let mut app_state = ApplicationState::Connection;

    let main_window = MainWindow::new().unwrap();
    let win_weak = main_window.as_weak();

    // Open first scene by state
    win_weak.upgrade().unwrap().set_scene(app_state.to_shared_string());

    main_window.on_connect(move |server_addr| {
        let win = win_weak.clone();
        let conn = Arc::clone(&server_conn);

        let req_conn = ServerConnection::new(http_client.clone(), server_addr.as_str());

        tokio::spawn(async move {
            match connect::connect(req_conn.clone(), app_state).await {
                Ok(next_state) => {
                    app_state = next_state;
                    conn.lock().unwrap().set_from(req_conn);

                    win.upgrade_in_event_loop(move |main_window| {
                        main_window.set_scene(app_state.to_shared_string());
                    }).unwrap()
                },
                Err(err) => println!("{err}")
            };
        });
    });

    main_window.run().unwrap();
}