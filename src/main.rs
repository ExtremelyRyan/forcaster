#![allow(dead_code)]

use anyhow;
use dotenv::dotenv;
use reqwest::blocking::get;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // _ = _sample_blocked_api_call_astronauts().is_ok();
    _ = get_weather();

    Ok(())
}

fn get_coordinates_from_zipcode(
    zip: String,
    api_key: String,
) -> Result<(f64, f64), anyhow::Error> {
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/zip?zip=\
    {zip},\
    US&appid={api_key}"
    );

    let resp: serde_json::Value = reqwest::blocking::get(url)?.json()?;

    let lat: f64 = resp["lat"].to_string().parse().unwrap();
    let lon: f64 = resp["lon"].to_string().parse().unwrap();

    Ok((lat, lon))
}

#[derive(Debug)]
struct Weather {
    city: String,
    current_temp: String,
    max_temp: String,
    min_temp: String,
    sunrise: String,
    sunset: String,
}

impl Weather {
    /// Creates a new [`Weather`].
    fn new(
        city: String,
        current_temp: String,
        max_temp: String,
        min_temp: String,
        sunrise: String,
        sunset: String,
    ) -> Self {
        Self {
            city, current_temp, max_temp,
            min_temp, sunrise, sunset,
        }
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if .
fn get_weather() -> Result<(), anyhow::Error> {
    let api_key = get_env("API_KEY".to_string());

    let (lat, lon) = get_coordinates_from_zipcode(String::from("75078"), api_key.clone())?;
    // println!("{},{}", lat, lon);
    // const LAT: f64 = 33.236228;
    // const LON: f64 = -96.80111;

    // let _weatherstack = format!("http://api.weatherstack.com/current\
    // ?access_key={api_key}\
    // &query=75078\
    // &forecast_days=1\
    // &hourly=1");

    let openweather = format!(
        "http://api.openweathermap.org/data/2.5/weather?\
    lat={}\
    &lon={}\
    &appid={}\
    &units=imperial",
        lat, lon, api_key
    );

    // println!("{}\n", &openweather);

    let body: serde_json::Value = reqwest::blocking::get(openweather)?.json()?;

    output_to_json(&body, "json/weather.json".to_string())?;

    let w: Weather = Weather {
        city: body["name"].to_string().replace('"', ""),
        current_temp: body["main"]["temp"].to_string(),
        max_temp: body["main"]["temp_max"].to_string(),
        min_temp: body["main"]["temp_min"].to_string(),
        sunrise: body["main"]["sunrise"].to_string(),
        sunset: body["main"]["sunset"].to_string(),
    };

    println!("{:?}", w);

    Ok(())
}

#[derive(Debug)]
struct Astronaut {
    name: String,
    craft: String,
}

fn _sample_blocked_api_call_astronauts() -> Result<(), anyhow::Error> {
    let mut astro: Vec<Astronaut> = Vec::new();

    let url = "http://api.open-notify.org/astros.json";
    let resp: serde_json::Value = get(url)?.json()?;

    output_to_json(&resp, "json/Astronauts.json".to_string())?;

    let n = &resp["number"];
    let n = n.as_u64().unwrap_or_default() as usize;
    println!("number of astronauts in space: {n}");

    for x in 0..n {
        let name = resp["people"][x]["name"].to_string();
        let craft = resp["people"][x]["craft"].to_string();
        astro.push(Astronaut { name, craft });
    }

    for a in astro {
        println!("astronaut {} is on {}", a.name, a.craft);
    }
    Ok(())
}

fn output_to_json(resp: &serde_json::Value, fname: String) -> Result<(), anyhow::Error> {
    let mut file = File::create(fname).unwrap();
    file.write_all(resp.to_string().as_bytes())?;
    Ok(())
}

fn get_env(key: String) -> String {
    dotenv().ok();
    let api_key = match env::var_os(&key) {
        Some(v) => v.into_string().unwrap(),
        None => panic!("{} is not set", key),
    };
    return api_key;
}

#[test]
fn get_env_test() {
    let test_key = get_env("TEST".to_string());
    assert_eq!(test_key, "TEST123");
}
