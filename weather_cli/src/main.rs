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
struct DetailPrecipation {
    precipitation_amount: f32,
}

impl Default for DetailPrecipation {
    fn default() -> DetailPrecipation {
        DetailPrecipation {
            precipitation_amount: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct InstantObject {
    details: Detail,
}

#[derive(Debug, Deserialize, Serialize)]
struct Next1HoursObject {
    details: DetailPrecipation,
}

impl Default for Next1HoursObject {
    fn default() -> Next1HoursObject {
        let detpre: DetailPrecipation = DetailPrecipation {
            ..Default::default()
        };
        Next1HoursObject { details: detpre }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct DataObject {
    instant: InstantObject,
    next_1_hours: Option<Next1HoursObject>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimeseriesObject {
    data: DataObject,
}

#[derive(Debug, Deserialize, Serialize)]
struct Properties {
    timeseries: Vec<TimeseriesObject>,
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
    let req_str = format!(
        "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={}&lon={}",
        location.lat, location.lon
    );
    let response = match reqwest::blocking::get(&req_str) {
        Ok(response) => response,
        Err(error) => panic!("Failed to fetch weather data from YR: {:?}", error),
    };

    let obj: YrResponse = match response.json() {
        Ok(prop) => prop,
        Err(error) => panic!("Failed to parse json: {:?}", error),
    };
    let mut temperatures: f32 = 0.0;
    let mut rain: f32 = 0.0;
    for hour in 0..hours {
        temperatures = temperatures
            + obj.properties.timeseries[hour]
                .data
                .instant
                .details
                .air_temperature;
        if let Some(x) = obj.properties.timeseries[hour].data.next_1_hours.as_ref() {
            rain += x.details.precipitation_amount;
        }
    }
    let avg = temperatures / hours as f32;
    println!(
        "Average temperature for the next {} hour(s) is going to be {}",
        hours, avg
    );
    println!("Total precipation: {}", rain);
}
