pub mod entry;
pub mod entry_composite;
pub mod entry_dir;
pub mod entry_wildcard;
pub mod entry_zip;

use crate::classpath::entry::{Entry, new_entry};
use crate::classpath::entry_wildcard::WildcardEntry;
use std::fmt;
use std::path::Path;
use std::env;


pub struct Classpath {
    pub boot_classpath: Box<dyn Entry>,
    pub ext_classpath: Box<dyn Entry>,
    pub user_classpath: Box<dyn Entry>,
    
}

impl Classpath {
    pub fn parse(jre_option: &str, cp_option: &str) -> Self {
        let boot_classpath = Classpath::parse_boot_classpath(jre_option);
        let ext_classpath = Classpath::parse_ext_classpath(jre_option);
        let user_classpath = Classpath::parse_user_classpath(cp_option);
        let cp = {
            Classpath{
                boot_classpath, 
                ext_classpath,
                user_classpath,
            }
        };
        cp
    }

    fn parse_boot_classpath(jre_option: &str) -> Box<dyn Entry> {
        let jre_dir = Classpath::get_jre_dir(jre_option); 
        // jre/lib/*
        let path = Path::new(&jre_dir).join("lib").join("*");
        let jre_lib_path = path.to_str().unwrap();
        Box::new(WildcardEntry::new(jre_lib_path))
    } 


    fn parse_ext_classpath(jre_option: &str) -> Box<dyn Entry> {
        let jre_dir = Classpath::get_jre_dir(jre_option); 
        // jre/lib/*
        let path = Path::new(&jre_dir).join("lib").join("ext").join("*");
        let jre_ext_path = path.to_str().unwrap();
        Box::new(WildcardEntry::new(jre_ext_path))
    }

    fn parse_user_classpath(cp_option: &str) -> Box<dyn Entry> {
        let mut cp = cp_option;
        if cp == ""{
            cp = ".";
        }
        new_entry(cp)
    }

    fn get_jre_dir(jre_option: &str) -> String {
        print!("get jre_dir, param:{}", jre_option);
        // use param    
        if jre_option != "" {
            let jre_dir = Path::new(jre_option);
            if jre_dir.exists() {
                print!("param jre_dir is exist !!!");
                return jre_option.to_string();
            }
        }

        let jre_dir = Path::new("./jre");
        if jre_dir.exists() {
            return "./jre".to_string();
        }

        match env::var("JAVA_HOME") {
            Ok(jh) => {
                if jh != "" {
                    return Path::new(&jh).join("jre").to_str().unwrap().to_string();
                }
            },
            Err(_err) => {},
        }

        panic!("{}", "Can not find jre folder!")
        
    }

}

impl Entry for Classpath {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        let class = class_name.to_string() + ".class";
        return match self.boot_classpath.read_class(&class) {
            Ok(data) => Ok(data),
            Err(_err) => {
                 match self.ext_classpath.read_class(&class) {
                    Ok(data) => Ok(data),
                    Err(_err) => {
                        match self.user_classpath.read_class(&class) {
                            Ok(data) => Ok(data),
                            Err(err) => {
                                return Err(err)
                            }
                        }
                    }
                }
            }
        }
    }
}
 impl fmt::Display for Classpath{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.user_classpath)
    }
}