use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use crate::libs::basic::{Result, ElementRender, TplService};
use crate::utils;


// DIR 生成
#[derive(Debug)]
pub struct EleDirRender {
    path: String,
}

impl EleDirRender {
    pub fn new(path: String) -> Self {
        EleDirRender {
            path
        }
    }
}

impl ElementRender for EleDirRender {
    fn render(&self) -> Result<()> {
        fs::create_dir(Path::new(&self.path))?;
        Ok(())
    }
}

pub struct EleRustCodeRender {
    path: String,
    data: HashMap<String, String>,
    template_name: String,
    tpl_service: Rc<dyn TplService>,
}

impl EleRustCodeRender {
    pub fn new(path: String, data: HashMap<String, String>, template_name: String, tpl_service: Rc<dyn TplService>) -> Self {
        EleRustCodeRender {
            path,
            data,
            template_name,
            tpl_service,
        }
    }
}

impl ElementRender for EleRustCodeRender {
    fn render(&self) -> Result<()> {
        utils::create_dir(&self.path)?;
        let content = self.tpl_service.render(&self.template_name, &self.data);

        fs::write(&self.path, content)?;
        Ok(())
    }
}

pub struct EleRustModRender {
    path: String,
    data: HashMap<String, String>,
    template_name: String,
    tpl_service: Rc<dyn TplService>,
}

impl EleRustModRender {
    pub fn new(path: String, data: HashMap<String, String>, template_name: String, tpl_service: Rc<dyn TplService>) -> Self {
        EleRustModRender {
            path,
            data,
            template_name,
            tpl_service,
        }
    }
}

impl ElementRender for EleRustModRender {
    fn render(&self) -> Result<()> {
        utils::create_dir(&self.path)?;

        let content = self.tpl_service.render(&self.template_name, &self.data);

        fs::write(self.path.to_string() + "/mod.rs", content)?;
        Ok(())
    }
}


pub struct EleRustTomlRender {
    path: String,
    data: HashMap<String, String>,
    template_name: String,
    tpl_service: Rc<dyn TplService>,
}

impl EleRustTomlRender {
    pub fn new(path: String, data: HashMap<String, String>, template_name: String, tpl_service: Rc<dyn TplService>) -> Self {
        EleRustTomlRender {
            path,
            data,
            template_name,
            tpl_service,
        }
    }
}

impl ElementRender for EleRustTomlRender {
    fn render(&self) -> Result<()> {
        utils::create_dir(&self.path)?;
        let content = self.tpl_service.render(&self.template_name, &self.data);

        fs::write(&self.path, content)?;
        Ok(())
    }
}

pub struct EleRustMakefileRender {
    path: String,
    data: HashMap<String, String>,
    template_name: String,
    tpl_service: Rc<dyn TplService>,
}

impl EleRustMakefileRender {
    pub fn new(path: String, data: HashMap<String, String>, template_name: String, tpl_service: Rc<dyn TplService>) -> Self {
        EleRustMakefileRender {
            path,
            data,
            template_name,
            tpl_service,
        }
    }
}

impl ElementRender for EleRustMakefileRender {
    fn render(&self) -> Result<()> {
        utils::create_dir(&self.path)?;
        let content = self.tpl_service.render(&self.template_name, &self.data);

        fs::write(&self.path, content)?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;
    use crate::libs::tpl::TeraTplService;

    #[test]
    fn test_element_dir_render() {
        let el = EleDirRender::new(String::from("./tmp"));
        match el.render() {
            Ok(()) => {}
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }

    #[test]
    fn test_element_rust_code_render() {
        let mut data: HashMap<String, String> = HashMap::new();
        data.insert(String::from("name"), String::from("excel-tools"));
        data.insert(String::from("version"), String::from("v1.0"));
        data.insert(String::from("author"), String::from("smartcat"));
        let mut tpl = TeraTplService::default();
        tpl.load_template(String::from("./src/tpls/rust/command/*"));
        let tpl = Rc::new(tpl);
        let el = EleRustCodeRender::new(
            String::from("./tmp/main.rs"),
            data,
            String::from("command.tpl"),
            tpl,
        );
        match el.render() {
            Ok(()) => {}
            Err(e) => {
                println!("render error: {:#?}", e);
            }
        };
    }
}