use std::{path::PathBuf, env, collections::HashMap, sync::Arc, fs::{OpenOptions, File, read_to_string}, io::{Write, Seek}, io::{Read, SeekFrom}, str::FromStr};

use dysql_tpl::Template;

use crate::{SqlxVer, get_sqlx_ver};

#[derive(Debug)]
pub struct DysqlContext {
    pub sql_fd: PathBuf,
    pub meta_path: PathBuf,
    pub meta_infos: HashMap<u64, String>,
    pub templats: HashMap<u64, Arc<Template>>,
    pub sqlx_ver: SqlxVer,
}

impl<'a> DysqlContext {
    pub fn new(sql_fd: PathBuf) -> Self {
        std::fs::create_dir_all(sql_fd.as_path()).unwrap();

        let mut meta_path = sql_fd.clone();
        meta_path.push("meta.dat");

        let sqlx_ver = get_sqlx_ver().expect("error: can not get sql version");
        if let SqlxVer::NotSupport(ver) = sqlx_ver {
            panic!("dysql not support the sqlx version: {}", ver);
        }

        let mut me = DysqlContext {
            sql_fd,
            meta_path: meta_path,
            meta_infos: Default::default(),
            templats: Default::default(),
            sqlx_ver,
        };

        match std::env::var("DYSQL_PESIST_SQL") {
            Ok(val) if val.to_ascii_uppercase() == "TRUE" => {
                // 加载持久化 sql
                me.load();
            },
            _ => (),
        }

        // println!("psql: {:?}", me.sqlx_ver);

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
    
                let meta_id : u64 = FromStr::from_str(content[0]).expect("meta_id must be type of u64");
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

                            let template_id: u64 = FromStr::from_str(template_id).expect("template_id must be type of u64");
                            let template = Arc::new(Template::new(sql).unwrap());

                            self.insert_template(template_id, template);
                        }
                    }
                }
            }
        }
    }

    pub fn get_template(&self, template_id: u64) -> Option<Arc<Template>>  {
        self.templats.get(&template_id).map(|tpl| tpl.clone())
    }

    pub fn insert_template(&mut self, template_id: u64, template: Arc<Template>) -> Option<Arc<Template>> {
        self.templats.insert(template_id, template)
    }

    pub fn save_sql_template(
        &mut self, 
        meta_id: u64, 
        source_file: String, 
        template_id: u64, 
        template: Arc<Template>,
        sql_name: Option<String>
    ) {
        let template_source = template.source().to_owned();
        if template_source.trim().is_empty() {
            panic!("source file path is empty in meta.id");
        }

        let rst = self.meta_infos.insert(meta_id, source_file.clone());

        if let None = rst {
            let content = meta_id.to_string() + ": " + &source_file + "\n";
            Self::append(&self.meta_path, content)
        }

        
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

    pub fn default() -> Self {
    
        let mut current_dir = if let Ok(_) = std::env::var("CARGO") {
            // 是在 cargo 下运行
            env::current_dir().unwrap()
        } else {
            // 非 cargo 运行
            let mut tmp_dir = env::current_exe().unwrap();
            tmp_dir.pop();
            tmp_dir
        };

        current_dir.push(".dysql");
        // println!("current_dir: {:?}", current_dir);
        
        DysqlContext::new(current_dir)
    }
}