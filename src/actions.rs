use {
    super::{
        File, MainWindow, NotificationType, repository::filetypes::FileTypes
    }, crate::{FilesInternal, LoadFile, PreparingStates, State, notification, service::{ServiceError, files::connections::ConnectionInfo}}, reqwest::StatusCode, slint::{ComponentHandle, ModelRc, ToSharedString, VecModel, Weak}, std::rc::Rc,
};

pub enum UiActions {
    /// Invoke next() method of State global
    InvokeNextState,

    ForceAuthorization,

    /// Show notification with description and type
    ShowNotification(String, NotificationType),

    /// Update files in data service. Required the files, and server path, where is this files located. 
    FilesUpdateFilesList(Vec<api::models::FilesListInner>, String),

    FilesUpdateLoadFiles(Vec<ConnectionInfo>),

    FilesUpdateCurrentDirectory(String),

    FilesUpdateAvailableSpace(String),
}

impl UiActions {
    fn run(self, win: MainWindow) {
        match self {
            UiActions::InvokeNextState => {
                win.global::<State>().invoke_next();
            },
            UiActions::ForceAuthorization => {
                win.global::<State>().invoke_force_preparing_state(PreparingStates::Login);
            },
            UiActions::ShowNotification(desc, r#type) => {
                notification::show(win, desc.as_str(), r#type);
            },
            UiActions::FilesUpdateFilesList(files, from) => {
                win.global::<FilesInternal>().set_showed_files(ModelRc::from(Rc::new(VecModel::from_iter(files.iter().map(|f| {
                    File {
                        icon: FileTypes::from(f).to_slint_image().expect("failed load file icon"),
                        name: f.name.to_shared_string(),
                        server_path: from.to_shared_string(),
                        is_dir: f.is_dir.unwrap_or(false),
                        size: f.size.unwrap_or(0) as i32,
                    }
                })))));
            },
            UiActions::FilesUpdateLoadFiles(files) => {
                win.global::<FilesInternal>().set_load_files(ModelRc::from(Rc::new(VecModel::from_iter(files.iter().map(|conn| {
                    LoadFile { connID: conn.id.to_shared_string(), is_upload: conn.is_upload, name: conn.filename.to_shared_string(), progress: conn.load_progress, previous: conn.previous_progress }
                })))));
            },
            UiActions::FilesUpdateCurrentDirectory(target) => {
                win.global::<FilesInternal>().set_current_directory(target.to_shared_string());
            },
            UiActions::FilesUpdateAvailableSpace(size) => {
                win.global::<FilesInternal>().set_available_space(size.to_shared_string());
            }
        }
    }

    pub fn run_in_event_loop(self, weak_win: Weak<MainWindow>) {
        let _ = weak_win.upgrade_in_event_loop(move |win| {
            self.run(win);
        });
    }
}

impl From<ServiceError> for UiActions {
    fn from(value: ServiceError) -> Self {
        if let Some(code) = value.code() {
            match code {
                StatusCode::UNAUTHORIZED => return UiActions::ForceAuthorization,
                _ => (), // not implement
            }
        }
        UiActions::ShowNotification(value.label(), NotificationType::Error)
    }
}