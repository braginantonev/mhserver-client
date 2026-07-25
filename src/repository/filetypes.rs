use {
    api::models::FilesListInner, std::path::Path,
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

const ICONS_PATH: &str = "ui/assets/file-icons";

impl FileTypes {
    pub fn to_slint_image(&self) -> Result<slint::Image, slint::LoadImageError> {
        slint::Image::load_from_path(Path::new(&self.to_file_path()))
    }

    pub fn to_file_path(&self) -> String {
        format!("{}/{}", ICONS_PATH, match self {
            FileTypes::Directory => "folder.png",
            FileTypes::Text => "text.png",
            FileTypes::Document => "document.png",
            FileTypes::Image => "image.png",
            FileTypes::Video => "video.png",
            FileTypes::Music => "music.png",
            FileTypes::Executable => "executable.png",
            FileTypes::Undefined => "undefined.png",
        })
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