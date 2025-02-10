use std::sync::Arc;
use std::path::PathBuf;

#[derive(Clone)]
pub struct AppState {
    dist_dir: Arc<PathBuf>,
}

impl AppState {
    pub fn new(dist_dir: PathBuf) -> Self {
        let dist_dir = Arc::new(dist_dir);

        Self { dist_dir }
    }
    pub fn get_dist_dir(&self) -> Arc<PathBuf> {
        Arc::clone(&self.dist_dir)
    }
}
