use serde::{Deserialize, Serialize};
use std::env;

struct Config {
    hours: usize,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let hours = match args[1].clone().parse() {
            Ok(hours) => hours,
            Err(error) => panic!(
                "Failed to provide a digit for the hours argument: {:?}",
                error
            ),
        };
        Config { hours }
    }
}

struct Location {
    lat: String,
    lon: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LocationResponse {
    loc: String,
}

impl Location {
    fn new() -> Location {
        let response = match reqwest::blocking::get("https://ipinfo.io") {
            Ok(response) => response,
            Err(error) => panic!("Failed to fetch current location: {:?}", error),
        };

        let location_response: LocationResponse = match response.json() {
            Ok(location_response) => location_response,
            Err(error) => panic!("Failed to parse location response: {:?}", error),
        };
        let parsed = location_response.loc.split(",");
        let collection: Vec<&str> = parsed.collect();
        Location {
            lat: String::from(collection[0]),
            lon: String::from(collection[1]),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Detail {
    air_temperature: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Details {
    details: Detail,
}

#[derive(Debug, Deserialize, Serialize)]
struct Instant {
    instant: Details,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    data: Instant,
}

#[derive(Debug, Deserialize, Serialize)]
struct Timeseries {
    timeseries: Vec<Data>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Properties {
    properties: Timeseries,
}

#[derive(Debug, Deserialize, Serialize)]
struct YrResponse {
    properties: Properties,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);
    let hours: usize = config.hours;
    let location: Location = Location::new();
    //println!("Current latitude: {:?}", location.lat);
    //println!("Current longitude: {:?}", location.lon);
    let req_str = format!(
        "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={}&lon={}",
        location.lat, location.lon
    );
    let response = match reqwest::blocking::get(&req_str) {
        Ok(response) => response,
        Err(error) => panic!("Failed to fetch weather data from YR: {:?}", error),
    };
    let obj: Properties = match response.json() {
        Ok(prop) => prop,
        Err(error) => panic!("Failed to parse json: {:?}", error),
    };
    let mut temperatures: f32 = 0.0;
    for hour in 0..hours {
        temperatures = temperatures
            + obj.properties.timeseries[hour]
                .data
                .instant
                .details
                .air_temperature;
    }
    let avg = temperatures / hours as f32;
    println!(
        "Average temperature for the next {} hour(s) is going to be {}",
        hours, avg
    );
}
