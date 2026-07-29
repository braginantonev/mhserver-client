use std::io::Write;

fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
    let mut ver_file = std::fs::File::create("VERSION").expect("failed create version file");
    ver_file.write_all(env!("CARGO_PKG_VERSION").as_bytes()).expect("failed save application crate version");
}
