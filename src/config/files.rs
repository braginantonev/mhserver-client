use std::path::PathBuf;
use api::apis::configuration::Configuration;

use crate::repository::dirs::default_download_dir;

#[derive(Debug)]
pub struct FileServiceConfig {
    // вот эта залупень и ещё та, что ниже - честно это пиздец
    // это нужно, просто чтобы избавиться от 429 + зависания наглухо программы, 
    // в огромном поотоке запросов на сохранение чанка, пока нахуй очередь дойдёт до запроса обновления файлов - в доте выйдет новый патч
    // и это пиздец. Отсюда вопрос, а нахуй нам посылать столько запросов одновременно - давно ведь придумали блядские соединения?
    // Ха-ха, идите нахуй. Я о них узнал давно, однако мне было лень переделывать хуетень на сервере, поэтому пока не выйдет api 3.x будет жёсткая дрочка.
    // Но судя по всему эту дрочку я очень скоро начну фиксить, ибо не приятно, когда тебя долбят взад.
    // Этот код полное дерьмо. Тот, кто это читает, прошу, никогда блять подобное не пиши, умоляю, сука на коленях стою!
    pub first_api_conf: Configuration,
    pub second_api_conf: Configuration,
    download_dir: PathBuf,
}

impl FileServiceConfig {
    pub fn new(first_api_conf: Configuration, second_api_conf: Configuration, download_dir: Option<PathBuf>) -> Self {
        Self { first_api_conf, second_api_conf: second_api_conf, download_dir: download_dir.unwrap_or(default_download_dir()) }
    }

    pub fn download_dir(&self) -> PathBuf {
        self.download_dir.clone()
    }
}

impl Default for FileServiceConfig {
    fn default() -> Self {
        Self { first_api_conf: Configuration::default(), second_api_conf: Configuration::default(), download_dir: default_download_dir() }
    }
}