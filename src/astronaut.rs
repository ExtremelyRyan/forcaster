use reqwest::blocking::get;

#[derive(Debug)]
struct Astronaut {
    name: String,
    craft: String,
}
impl Astronaut {
    fn new(name: String, craft: String) -> Astronaut {
        Self { name, craft }
    }
    fn print(self) {
        println!("astronaut {} on craft {}", self.name, self.craft);
    }
}

pub fn _sample_blocked_api_call_astronauts() -> Result<(), anyhow::Error> {
    let mut astro: Vec<Astronaut> = Vec::new();

    let url = "http://api.open-notify.org/astros.json";
    let resp: serde_json::Value = get(url)?.json()?;

    forcaster::output_to_json(&resp, "json/Astronauts.json".to_string())?;

    let n = &resp["number"];
    let n = n.as_u64().unwrap_or_default() as usize;
    println!("number of astronauts in space: {n}");

    for x in 0..n {
        let name = resp["people"][x]["name"].to_string();
        let craft = resp["people"][x]["craft"].to_string();
        astro.push(Astronaut::new(name, craft) );
    }

    for a in astro {
        a.print();
    };

    Ok(())
}
