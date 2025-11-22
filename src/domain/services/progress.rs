use serde::{Deserialize, Serialize};

use crate::{
    core::settings::app_settings,
    dal::local::adapters::files::{CrudAdapter, FileAdapter},
    debug, warn,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    pub last_image_url_id: String,
    pub how_many_scrolls: u32,
}

impl Progress {
    pub fn new(&self, last_image_url_id: String, how_many_scrolls: u32) -> Self {
        // FileAdapter implementation to load progress from a file would go here
        //
        //
        let base_path = format!("{}/dal/local", self.root_path());
        let extension = "json";

        let file_adapter = FileAdapter::<Progress>::new(base_path, extension).expect("Not Found!");

        let id = "progress"; // &str implements Display, so it's fine as Id

        let actual_progress = Progress {
            last_image_url_id,
            how_many_scrolls,
        };

        match file_adapter.read(&id) {
            Ok(Some(progress)) => {
                return progress;
            }
            Ok(None) => {
                warn!("Progress file not found, starting fresh.");
                file_adapter
                    .create(&id, &actual_progress)
                    .expect("Failed to create progress file.");

                return actual_progress;
            }
            Err(e) => {
                panic!("Error reading progress file: {:?}", e);
            }
        }
    }

    pub fn update_progress(&self, last_image_url_id: String, how_many_scrolls: u32) {
        let base_path = format!("{}/dal/local", self.root_path());
        let extension = "json";

        let file_adapter = FileAdapter::<Progress>::new(base_path, extension).expect("Not Found!");

        let id = "progress"; // &str implements Display, so it's fine as Id

        let updated_progress = Progress {
            last_image_url_id,
            how_many_scrolls,
        };

        match file_adapter.update(&id, &updated_progress) {
            Ok(_) => {
                debug!("Progress updated successfully.");
            }
            Err(e) => {
                panic!("Error updating progress file: {:?}", e);
            }
        }
    }

    pub fn root_path(&self) -> &String {
        return &app_settings().root_path;
    }

    pub fn save_extraction_progress(&self) {}
}
