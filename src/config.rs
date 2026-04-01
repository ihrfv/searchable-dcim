use std::env::args;

pub struct Config {
    dcim_path: String,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let args: Vec<String> = args().collect();
        if args.len() != 2 {
            return Err("The command line arguments should contain video folder");
        }

        let dcim_path = args[1].clone();
        Ok(Config { dcim_path })
    }

    pub fn get_dcim_path(&self) -> &str {
        &self.dcim_path
    }
}
