#![allow(dead_code, non_snake_case)] 
mod Astronaut;
mod Weather;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // _ = _sample_blocked_api_call_astronauts().is_ok();
    _ = Weather::get_weather();

    Ok(())
}


