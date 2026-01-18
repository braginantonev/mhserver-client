use {
    super::{ MainWindow, NotificationInfo, NotificationType }, 
    slint::{ ToSharedString }
};

pub fn show(moved_win: MainWindow, desc: &str, typ: NotificationType) {
    moved_win.invoke_update_main_notification(NotificationInfo {
        id: "".to_shared_string(), 
        text: desc.to_shared_string(),
        r#type: typ 
    });
}