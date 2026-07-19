use {
    super::{
        AppStates, File, MainWindow, NotificationType, PreparingStates, Services, repository::filetypes::FileTypes, LoadDataInfo
    }, crate::{notification}, slint::{ModelRc, ToSharedString, VecModel, Weak}, std::rc::Rc,
};

#[derive(Debug, Clone)]
pub enum UiActions {
    /// Change application state to target
    ChangeAppState(AppStates),

    /// Change preparing state to target
    ChangePreparingState(PreparingStates),

    /// Change active service to target
    ChangeActiveService(Services),

    /// Show notification with description and type
    ShowNotification(String, NotificationType),

    /// Update files in data service. Required the files, and server path, where is this files located. 
    DataUpdateFilesList(Vec<api::models::FilesListInner>, String),

    DataUpdateLoadFiles(Vec<(uuid::Uuid, bool, String, f32)>),
}

impl UiActions {
    fn run(self, win: MainWindow) {
        match self {
            UiActions::ChangeAppState(next) => win.set_state(next),
            UiActions::ChangePreparingState(next) => {
                win.set_prepare_state(next);
                win.invoke_change_preparing_state(next);
            },
            UiActions::ChangeActiveService(new_service) => {
                win.set_active_service(new_service);
            },
            UiActions::ShowNotification(desc, r#type) => {
                notification::show(win, desc.as_str(), r#type);
            },
            UiActions::DataUpdateFilesList(files, from) => {
                let slint_files = files.iter().map(|f| {
                    File {
                        icon: FileTypes::from(f).to_slint_image().expect("failed load file icon"),
                        name: f.name.to_shared_string(),
                        server_path: from.to_shared_string(),
                        is_dir: f.is_dir.unwrap_or(false),
                        size: f.size.unwrap_or(0) as i32,
                    }
                });
                win.invoke_files_update_showed_files(ModelRc::from(Rc::new(VecModel::from_iter(slint_files))));
            },
            UiActions::DataUpdateLoadFiles(files) => {
                let slint_files = files.iter().map(|f| {
                    LoadDataInfo { connID: f.0.to_shared_string(), is_upload: f.1, load_to: "".to_shared_string(), name: f.2.to_shared_string(), progress: f.3 }
                });
                win.invoke_files_update_load_data(ModelRc::from(Rc::new(VecModel::from_iter(slint_files))));
            },
        }
    }

    pub fn run_in_event_loop(self, weak_win: Weak<MainWindow>) {
        let _ = weak_win.upgrade_in_event_loop(move |win| {
            self.run(win);
        });
    }
}