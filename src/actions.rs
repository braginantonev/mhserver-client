use {
    super::{
        AppStates, File, MainWindow, NotificationType, PreparingStates, Services, repository::filetypes::FileTypes
    },
    crate::notification, 
    slint::{ToSharedString, Weak, ModelRc, VecModel}, 
    std::rc::Rc,
};

pub enum UiActions {
    ChangeAppState(AppStates),
    ChangePreparingState(PreparingStates),
    ChangeActiveService(Services),
    ShowNotification(String, NotificationType),
    DataUpdateFilesList(Vec<api::data::FileInfo>),
}

impl UiActions {
    pub fn run(self, win: MainWindow) {
        match self {
            UiActions::ChangeAppState(next) => win.set_state(next),
            UiActions::ChangePreparingState(next) => {
                win.set_prepare_state(next);
                win.invoke_change_preparing_state(next);
            },
            UiActions::ChangeActiveService(new_service) => {
                win.set_active_service(new_service);
            }
            UiActions::ShowNotification(desc, r#type) => {
                notification::show(win, desc.as_str(), r#type);
            }
            UiActions::DataUpdateFilesList(files) => {
                let slint_files = files.iter().map(|f| {
                    File {
                        icon: FileTypes::from(f).to_slint_image().expect("failed load file icon"),
                        name: f.name().to_shared_string(),
                        server_path: "".to_shared_string()
                    }
                });

                win.invoke_data_update_showed_files(ModelRc::from(Rc::new(VecModel::from_iter(slint_files))));
            }
        }
    }

    pub fn run_in_event_loop(self, weak_win: Weak<MainWindow>) {
        let _ = weak_win.upgrade_in_event_loop(move |win| {
            self.run(win);
        });
    }
}