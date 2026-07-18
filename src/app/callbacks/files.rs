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
                    let files = service.write().await.get_files().await; 
                    match files {
                        Ok(res) => UiActions::DataUpdateFilesList(res, String::from("/")),
                        Err(err_act) => err_act
                    }.run_in_event_loop(win);
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
                        Ok(files) => UiActions::DataUpdateFilesList(files, lock.current_dir()),
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
                            let (files, from) = {
                                let lock = service.read().await;
                                (lock.cached_files(), lock.current_dir())
                            };
                            UiActions::DataUpdateFilesList(files, from)
                        },
                        Err(err) => err
                    }.run_in_event_loop(win);
                });
            }
        });

        self.ui_window.on_files_remove_directory({
            let win = self.ui_window.as_weak();
            let service = files_service.clone();

            move |dir_name| {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let resp = service.write().await.remove_dir(dir_name.as_str()).await;
                    match resp {
                        Ok(_) => {
                            let (files, from) = {
                                let lock = service.read().await;
                                (lock.cached_files(), lock.current_dir())
                            };
                            UiActions::DataUpdateFilesList(files, from)
                        },
                        Err(err) => err
                    }.run_in_event_loop(win);
                });

            }
        });

        self.ui_window.on_files_upload_files( {
            let win = self.ui_window.as_weak();
            let service = files_service.clone();

            move || {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let files = rfd::AsyncFileDialog::new()
                        .set_directory("/")
                        .pick_files()
                        .await;
                    
                    if files.is_none() {
                        return;
                    }

                    let files = files.unwrap();
                    let mut uuids = Vec::<String>::with_capacity(files.len()); // 10 - reserve

                    for f in files {
                        match service.write().await.upload_file(f.path()).await {
                            Ok(id) => uuids.push(id.to_string()),
                            Err(act) => act.run_in_event_loop(win.clone()), 
                        }
                    }
                    
                });
            }
        });
    }
}