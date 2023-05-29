use dotenv::dotenv;
use std::env; 

fn main() ->  Result<(), Box<dyn std::error::Error>>{ 

    let api_key = match get_env("API_KEY") {
        None => panic!("failed to find API_KEY"),
        Some(str) => str,
    };
    // println!("{:?}", api_key );

    let site = format!("http://api.weatherstack.com/current\
    ?access_key={api_key}\
    &query=75078\
    &forecast_days=1\
    &hourly=1");

    println!("{}\n\n", site);


    let body = reqwest::blocking::get(site)?;
    println!("body = {:?}", body);

    Ok(())
}

fn get_env(key: &str) -> Option<String> {
    dotenv().ok(); 
    let mut api_key = None;
    if env::var(key).is_ok() {
        api_key = Some(env::var(key).unwrap());
    } 
    api_key
}

#[test]
fn get_env_test() {
    let test_key = match get_env("TEST") {
        None => panic!("failed to find API_KEY"),
        Some(str) => str,
    };
    assert_eq!(test_key, "TEST123");
}