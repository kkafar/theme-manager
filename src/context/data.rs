use std::path::PathBuf;

use crate::constant::ConstantRepo;

pub struct DataRepo {
    pub app_data_dir: PathBuf
}

impl DataRepo {
    // pub fn new() -> Self {
    //     Self {
    //         app_data_dir: DataRepo::default_data_dir().unwrap(),
    //     }
    // }

    pub fn default_data_dir() -> Option<PathBuf> {
        Some(dirs::data_dir().unwrap().join(ConstantRepo::app_name()))
    }
}

impl Default for DataRepo {
    fn default() -> Self {
        Self {
            app_data_dir: DataRepo::default_data_dir().unwrap(),
        }
    }
}
