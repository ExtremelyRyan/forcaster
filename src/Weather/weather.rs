pub fn get_coordinates_from_zipcode(
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
            city,
            current_temp,
            max_temp,
            min_temp,
            sunrise,
            sunset,
        }
    }
}

pub fn get_weather() -> Result<(), anyhow::Error> {
    let api_key = forcaster::get_env_v2("OPEN_WEATHER".to_string()).unwrap();

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

    forcaster::output_to_json(&body, "json/weather.json".to_string())?;

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
