use {
    crate::{
        Services, 
        actions::UiActions, 
        app::Application, 
        service
    }, 
    slint::ComponentHandle, 
    std::sync::Arc, 
    tokio::sync::RwLock
};

impl Application {
    pub fn init_files_callbacks(&mut self, files_service: Arc<RwLock<service::files::FileManager>>) {
        self.ui_window.on_show_service({
            let win = self.ui_window.as_weak();
            let service = files_service.clone();

            move |target| {
                if target != Services::Files {
                    return
                }

                let win = win.clone();
                let service = service.clone();

                UiActions::ChangeActiveService(target).run_in_event_loop(win.clone());

                tokio::spawn(async move {
                    let files = match service.read().await.get_files().await {
                        Ok(res) => res,
                        Err(act) => {
                            act.run_in_event_loop(win);
                            return
                        }
                    };

                    println!("files: {:?}", files);

                    UiActions::DataUpdateFilesList(files, String::from("/")).run_in_event_loop(win);
                });
            }
        });
    }
}