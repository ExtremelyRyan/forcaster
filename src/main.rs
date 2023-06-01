use dotenv::dotenv;
use serde::Deserialize;
use std::error::Error;
use std::{env, vec};
use reqwest::blocking::get;
use std::fs::File;
use std::io::prelude::*;



//  #[tokio::main]
//  async fn main() ->  Result<(), Box<dyn std::error::Error>>{ 
fn main() -> Result<(), Box<dyn std::error::Error>> { 
    _ = _sample_blocked_api_call_astronauts().is_ok();  
    _ = get_weather();

    Ok(())
}

fn get_weather() -> Result<(), Box<dyn std::error::Error>>{
    const LAT: f64 = 33.236228;
    const LON: f64 = -96.80111;


    let api_key = match get_env("API_KEY".to_string()) {
        None => panic!("failed to find API_KEY"),
        Some(str) => str,
    };

    // let _weatherstack = format!("http://api.weatherstack.com/current\
    // ?access_key={api_key}\
    // &query=75078\
    // &forecast_days=1\
    // &hourly=1");
 
    let openweather = format!("http://api.openweathermap.org/data/2.5/weather?\
    lat={}\
    &lon={}\
    &appid={}\
    &units=imperial", 
    LAT, LON, api_key);

    // println!("{}\n", &openweather);

    let body: serde_json::Value = reqwest::blocking::get(openweather)?.json()?;

    let current_temp: String = body["main"]["temp"].to_string();
    let max_temp: String = body["main"]["temp_max"].to_string();
    let min_temp: String = body["main"]["temp_min"].to_string();

    println!("cur temperature: {}", current_temp);
    println!("max temperature: {}", max_temp);
    println!("min temperature: {}", min_temp);

    Ok(())
}

#[derive(Debug)]
struct Astronaut {
    name: String,
    craft: String,
}

impl Astronaut {
    fn new(name: String, craft: String) -> Self {
        return Astronaut { name, craft };
    }
}

fn _sample_blocked_api_call_astronauts() -> Result<(), anyhow::Error>{ 

    let mut astro: Vec<Astronaut> = Vec::new();

    let url = "http://api.open-notify.org/astros.json";
    let resp: serde_json::Value = get(url)?.json()?;

    let mut file = File::create("response.json").unwrap();
    file.write_all(resp.to_string().as_bytes())?;

    let n = &resp["number"];
    let n = n.as_u64().unwrap_or_default() as usize;
    println!("number of astronauts in space: {n}");
 

    for x in 0..n {
        let name = resp["people"][x]["name"].to_string();
        let craft = resp["people"][x]["craft"].to_string(); 
        astro.push(Astronaut { name, craft});
    }

    for a in astro {
        println!("astronaut {} is on {}", a.name, a.craft);
    };
    Ok(())
}

fn get_env(key: String) -> Option<String> {
    dotenv().ok();
    let api_key = match env::var_os(&key) {
        Some(v) => v.into_string().unwrap(),
        None => panic!("{} is not set", key)
    };
    return Some(api_key);
}

#[test]
fn get_env_test() {
    let test_key = match get_env("TEST".to_string()) {
        None => panic!("failed to find API_KEY"),
        Some(str) => str,
    };
    assert_eq!(test_key, "TEST123");
}
 