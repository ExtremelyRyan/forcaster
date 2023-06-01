use dotenv::dotenv;
use std::env;
use reqwest::blocking::get;


//  #[tokio::main]
//  async fn main() ->  Result<(), Box<dyn std::error::Error>>{ 
fn main() -> Result<(), Box<dyn std::error::Error>> { 
    _ = _sample_blocked_api_call_astronauts().is_ok();  

    Ok(())
}



fn get_weather() -> Result<(), Box<dyn std::error::Error>>{
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

    let body: serde_json::Value = reqwest::blocking::get(openweather)?.json()?;
    println!("body = {:?}", body);
    Ok(())
}

fn _sample_blocked_api_call_astronauts() -> Result<(), reqwest::Error>{
    
    let url = "http://api.open-notify.org/astros.json";
    let resp: serde_json::Value = get(url)?.json()?;

    let n = &resp["number"];
    let n = n.as_u64().unwrap_or_default() as usize;
    println!("number of astronauts: {n}");
 

    for x in 0..n {
        let people = &resp["people"][x]["name"];
        let people = people.to_string().replace('"', "");
        println!("{}. {}",x+1, people)
    }; 
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
 