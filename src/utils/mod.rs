use std::fs;
use std::path::Path;
use std::io::Result;

pub fn create_dir(path: &str) -> Result<()> {
    if path.ends_with(".rs") || path.ends_with(".go") ||
        path.ends_with(".toml") || path.ends_with("md") {
        if let Some(parent_dir) = Path::new(path).parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)?;
            }
        }
    } else {
        let path = Path::new(path);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn test_demo() {}
}