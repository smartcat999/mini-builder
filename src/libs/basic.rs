use std::collections::HashMap;
use std::error::Error;
use std::result;

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub trait Constructor {
    fn build(&self) -> Result<()>;
}

pub trait ElementRender {
    fn render(&self, ) -> Result<()>;
}

pub trait TplService: Sync + Send {
    fn render(&self, template_name: &str, data: &HashMap<String, String>) -> String;
}
