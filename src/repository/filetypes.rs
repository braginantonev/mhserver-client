use {
    std::path::Path,
    openapi::models::FilesListInner,
};

pub enum FileTypes {
    Directory,
    Text,
    Image,
    Executable,
    Undefined
}

const ICONS_PATH: &str = "ui/assets/file-icons";

impl FileTypes {
    pub fn to_slint_image(&self) -> Result<slint::Image, slint::LoadImageError> {
        let file_icon = match self {
            FileTypes::Directory => "folder.png",
            FileTypes::Text => "text.png",
            FileTypes::Image => "image.png",
            FileTypes::Executable => "executable.png",
            FileTypes::Undefined => "undefined.png",
        };

        slint::Image::load_from_path(Path::new(format!("{}/{}", ICONS_PATH, file_icon).as_str()))
    }
}

impl From<&FilesListInner> for FileTypes {
    fn from(value: &FilesListInner) -> Self {
        if value.is_dir != None {
            return FileTypes::Directory
        }

        let extension = if !value.name.contains('.') {
            "exe" // Linux use empty extension like executable file
        } else {
            match value.name.split('.').last() {
                Some(x) => x,
                None => "UNDEFINED" 
            }
        };

        match extension {
            "exe" => FileTypes::Executable,
            "txt" => FileTypes::Text,
            "png" | "jpg" | "jpeg" | "webp" => FileTypes::Image,
            _ => FileTypes::Undefined
        }
    }
}