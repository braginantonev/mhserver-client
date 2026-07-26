use {
    crate::{
        FilesInternal, actions::UiActions, app::Application, service
    }, slint::ComponentHandle, std::{str::FromStr, sync::Arc}, tokio::sync::RwLock
};

impl Application {
    pub fn init_files_callbacks(&self, files_service: Arc<RwLock<service::files::FileManager>>) {
        let internal = self.ui_window.global::<FilesInternal>();

        internal.on_update_list({
            let win = self.ui_window.as_weak();
            let service = files_service.clone();

            move || {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let files = service.write().await.get_files(None).await; 
                    match files {
                        Ok(res) => UiActions::FilesUpdateFilesList(res, String::from("/")),
                        Err(err_act) => err_act
                    }.run_in_event_loop(win);
                });
            }
        });

        internal.on_cd({
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
                        Ok(files) => UiActions::FilesUpdateFilesList(files, lock.current_dir()),
                        Err(act) => act
                    }.run_in_event_loop(win.clone());
                    UiActions::FilesUpdateCurrentDirectory(lock.current_dir()).run_in_event_loop(win);
                });
            }
        });

        internal.on_mkdir({
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
                            UiActions::FilesUpdateFilesList(files, from)
                        },
                        Err(err) => err
                    }.run_in_event_loop(win);
                });
            }
        });

        internal.on_rmdir({
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
                            UiActions::FilesUpdateFilesList(files, from)
                        },
                        Err(err) => err
                    }.run_in_event_loop(win);
                });

            }
        });

        internal.on_upload_files({
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

                    {
                        let mut lock = service.write().await;
                        for f in files.unwrap() {
                            if let Err(act) = lock.upload_file(f.path()).await {
                                act.run_in_event_loop(win.clone());
                            }
                        }
                    }

                    UiActions::FilesUpdateLoadFiles(service.read().await.get_load_files().await).run_in_event_loop(win);
                });
            }
        });

        internal.on_download_file({
            let win = self.ui_window.as_weak();
            let service = files_service.clone();

            move |filename| {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    if let Err(act) = service.write().await.download_file(None, filename.to_string()).await {
                        act.run_in_event_loop(win.clone());
                    };
                    UiActions::FilesUpdateLoadFiles(service.read().await.get_load_files().await).run_in_event_loop(win);
                });
            }
        });

        internal.on_cancel_load({
            let service = files_service.clone();

            move |uuid| {
                let service = service.clone();
                tokio::spawn(async move {
                    service.write().await.cancel_load(uuid::Uuid::from_str(uuid.as_str()).unwrap()).await;
                });
            }
        });

        internal.on_update_load_files({
            let win = self.ui_window.as_weak();
            let service = files_service.clone();

            move || {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    UiActions::FilesUpdateLoadFiles(service.read().await.get_load_files().await).run_in_event_loop(win);
                });
            }
        });
    }
}