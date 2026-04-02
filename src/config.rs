use std::path::Path;

pub struct Config {
    dcim_path: String,
}

impl Config {
    /// Builds a valid Config from the env args
    ///
    /// # Errors
    ///
    /// 1. If the are not enough provided args
    /// 2. If the first provided arg doesn't point to a directory
    pub fn build() -> Result<Config, &'static str> {
        let mut args = std::env::args();
        args.next();

        let dcim_path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get a dcim folder path"),
        };

        if !Path::new(&dcim_path).is_dir() {
            return Err("The provided path is not a directory");
        }

        Ok(Config { dcim_path })
    }

    pub fn get_dcim_path(&self) -> &str {
        &self.dcim_path
    }
}
