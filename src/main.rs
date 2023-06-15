#![allow(dead_code, non_snake_case)]

mod Astronaut;
mod Weather;
use crate::Astronaut::astronaut::sample_blocked_api_call_astronauts;
use forcaster::get_env_v2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // _ = _sample_blocked_api_call_astronauts().is_ok();
    // _ = weather::get_weather();

    println!("what information would you like?");
    println!("1. Current Astronauts in space");
    println!("2. Weather by zipcode");
    println!("3. Nasa Image of the day");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    eprintln!("read: {}", input);

    let num: isize = input.trim().parse().unwrap();
 

    match num {
        1 => _ = sample_blocked_api_call_astronauts(),
        2 => _ = Weather::weather::get_weather(),
        3 => todo!(),
        _ => panic!("not an option"),
    };

    let key: String = String::from("NASA");

    match get_env_v2(key.clone()) {
        Some(s) => println!("found key value: {}", s),
        None => eprintln!("missing key: {}", key),
    }

    Ok(())
}
