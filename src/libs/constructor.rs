use crate::libs::basic::{ElementRender, Constructor, Result};


#[derive(Default)]
pub struct EleConstructorV1 {
    element_render: Vec<Box<dyn ElementRender>>,
}

impl EleConstructorV1 {
    pub fn insert(&mut self, render: Box<dyn ElementRender>) {
        self.element_render.push(render);
    }
}

impl Constructor for EleConstructorV1 {
    fn build(&self) -> Result<()> {
        for ele in self.element_render.iter() {
            ele.render()?;
        }
        Ok(())
    }
}