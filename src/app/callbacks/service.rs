use {
    crate::{Services, actions::UiActions, app::Application, service::*}, 
    slint::ComponentHandle,
    std::sync::Arc,
    tokio::sync::RwLock
};

impl Application {
    pub fn init_service_callbacks(
        &self, 
        s_files: Arc<RwLock<files::FileManager>>
    ) {
        let win_weak = self.ui_window.as_weak();
        
        self.ui_window.on_show_service({
            let win = win_weak.clone();
            let file_service = s_files.clone();

            move |service| {
                let win = win.clone();
                let file_service = file_service.clone();

                UiActions::ChangeActiveService(service).run_in_event_loop(win.clone());

                tokio::spawn(async move {
                    match service {
                        Services::Files => {
                            let files = match file_service.read().await.get_files().await {
                                Ok(res) => res,
                                Err(act) => {
                                    act.run_in_event_loop(win);
                                    return
                                }
                            };

                            UiActions::DataUpdateFilesList(files).run_in_event_loop(win);
                        },
                        _ => todo!()
                    }
                });
            }
        });
    }
}