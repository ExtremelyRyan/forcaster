use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn output_to_json(resp: &serde_json::Value, fname: String) -> Result<(), anyhow::Error> {
    let mut file = File::create(fname)?;
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

pub fn get_env_v2(key: String) -> Option<String> {
    dotenv().ok();
    match env::var_os(&key) {
        Some(v) => Some(v.into_string().unwrap()),
        None => {
            eprintln!("{} is not set", key);
            None
        }
    }
}

pub fn query_user_for_api_key() -> Option<String> {
    dotenv().ok();

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_env_test() {
        let test_key = get_env_v2("TEST".to_string()).unwrap();
        assert_eq!(test_key, "TEST123".to_string());
    }

    #[test]
    fn fail_env_test() {
        let test_key = get_env_v2("qwerty".to_string());
        assert_eq!(test_key, None);
    }
}
