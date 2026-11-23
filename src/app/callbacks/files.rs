use {
    crate::{
        actions::{AnyActions, FilesActions, MainActions, UiActions},
        api::v3::{files_service_client::FilesServiceClient, FilesList},
        app::Application,
        service::{
            files::{
                dirs::Directory,
                loader::LoadStatus,
                path::ServerPath,
                FileManager,
            },
            ServiceError,
        },
        repository::pools::JoinHandlePool,
        FilesInternal,
        NotificationType,
    },
    slint::ComponentHandle,
    std::sync::Arc,
    tokio::sync::RwLock,
};

impl Application {
    pub fn init_files_service(&self, service: Arc<RwLock<FileManager>>) {
        let internal = self.ui_window.global::<FilesInternal>();
        let win = self.ui_window.as_weak();

        internal.on_update_list({
            let win = win.clone();
            let chan = self.chan.clone();
            let service = service.clone();

            move || {
                let win = win.clone();
                let service = service.clone();
                let chan = chan.clone();

                tokio::spawn(async move {
                    let client = FilesServiceClient::new(chan.read().await.as_ref().unwrap().intercepted());
                    let files = service.write().await.get_files(client.clone(), None).await;

                    match files {
                        Ok(res) => FilesActions::UpdateFilesList(
                            FilesList { value: res },
                            String::from("/"),
                        )
                        .run_in_event_loop(win.clone()),
                        Err(err) => return AnyActions::from(err).run_in_event_loop(win), // actions because, user can be unauthorized
                    };

                    match service.read().await.available_space(client).await {
                        Ok(s) => FilesActions::UpdateAvailableSpace(s.to_string_candy())
                            .run_in_event_loop(win),
                        Err(err) => AnyActions::from(err).run_in_event_loop(win),
                    };
                });
            }
        });

        internal.on_cd({
            let win = win.clone();
            let chan = self.chan.clone();
            let service = service.clone();

            move |target| {
                let win = win.clone();
                let service = service.clone();
                let chan = chan.clone();

                tokio::spawn(async move {
                    let client = FilesServiceClient::new(chan.read().await.as_ref().unwrap().intercepted());
                    let mut lock = service.write().await;

                    match if target != ".." {
                        lock.next(client, target.as_str()).await
                    } else {
                        lock.prev(client).await
                    } {
                        Ok(files) => FilesActions::UpdateFilesList(
                            FilesList { value: files },
                            lock.current_dir().into(),
                        )
                        .run_in_event_loop(win.clone()),
                        Err(err) => AnyActions::from(err).run_in_event_loop(win.clone()),
                    };
                    FilesActions::UpdateCurrentDirectory(lock.current_dir().into())
                        .run_in_event_loop(win);
                });
            }
        });

        internal.on_cd_current({
            let win = win.clone();
            let chan = self.chan.clone();
            let service = service.clone();

            move |id| {
                let win = win.clone();
                let service = service.clone();
                let chan = chan.clone();

                let id = id as usize;

                tokio::spawn(async move {
                    let client = FilesServiceClient::new(chan.read().await.as_ref().unwrap().intercepted());
                    let from = service.read().await.current_dir();

                    if from.count() == 0 || from.count() - 1 == id {
                        return
                    }

                    let target = ServerPath::new(Into::<Vec<String>>::into(from).into_iter().take(id + 1).collect());
                    let to = target.to_string();
                    match service.write().await.cd(client, target.clone()).await {
                        Ok(files) => {
                            FilesActions::UpdateFilesList(FilesList { value: files }, to).run_in_event_loop(win.clone());
                            FilesActions::UpdateCurrentDirectory(target.into()).run_in_event_loop(win);
                        },
                        Err(err) => {
                            AnyActions::from(err).run_in_event_loop(win);
                        }
                    }
                });
            }
        });

        internal.on_mkdir({
            let win = win.clone();
            let chan = self.chan.clone();
            let service = service.clone();

            move |dir_name| {
                let win = win.clone();
                let service = service.clone();
                let chan = chan.clone();

                tokio::spawn(async move {
                    let client = FilesServiceClient::new(chan.read().await.as_ref().unwrap().intercepted());

                    let resp = service
                        .write()
                        .await
                        .make_dir(client, None, dir_name.as_str())
                        .await;
                    match resp {
                        Ok(_) => {
                            let (files, from) = {
                                let lock = service.read().await;
                                (lock.cached_files(), lock.current_dir())
                            };
                            FilesActions::UpdateFilesList(FilesList { value: files }, from.into())
                                .run_in_event_loop(win);
                        }
                        Err(err) => AnyActions::from(err).run_in_event_loop(win),
                    };
                });
            }
        });

        internal.on_rmdir({
            let win = win.clone();
            let chan = self.chan.clone();
            let service = service.clone();

            move |dir_name| {
                let win = win.clone();
                let service = service.clone();
                let chan = chan.clone();

                tokio::spawn(async move {
                    let client = FilesServiceClient::new(chan.read().await.as_ref().unwrap().intercepted());

                    let resp = service
                        .write()
                        .await
                        .remove_dir(client, dir_name.as_str())
                        .await;
                    match resp {
                        Ok(_) => {
                            let (files, from) = {
                                let lock = service.read().await;
                                (lock.cached_files(), lock.current_dir())
                            };
                            FilesActions::UpdateFilesList(FilesList { value: files }, from.into())
                                .run_in_event_loop(win);
                        }
                        Err(err) => AnyActions::from(err).run_in_event_loop(win),
                    };
                });
            }
        });

        internal.on_upload_files({
            let win = win.clone();
            let chan = self.chan.clone();
            let service = service.clone();

            move || {
                let win = win.clone();
                let service = service.clone();
                let chan = chan.clone();

                let handles_pool = Arc::new(JoinHandlePool::with_capacity(25));

                tokio::spawn({
                    let handles_pool = handles_pool.clone();
                    let win = win.clone();
                    async move {
                        while let Some(res) = handles_pool.get_complete().await {
                            if let Err(err) = res {
                                AnyActions::from(err).run_in_event_loop(win.clone());
                            }
                        }
                    }
                });

                tokio::spawn(async move {
                    let client = FilesServiceClient::new(chan.read().await.as_ref().unwrap().intercepted());
                    let current_dir = service.read().await.current_dir();

                    let files = rfd::AsyncFileDialog::new()
                        .set_directory("~")
                        .pick_files()
                        .await;

                    let files = match files {
                        Some(v) => v,
                        None => return,
                    };

                    for f in files {
                        let service = service.clone();
                        let client = client.clone();
                        let current_dir = current_dir.clone();
                        
                        handles_pool.push_future(async move {
                            service.write().await.upload_file(client, Some(current_dir), f.path().to_path_buf()).await
                        }).await;
                    }
                });
            }
        });

        internal.on_upload_folders({
            let win = win.clone();
            let chan = self.chan.clone();
            let service = service.clone();

            move || {
                let win = win.clone();
                let service = service.clone();
                let chan = chan.clone();

                let handles_pool = Arc::new(JoinHandlePool::with_capacity(25));

                tokio::spawn({
                    let handles_pool = handles_pool.clone();
                    let win = win.clone();
                    async move {
                        while let Some(res) = handles_pool.get_complete().await {
                            if let Err(err) = res {
                                AnyActions::from(err).run_in_event_loop(win.clone());
                            }
                        }
                    }
                });

                tokio::spawn(async move {
                    let client = FilesServiceClient::new(chan.read().await.as_ref().unwrap().intercepted());
                    let current_dir = service.read().await.current_dir();

                    let dirs = rfd::AsyncFileDialog::new()
                        .set_directory("~")
                        .pick_folders()
                        .await;

                    let dirs = match dirs {
                        Some(v) => v,
                        None => return,
                    };

                    for dir in dirs {
                        let mut files = Directory::from_recursive(dir.path()).read();
                        while let Some(files) = files.recv().await {
                            if let Err(err) = service
                                .write()
                                .await
                                .make_dir(
                                    client.clone(),
                                    Some(current_dir.clone()),
                                    files
                                        .0
                                        .to_string()
                                        .trim_start_matches('/')
                                        .trim_end_matches('/'),
                                )
                                .await
                            {
                                AnyActions::from(err).run_in_event_loop(win.clone());
                                continue;
                            };

                            for f in files.1 {
                                let service = service.clone();
                                let client = client.clone();
                                let server_path = files.0.clone();

                                handles_pool.push_future(async move {
                                    service.write().await.upload_file(client.clone(), Some(server_path), f).await
                                }).await;
                            }
                        }
                    }
                });
            }
        });

        internal.on_download_file({
            let win = win.clone();
            let chan = self.chan.clone();
            let service = service.clone();

            move |filename| {
                let win = win.clone();
                let service = service.clone();
                let chan = chan.clone();

                tokio::spawn(async move {
                    let client = FilesServiceClient::new(chan.read().await.as_ref().unwrap().intercepted());
                    if let Err(err) = service
                        .write()
                        .await
                        .download_file(client, None, filename.to_string())
                        .await
                    {
                        AnyActions::from(err).run_in_event_loop(win);
                    }
                });
            }
        });

        internal.on_download_directory({
            let win = win.clone();
            let chan = self.chan.clone();
            let service = service.clone();

            move |dir_name| {
                let win = win.clone();
                let service = service.clone();
                let chan = chan.clone();

                let handles_pool = Arc::new(JoinHandlePool::with_capacity(25));

                tokio::spawn({
                    let handle_pool = handles_pool.clone();
                    let win = win.clone();
                    async move {
                        while let Some(res) = handle_pool.get_complete().await {
                            if let Err(err) = res {
                                AnyActions::from(err).run_in_event_loop(win.clone());
                            }
                        }
                    }
                });

                tokio::spawn(async move {
                    let client = FilesServiceClient::new(chan.read().await.as_ref().unwrap().intercepted());
                    let from = service.read().await.current_dir().to_string() + &dir_name;
                    
                    MainActions::ShowNotification(
                        "Using unstable method".to_string(), 
                        "This method is unstable and can work with some troubles".to_string(),
                        NotificationType::Info
                    ).run_in_event_loop(win.clone());

                    let download_files = match service
                        .write()
                        .await
                        .get_files(client.clone(), Some(from.clone()))
                        .await
                    {
                        Ok(files) => files,
                        Err(err) => {
                            AnyActions::from(err).run_in_event_loop(win);
                            return;
                        }
                    };

                    for f in download_files {
                        let service = service.clone();
                        let client = client.clone();
                        let from = from.clone();
                        handles_pool.push_future(async move {
                            service.write().await.download_file(client, Some(from), f.name).await
                        }).await;
                    }
                });
            }
        });

        internal.on_rmfile({
            let win = win.clone();
            let chan = self.chan.clone();
            let service = service.clone();

            move |target| {
                let win = win.clone();
                let chan = chan.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let client = FilesServiceClient::new(chan.read().await.as_ref().unwrap().intercepted());
                    let resp = service
                        .write()
                        .await
                        .remove_file(client, &target)
                        .await;

                    match resp {
                        Ok(_) => {
                            let (files, from) = {
                                let lock = service.read().await;
                                (lock.cached_files(), lock.current_dir())
                            };
                            FilesActions::UpdateFilesList(FilesList { value: files }, from.to_string()).run_in_event_loop(win);
                        },
                        Err(err) => AnyActions::from(err).run_in_event_loop(win),
                    }
                });
            }
        });

        internal.on_cancel_load({
            let service = service.clone();

            move |id| {
                let service = service.clone();
                tokio::spawn(async move {
                    service
                        .write()
                        .await
                        .cancel_load(id.parse().unwrap()) // I guarantee the string is u64
                        .await;
                });
            }
        });

        internal.on_update_load_files({
            let win = win.clone();
            let service = service.clone();

            FilesActions::UpdateLoadFiles(vec![]).run_in_event_loop(win.clone()); // set default value

            move || {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    let (load, complete) = {
                        let lock = service.read().await;
                        (lock.get_load_files().await, lock.get_loaded_files().await)
                    };

                    FilesActions::UpdateLoadFiles(load).run_in_event_loop(win.clone());

                    // Show downloaded files
                    complete.into_iter().for_each(|load| {
                        match load.status() {
                            LoadStatus::Completed => FilesActions::NotifyLoadedFile(load.file(), load.typ())
                                .run_in_event_loop(win.clone()),
                            _ => {},
                        }
                    });
                });
            }
        });
    }
}
