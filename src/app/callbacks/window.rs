use {
    crate::{MainInternal, NotificationInfo, NotificationsInternal, app::Application, repository::dirs::default_download_dir}, slint::{ComponentHandle, Global, Model, ModelRc, VecModel}, std::{process::Command, rc::Rc}
};

impl Application {
    pub fn init_window_callbacks(&self) {
        let win = self.ui_window.as_weak();
        let main_internal = self.ui_window.global::<MainInternal>();

        main_internal.on_open_downloads({
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

        main_internal.on_quit({
            let win = win.clone();
            move || {
                let _ = win.upgrade_in_event_loop(move |win| {
                    win.window().dispatch_event(slint::platform::WindowEvent::CloseRequested);
                });
            }
        });

        let notifications_internal = self.ui_window.global::<NotificationsInternal>();
        let weak_notifications_internal = notifications_internal.as_weak();
        
        // set empty model for notifications
        notifications_internal.set_active_notifications(ModelRc::from(Rc::new(VecModel::<NotificationInfo>::default())));
        
        notifications_internal.on_push({
            let internal = weak_notifications_internal.clone();
            move |info| {
                let notifications = internal.upgrade().unwrap().get_active_notifications();
                let notifications = notifications.as_any().downcast_ref::<VecModel<NotificationInfo>>().unwrap();
                notifications.push(info);
            }
        });

        notifications_internal.on_pop({
            let internal = weak_notifications_internal.clone();
            move || {
                let notifications = internal.upgrade().unwrap().get_active_notifications();
                let notifications = notifications.as_any().downcast_ref::<VecModel<NotificationInfo>>().unwrap();
                notifications.remove(0);
            }
        });
    }
}