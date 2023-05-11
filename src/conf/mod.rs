use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/tpls/"]
pub struct Asset;


#[cfg(test)]
mod test {
    use crate::conf::Asset;
    use crate::libs::tpl::TeraTplService;

    #[test]
    fn test_load_template() {
        let mut tpl = TeraTplService::default();

        for v in Asset::iter() {
            println!("{:#?}", v);
            let data = v.parse::<String>().unwrap();
            tpl.load_raw_template(&format!("{:#?}", v), &data);
        }
        println!("{:#?}", tpl);
    }
}