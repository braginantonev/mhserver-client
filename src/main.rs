slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let http_client = reqwest::Client::builder()
        .tls_info(true)
        .tls_backend_rustls()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::new(2, 0))
        .build()
        .expect("failed build http client");

    let main_window = MainWindow::new()?;
    //let win_weak = main_window.as_weak();

    main_window.on_connect(move |server_addr| {
        let client = http_client.clone();

        tokio::spawn(async move {
            println!("Подключение к {}", server_addr.as_str());

            match api::ping::ping(client, server_addr.as_str()).await {
                Ok(res) => {
                    if res { println!("Подключение успешно") }
                    else { println!("Сервер отключен или неверный ip") }
                },
                Err(err) => {
                    println!("Ошибка подключения: {}", err.to_string())
                }
                
            }
        });
    });

    main_window.run()
}