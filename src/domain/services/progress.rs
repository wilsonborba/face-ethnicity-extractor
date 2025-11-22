use crate::dal::local::adapters::files::FileAdapter;

pub struct Progress {
    pub last_image_url: String,
    pub how_many_scrolls: u32,
}

impl Progress {
    pub fn new(&self) {
        // FileAdapter implementation to load progress from a file would go here
        //
        //

        //let file_adapter = FileAdapter::new(base_path, extension);
    }

    pub fn save_extraction_progress(&self) {}
}
