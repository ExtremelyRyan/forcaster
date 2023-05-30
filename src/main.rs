use dotenv::dotenv;
use std::env; 
use reqwest;


 #[tokio::main]
 async fn main() ->  Result<(), Box<dyn std::error::Error>>{ 
// fn main() ->  Result<(), Box<dyn std::error::Error>>{ 
    const LAT: f64 = 33.236228;
    const LON: f64 = -96.80111;


    let api_key = match get_env("API_KEY") {
        None => panic!("failed to find API_KEY"),
        Some(str) => str,
    };
    // println!("{:?}", api_key );

    let _weatherstack = format!("http://api.weatherstack.com/current\
    ?access_key={api_key}\
    &query=75078\
    &forecast_days=1\
    &hourly=1");
 
    let openweather = format!("http://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}", LAT, LON, api_key);

    println!("{}\n", &openweather);

    let body = reqwest::get(openweather).await?.bytes().await?;
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