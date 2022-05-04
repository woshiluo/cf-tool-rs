#[derive(Debug)]
pub struct Config {
    pub session_file: std::path::PathBuf,
    pub handle: String,
    pub code_suffix: String,
    pub language_id: u8,
}

impl Config {
    pub fn from_file(file: impl AsRef<std::path::Path>) -> Config {
        use std::io::Read;
        use toml::Value;

        let mut file = std::fs::File::open(file.as_ref()).unwrap();
        let mut raw_config = String::new();

        file.read_to_string(&mut raw_config).unwrap();
        let config = raw_config.parse::<Value>().unwrap();

        Config {
            session_file: config["session_file"].as_str().unwrap().into(),
            handle: config["handle"].as_str().unwrap().into(),
            code_suffix: config["code_suffix"].as_str().unwrap().into(),
            language_id: config["language_id"]
                .as_str()
                .unwrap()
                .parse::<u8>()
                .unwrap(),
        }
    }
}
