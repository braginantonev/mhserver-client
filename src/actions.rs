use {
    super::{
        File, MainWindow, NotificationType, repository::filetypes::FileTypes
    }, crate::{FilesInternal, LoadFile, NotificationInfo, NotificationsInternal, PreparingStates, State, service::{ServiceError, files::connections::ConnectionInfo}}, reqwest::StatusCode, slint::{ComponentHandle, ModelRc, ToSharedString, VecModel, Weak}, std::rc::Rc,
};

pub trait UiActions: TryFrom<ServiceError> {
    fn run(self, win: MainWindow);
    fn run_in_event_loop(self, weak_win: Weak<MainWindow>)
    where
        Self: Sized + Send + 'static
    {
        let _ = weak_win.upgrade_in_event_loop(move |win| {
            self.run(win);
        });
    }
}

pub enum PreparingActions {
    /// Invoke next() method of State global
    InvokeNextState,

    ForceAuthorization,
}

impl TryFrom<ServiceError> for PreparingActions {
    type Error = ();
    fn try_from(value: ServiceError) -> Result<Self, Self::Error> {
        match value.code().unwrap_or_default() {
            StatusCode::UNAUTHORIZED => Ok(PreparingActions::ForceAuthorization),
            _ => Err(())
        }
    }
}

impl UiActions for PreparingActions {
    fn run(self, win: MainWindow) {
        match self {
            PreparingActions::InvokeNextState => {
                win.global::<State>().invoke_next();
            },
            PreparingActions::ForceAuthorization => {
                win.global::<State>().invoke_force_preparing_state(PreparingStates::Login);
            },
        }
    }
}

pub enum MainActions {
    /// Show notification with description and type
    ShowNotification(String, String, NotificationType),
}

impl UiActions for MainActions {
    fn run(self, win: MainWindow) {
        match self {
            MainActions::ShowNotification(label, desc, r#type) => {
                win.global::<NotificationsInternal>().invoke_push(NotificationInfo {
                    id: 0, // will be override in callback
                    r#type,
                    label: label.to_shared_string(),
                    description: desc.to_shared_string(),
                });
            },
        }
    }
}

impl From<ServiceError> for MainActions {
    fn from(value: ServiceError) -> Self {
        MainActions::ShowNotification(value.label(), value.description(), NotificationType::Error)
    }
}

pub enum FilesActions {
    /// Update files in data service. Required the files, and server path, where is this files located. 
    UpdateFilesList(Vec<api::models::FilesListInner>, String),

    UpdateLoadFiles(Vec<ConnectionInfo>),

    UpdateCurrentDirectory(String),

    UpdateAvailableSpace(String),
}

impl UiActions for FilesActions {
    fn run(self, win: MainWindow) {
        match self {
            FilesActions::UpdateFilesList(files, from) => {
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
            FilesActions::UpdateLoadFiles(files) => {
                win.global::<FilesInternal>().set_load_files(ModelRc::from(Rc::new(VecModel::from_iter(files.iter().map(|conn| {
                    LoadFile { connID: conn.id.to_shared_string(), is_upload: conn.is_upload, name: conn.filename.to_shared_string(), progress: conn.load_progress, previous: conn.previous_progress }
                })))));
            },
            FilesActions::UpdateCurrentDirectory(target) => {
                win.global::<FilesInternal>().set_current_directory(target.to_shared_string());
            },
            FilesActions::UpdateAvailableSpace(size) => {
                win.global::<FilesInternal>().set_available_space(size.to_shared_string());
            }
        }
    }
}

impl TryFrom<ServiceError> for FilesActions {
    type Error = ();
    fn try_from(_: ServiceError) -> Result<Self, Self::Error> {
        Err(()) // tmp
    }
}

pub enum AnyActions {
    Preparing(PreparingActions),
    Main(MainActions),
    Files(FilesActions),
}

impl UiActions for AnyActions {
    fn run(self, win: MainWindow) {
        match self {
            AnyActions::Preparing(preparing_actions) => preparing_actions.run(win),
            AnyActions::Main(main_actions) => main_actions.run(win),
            AnyActions::Files(files_actions) => files_actions.run(win),
        }  
    }
}

impl From<ServiceError> for AnyActions {
    fn from(value: ServiceError) -> Self {
        // I use the priority system
        // First we check preparing states, because not prepared app - not working
        // Next will be services, and last - main actions. Because, main actions do not affect the app functionality

        if let Ok(act) = PreparingActions::try_from(value.clone()) {
            return AnyActions::Preparing(act);
        }

        if let Ok(act) = FilesActions::try_from(value.clone()) {
            return AnyActions::Files(act)
        }

        AnyActions::Main(MainActions::from(value))
    }
}