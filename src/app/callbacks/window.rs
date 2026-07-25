use {
    crate::{MainInternal, app::Application, repository::dirs::default_download_dir},
    std::process::Command,
    slint::ComponentHandle
};

impl Application {
    pub fn init_window_callbacks(&self) {
        let internal = self.ui_window.global::<MainInternal>();

        internal.on_open_downloads({
            let cfg = self.cfg.clone();
            move || {
                let cfg = cfg.clone();
                tokio::spawn(async move {
                    #[cfg(target_os = "windows")]
                    Command::new("open").arg(cfg.read().await.download_dir().unwrap_or(default_download_dir()).spawn().unwrap());

                    #[cfg(target_os = "linux")]
                    Command::new("xdg-open").arg(cfg.read().await.download_dir().unwrap_or(default_download_dir())).spawn().unwrap();
                });
            }
        });

        internal.on_quit({
            let win = self.ui_window.as_weak();
            move || {
                let _ = win.upgrade_in_event_loop(move |win| {
                    win.window().dispatch_event(slint::platform::WindowEvent::CloseRequested);
                });
            }
        });
    }
}