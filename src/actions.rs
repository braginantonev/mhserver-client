use {
    super::{
        AppStates, MainWindow, NotificationType, PreparingStates
    },
    crate::notification, slint::Weak,
};

pub enum UiActions {
    ChangeAppState(AppStates),
    ChangePreparingState(PreparingStates),
    ShowNotification(String, NotificationType)
}

impl UiActions {
    pub fn run(self, win: MainWindow) {
        match self {
            UiActions::ChangeAppState(next) => win.set_state(next),
            UiActions::ChangePreparingState(next) => {
                win.set_prepare_state(next);
                win.invoke_change_preparing_state(next);
            },
            UiActions::ShowNotification(desc, r#type) => {
                notification::show(win, desc.as_str(), r#type);
            }
        }
    }

    pub fn run_in_event_loop(self, weak_win: Weak<MainWindow>) {
        let _ = weak_win.upgrade_in_event_loop(move |win| {
            self.run(win);
        });
    }
}