use {
    crate::FileIcons, api::models::FilesListInner, slint::Weak,
};

pub enum FileTypes {
    Directory,
    Text,
    Document,
    Image,
    Video,
    Music,
    Executable,
    Undefined
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

impl From<&FilesListInner> for FileTypes {
    fn from(value: &FilesListInner) -> Self {
        if value.is_dir != None {
            return FileTypes::Directory
        }

        FileTypes::from(if !value.name.contains('.') {
            "exe" // Linux use empty extension like executable file
        } else {
            match value.name.split('.').last() {
                Some(x) => x,
                None => "" 
            }
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
            _ => FileTypes::Undefined
        }
    }
}