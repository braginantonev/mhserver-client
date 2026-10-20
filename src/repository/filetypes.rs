use {
    crate::{api::FileInfo, FileIcons},
    slint::Weak,
};

pub enum FileTypes {
    Directory,
    Text,
    Document,
    Image,
    Video,
    Music,
    Executable,
    Undefined,
}

impl FileTypes {
    pub fn to_slint_image(&self, assets: Weak<FileIcons<'static>>) -> slint::Image {
        let strong = assets.unwrap();
        match self {
            FileTypes::Directory => strong.get_folder(),
            FileTypes::Text => strong.get_text(),
            FileTypes::Document => strong.get_document(),
            FileTypes::Image => strong.get_image(),
            FileTypes::Video => strong.get_video(),
            FileTypes::Music => strong.get_music(),
            FileTypes::Executable => strong.get_executable(),
            _ => strong.get_undefined(),
        }
    }
}

impl From<&FileInfo> for FileTypes {
    fn from(value: &FileInfo) -> Self {
        if value.is_dir {
            return FileTypes::Directory;
        }

        FileTypes::from(if !value.name.contains('.') {
            "exe" // Linux use empty extension like executable file
        } else {
            value.name.split('.').last().unwrap_or_default()
        })
    }
}

impl From<&str> for FileTypes {
    fn from(value: &str) -> Self {
        match value {
            "exe" | "bat" | "sh" => FileTypes::Executable,
            "txt" | "md" | "markdown" => FileTypes::Text,
            "doc" | "docx" | "ods" | "odx" => FileTypes::Document,
            "png" | "jpg" | "jpeg" | "webp" | "gif" => FileTypes::Image,
            "mp4" => FileTypes::Video,
            "mp3" | "flac" => FileTypes::Music,
            _ => FileTypes::Undefined,
        }
    }
}
