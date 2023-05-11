use std::collections::HashMap;
use serde::de::Unexpected::Option;
use crate::libs::basic::TplService;
use tera::{Tera, Context};


#[derive(Default, Debug, Clone)]
pub struct TeraTplService {
    tpl_instance: Tera,
}

impl TeraTplService {
    pub fn load_template(&mut self, path: String) {
        match Tera::parse(&path) {
            Ok(v) => {
                match self.tpl_instance.extend(&v) {
                    Ok(()) => {}
                    Err(e) => {
                        println!("[load_template] Tera::extend error: {:#?}", e);
                    }
                };
            }
            Err(e) => {
                println!("[load_template] Tera::parse error: {:#?}", e);
            }
        }
    }

    pub fn load_raw_template(&mut self, name: &str, content: &str) {
        match self.tpl_instance.add_raw_template(name, content) {
            Ok(_) => {},
            Err(e) => {
                println!("[load_raw_template] Tera::parse error: {:#?}", e);
            }
        }
    }
}


impl TplService for TeraTplService {
    fn render(&self, template_name: &str, data: &HashMap<String, String>) -> String {
        let mut context = Context::new();
        for (key, val) in data.iter() {
            context.insert(key, val)
        }
        match self.tpl_instance.render(template_name, &context) {
            Ok(v) => v,
            Err(e) => {
                println!("{:#?}", e);
                String::from("")
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn test_tpl_engine() {
        let mut context = Context::new();
        context.insert("content", "hello world!");
        let current_dir = env::current_dir();
        println!(
            "current_dir: {:?}:",
            current_dir
        );

        let tera = match Tera::parse("./src/tpls/rust/hello/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        let data = match tera.render("hello.tpl", &context) {
            Ok(v) => v,
            Err(e) => {
                println!("{:#?}", e);
                String::from("")
            }
        };
        println!("{:?}", data);
    }
}