pub struct Config {
    dcim_path: String,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let mut args = std::env::args();
        args.next();

        let dcim_path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get a dcim folder path"),
        };

        Ok(Config { dcim_path })
    }

    pub fn get_dcim_path(&self) -> &str {
        &self.dcim_path
    }
}
