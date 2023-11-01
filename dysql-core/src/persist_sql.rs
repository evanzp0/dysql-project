use std::{path::PathBuf, env};

pub struct PersistSql {
    pub sql_fd: PathBuf,
}

impl PersistSql {
    pub fn new(path: PathBuf) -> Self {
        PersistSql {
            sql_fd: path
        }
    }

    pub fn prepare_sql_fd(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(self.sql_fd.as_path())
    }
}

impl Default for PersistSql {
    fn default() -> Self {
        let mut current_dir = env::current_dir().unwrap();
        current_dir.push(".sql");

        PersistSql::new(current_dir)
    }
}
