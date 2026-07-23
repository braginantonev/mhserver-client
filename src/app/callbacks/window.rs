use slint::ComponentHandle;
use std::process::Command;
use crate::app::Application;

use crate::repository::dirs::default_download_dir;

impl Application {
    pub fn init_window_callbacks(&self) {
        let win_weak = self.ui_window.as_weak();

        self.ui_window.on_open_downloads({
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

        self.ui_window.on_quit({
            let win = win_weak.clone();
            move || {
                let _ = win.upgrade_in_event_loop(move |win| {
                    win.window().dispatch_event(slint::platform::WindowEvent::CloseRequested);
                });
            }
        });
    }
}