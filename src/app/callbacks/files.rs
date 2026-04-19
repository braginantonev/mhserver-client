use {
    crate::{
        Services, 
        actions::UiActions, 
        app::Application, 
        service
    }, slint::ComponentHandle, std::sync::Arc, tokio::sync::RwLock
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
                    let files = match service.write().await.get_files().await {
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

        self.ui_window.on_files_change_directory({
            let win = self.ui_window.as_weak();
            let service = files_service.clone();

            move |target| {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let mut lock = service.write().await;

                    match if target != ".." {
                        lock.next(target.as_str()).await
                    } else {
                        lock.prev().await
                    } {
                        Ok(files) => UiActions::DataUpdateFilesList(files, lock.get_current_dir().to_owned()),
                        Err(act) => act
                    }.run_in_event_loop(win);
                });
            }
        });

        self.ui_window.on_files_make_directory({
            let win = self.ui_window.as_weak();
            let service = files_service.clone();

            move |dir_name| {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let resp = service.write().await.make_dir(dir_name.as_str()).await;
                    match resp {
                        Ok(_) => {
                            // Append new dir to files list instead a send request to server, to reduce the load on it.
                            let (files, from) = {
                                let lock = service.read().await;
                                (lock.get_cached_files(), lock.get_current_dir().to_owned())
                            };
                            UiActions::DataUpdateFilesList(files, from)
                        },
                        Err(err) => err
                    }.run_in_event_loop(win);
                });
            }
        });
    }
}