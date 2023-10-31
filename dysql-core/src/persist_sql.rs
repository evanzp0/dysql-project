use std::{path::PathBuf, env};

pub struct PersistSql {
    pub path: PathBuf,
}

impl PersistSql {
    pub fn new(path: PathBuf) -> Self {
        PersistSql {
            path
        }
    }

    pub fn create_path(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(self.path.as_path())
    }
}

impl Default for PersistSql {
    fn default() -> Self {
        let mut current_dir = env::current_dir().unwrap();
        current_dir.push("sql");

        PersistSql::new(current_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_path() {
        let ps = PersistSql::default();
        ps.create_path().unwrap();
    }
}