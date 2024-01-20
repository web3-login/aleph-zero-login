use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Config {
    pub ext_hostname: String,
    pub frontend_host: String,
    pub key_id: String,
    pub eddsa_pem: Option<String>,
    pub port: u16,
    pub frontend_dir: PathBuf,
}

pub fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_config() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
ext_hostname: https://oidc.web3-login.example.com
frontend_host: https://web3-login.example.com
key_id: changeme
eddsa_pem: assets/do_not_use.pem
port: 8080
frontend_dir: dist/
"#
        )
        .unwrap();

        let config_path = temp_file.path().to_str().unwrap();
        let config = load_config(config_path).unwrap();

        assert_eq!(
            config,
            Config {
                ext_hostname: "https://oidc.web3-login.example.com".to_string(),
                frontend_host: "https://web3-login.example.com".to_string(),
                key_id: "changeme".to_string(),
                eddsa_pem: Some("assets/do_not_use.pem".to_string()),
                port: 8080,
                frontend_dir: "dist/".into(),
            }
        );
    }
}
