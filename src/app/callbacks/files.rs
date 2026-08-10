use {
    crate::{
        FilesInternal, actions::{AnyActions, FilesActions, MainActions, UiActions}, app::Application, service::files::{dirs::Directory, FileManager},
    }, slint::ComponentHandle, std::{str::FromStr, sync::Arc}, tokio::{sync::RwLock}
};

impl Application {
    pub fn init_files_callbacks(&self, files_service: Arc<RwLock<FileManager>>) {
        let internal = self.ui_window.global::<FilesInternal>();
        let win = self.ui_window.as_weak();
        
        internal.on_update_list({
            let win = win.clone();
            let service = files_service.clone();

            move || {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let files = service.write().await.get_files(None).await; 

                    match files {
                        Ok(res) => FilesActions::UpdateFilesList(res, String::from("/")).run_in_event_loop(win.clone()),
                        Err(err) => return AnyActions::from(err).run_in_event_loop(win), // actions because, user can be unauthorized 
                    };

                    match service.read().await.available_space().await {
                        Ok(s) => FilesActions::UpdateAvailableSpace(s.to_string_candy()).run_in_event_loop(win),
                        Err(err) => MainActions::from(err).run_in_event_loop(win),
                    };
                });
            }
        });

        internal.on_cd({
            let win = win.clone();
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
                        Ok(files) => FilesActions::UpdateFilesList(files, lock.current_dir().into()).run_in_event_loop(win.clone()),
                        Err(err) => MainActions::from(err).run_in_event_loop(win.clone()),
                    };
                    FilesActions::UpdateCurrentDirectory(lock.current_dir().into()).run_in_event_loop(win);
                });
            }
        });

        internal.on_mkdir({
            let win = win.clone();
            let service = files_service.clone();

            move |dir_name| {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let resp = service.write().await.make_dir(None, dir_name.as_str()).await;
                    match resp {
                        Ok(_) => {
                            let (files, from) = {
                                let lock = service.read().await;
                                (lock.cached_files(), lock.current_dir())
                            };
                            FilesActions::UpdateFilesList(files, from.into()).run_in_event_loop(win);
                        },
                        Err(err) => MainActions::from(err).run_in_event_loop(win),
                    };
                });
            }
        });

        internal.on_rmdir({
            let win = win.clone();
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
                            FilesActions::UpdateFilesList(files, from.into()).run_in_event_loop(win);
                        },
                        Err(err) => MainActions::from(err).run_in_event_loop(win),
                    };
                });

            }
        });

        internal.on_upload_files({
            let win = win.clone();
            let service = files_service.clone();

            move || {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let current_dir = service.read().await.current_dir();

                    let files = rfd::AsyncFileDialog::new()
                        .set_directory("~")
                        .pick_files()
                        .await;
                    
                    let files = match files {
                        Some(v) => v,
                        None => return,
                    };

                    let mut handles = Vec::with_capacity(files.len());
                    for f in files {
                        // todo: use single lock in api 3.x if it's will be needed
                        handles.push(service.write().await.upload_file(Some(current_dir.clone()), f.path().to_path_buf()));
                    }

                    for h in handles {
                        if let Err(err) = h.await.unwrap() {
                            MainActions::from(err).run_in_event_loop(win.clone());
                        }
                    } 
                });
            }
        });

        internal.on_upload_folders({
            let win = win.clone();
            let service = files_service.clone();

            move || {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let current_dir = service.read().await.current_dir();

                    let dirs = rfd::AsyncFileDialog::new()
                        .set_directory("~")
                        .pick_folders()
                        .await;
                    
                    let dirs = match dirs {
                        Some(v) => v,
                        None => return,
                    };

                    let mut handles = Vec::new();
                    for dir in dirs {
                        let mut files = Directory::from_recursive(dir.path()).read();
                        while let Some(files) = files.recv().await {
                            if let Err(err) = service.write().await.make_dir(Some(current_dir.clone()), files.0.to_string().trim_start_matches('/').trim_end_matches('/')).await {
                                MainActions::from(err).run_in_event_loop(win.clone());
                                continue
                            };

                            for f in files.1 {
                                handles.push(service.write().await.upload_file(Some(files.0.clone()), f));
                            }
                        }
                    }

                    for h in handles {
                        if let Err(err) = h.await.unwrap() {
                            MainActions::from(err).run_in_event_loop(win.clone());
                            continue;
                        }
                    }
                });
            }
        });

        internal.on_download_file({
            let win = win.clone();
            let service = files_service.clone();

            move |filename| {
                let win = win.clone();
                let service = service.clone();
                
                tokio::spawn(async move {
                    if let Err(err) = service.write().await.download_file(None, filename.to_string()).await.unwrap() {
                        MainActions::from(err).run_in_event_loop(win.clone());
                    };
                });
            }
        });

        internal.on_download_directory({
            let win = win.clone();
            let service = files_service.clone();

            move |dir_name| {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let mut from = service.read().await.current_dir().to_string() + &dir_name;
                    from.push('/');
                    
                    let download_files = match service.write().await.get_files(Some(from.clone())).await {
                        Ok(files) => files,
                        Err(err) => {
                            MainActions::from(err).run_in_event_loop(win);
                                return 
                        }
                    };

                    let mut handles = Vec::with_capacity(download_files.len());
                    for file in download_files {
                        // todo: use single lock in api 3.x if it's will be needed
                        handles.push(service.write().await.download_file(Some(from.clone()), file.name.clone()));
                    }
                    
                    for h in handles {
                        if let Err(err) = h.await.unwrap() {
                            MainActions::from(err).run_in_event_loop(win.clone());
                        }
                    }
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
            let win = win.clone();
            let service = files_service.clone();

            FilesActions::UpdateLoadFiles(vec![]).run_in_event_loop(win.clone()); // set default value

            move || {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let (load, complete) =  {
                        let lock = service.read().await;
                        (lock.get_load_files().await, lock.get_loaded_files().await)
                    };

                    FilesActions::UpdateLoadFiles(load).run_in_event_loop(win.clone());
                    
                    // Show downloaded files
                    complete.into_iter()
                        .for_each(|load| FilesActions::NotifyLoadedFile(load.0, load.1).run_in_event_loop(win.clone()));
                });
            }
        });
    }
}