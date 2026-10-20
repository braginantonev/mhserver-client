use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint_build::compile("ui/app-window.slint")?;
    let mut ver_file = std::fs::File::create("VERSION")?;
    ver_file.write_all(env!("CARGO_PKG_VERSION").as_bytes())?;

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(
            &["proto/auth/auth.proto", "proto/data/data.proto"],
            &["proto"],
        )?;

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("docs/logo.ico");
        res.compile().expect("failed compile windows resourcess");
    };

    Ok(())
}
