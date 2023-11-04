use std::{path::PathBuf, env, collections::HashMap, sync::Arc, fs::{OpenOptions, File, read_to_string}, io::{Write, Seek}, io::{Read, SeekFrom}, str::FromStr, ops::Index};

use dysql_tpl::Template;

#[derive(Debug)]
pub struct PersistSql<'a> {
    pub sql_fd: PathBuf,
    pub meta_path: PathBuf,
    pub meta_infos: HashMap<u64, String>,
    pub templats: HashMap<u64, Arc<Template<'a>>>,
}

impl<'a> PersistSql<'a> {
    pub fn new(sql_fd: PathBuf) -> Self {
        std::fs::create_dir_all(sql_fd.as_path()).unwrap();

        let mut meta_path = sql_fd.clone();
        meta_path.push("meta.dat");

        let mut me = PersistSql {
            sql_fd,
            meta_path: meta_path,
            meta_infos: Default::default(),
            templats: Default::default(),
        };

        match std::env::var("DYSQL_PESIST_SQL") {
            Ok(val) if val == "TRUE" => {
                // 加载持久化 sql
                me.load();
            },
            _ => (),
        }

        me
    }
    
    fn load(&mut self) {
        if !self.meta_path.exists() {
            File::create(&self.meta_path).unwrap();
        } else {
            let rst = read_to_string(&self.meta_path.as_path());
            
            // 从 meta.dat 文件中加载 meta info
            for line in rst.unwrap().lines() {
                let line = line.trim();
                if line == "" {
                    continue;
                }
    
                let content: Vec<_> = line.split(":").collect();
                if content.len() != 2 {
                    panic!("meta.dat file content error");
                }
    
                let meta_id : u64 = FromStr::from_str(content[0]).unwrap();
                let source_file: String = content[1].to_string();
                self.meta_infos.insert(meta_id, source_file);
    
                // 从 template 文件中加载 sql
                let mut template_file = self.sql_fd.clone();
                template_file.push(meta_id.to_string() + ".dat");

                if template_file.exists() {
                    let mut template_id: &str = "";
                    for (line_no, line) in read_to_string(&template_file).unwrap().lines().enumerate() {
                        if line_no % 2 == 0 {
                            let line = line.trim();
                            let offset = line.find(':').unwrap_or(line.len());
                            template_id = &line[0..offset];
                        } else {
                            let sql = line.trim();

                            let template_id: u64 = FromStr::from_str(template_id).unwrap();
                            let template = Arc::new(Template::new(sql.to_owned()).unwrap());

                            self.insert_template(template_id, template);
                        }
                    }
                }
            }
        }
    }

    pub fn get_template(&self, template_id: u64) -> Option<Arc<Template<'a>>>  {
        self.templats.get(&template_id).map(|tpl| tpl.clone())
    }

    pub fn insert_template(&mut self, template_id: u64, template: Arc<Template<'a>>) -> Option<Arc<Template<'a>>> {
        self.templats.insert(template_id, template)
    }

    pub fn save_sql_template(
        &mut self, 
        meta_id: u64, 
        source_file: String, 
        template_id: u64, 
        template: Arc<Template<'a>>,
        sql_name: Option<String>
    ) {
        let rst = self.meta_infos.insert(meta_id, source_file.clone());

        if let None = rst {
            let content = meta_id.to_string() + ": " + &source_file + "\n";
            Self::append(&self.meta_path, content)
        }

        let template_source = template.source().to_owned();
        let rst = self.templats.insert(template_id, template);
        if let None = rst {
            let mut template_file = self.sql_fd.clone();
            template_file.push(meta_id.to_string() + ".dat");
            
            let content = template_id.to_string() + ": " + &sql_name.unwrap_or_default() + "\n  " + &template_source + "\n";
            Self::append(&template_file, content);
        }
    }

    fn append(path: &PathBuf, content: String) {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path.as_path())
            .expect(&format!("unable to open file: {:?}", path));

        let mut cur = file.metadata().unwrap().len();
        let mut buf = [0 as u8];

        if cur >= 0 as u64 {
            file.seek(SeekFrom::Start(cur)).ok();

            while let Ok(_) = file.read(&mut buf) {
                if buf[0] != 10 && buf[0] != 13 && buf[0] != 0 {
                    break;
                }
                
                if cur == 0 {
                    file.seek(SeekFrom::Start(cur)).ok();
                    break;
                } else {
                    cur -= 1;
                }
                
                file.seek(SeekFrom::Start(cur)).ok();
            }
        }

        // let c = meta_file.seek(SeekFrom::Current(0)).unwrap();
        if cur > 0 as u64 {
            file.seek(SeekFrom::Start(cur + 1)).ok();
            file.write("\n".as_bytes()).expect(&format!("write file error: {:?}", path));
        }

        file.write(content.as_bytes()).expect(&format!("write file error: {:?}", path));
    }
}

impl Default for PersistSql<'_> {
    fn default() -> Self {
        let mut current_dir = env::current_dir().unwrap();
        current_dir.push(".dysql");

        PersistSql::new(current_dir)
    }
}
