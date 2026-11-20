use {
    crate::{app::Application, RFDApi},
    slint::{ComponentHandle, ToSharedString},
};

impl Application {
    pub fn init_apis(&self) {
        let rfd_api = self.ui_window.global::<RFDApi>();

        rfd_api.on_pick_file(move || {
            rfd::FileDialog::new()
                .set_title("Choice server cert file")
                .add_filter("Certs", &["pem"])
                .pick_file()
                .map(|x| x.display().to_shared_string())
                .unwrap_or_default()
        });
    }
}
