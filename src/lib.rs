
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn output_to_json(resp: &serde_json::Value, fname: String) -> Result<(), anyhow::Error> {
    let mut file = File::create(fname).unwrap();
    file.write_all(resp.to_string().as_bytes())?;
    Ok(())
}

pub fn get_env(key: String) -> String {
    dotenv().ok();
    match env::var_os(&key) {
        Some(v) => v.into_string().unwrap(),
        None => panic!("{} is not set", key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_env_test() {
        let test_key = get_env("TEST".to_string());
        assert_eq!(test_key, "TEST123");
    }
}