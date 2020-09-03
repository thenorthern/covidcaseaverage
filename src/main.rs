mod country;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    validate_arguments(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let country = args[1].clone();
    println!("Your selected country is {}", country);

    let covid_data = get_covid_data().await?;
    let country_data = get_country_data(&country).await?;

    println!(
        "Its population of {} is {}",
        country, country_data[0].population
    );

    let number_of_days = covid_data[&country].iter().count();
    let number_of_infections = covid_data[&country][number_of_days - 1].confirmed
        - covid_data[&country][number_of_days - 14].confirmed;
    let infections_per_14_days =
        number_of_infections as f32 * 100000.0 / country_data[0].population as f32;

    print_result(number_of_infections, infections_per_14_days, country);

    Ok(())
}

fn print_result(number_of_infections: i32, infections_per_14_days: f32, country: String) {
    println!(
        "Total number of infections in the last 14 days {}",
        number_of_infections
    );
    println!(
        "Number of infections per 100000 people in the last 14 days {}",
        infections_per_14_days
    );

    if infections_per_14_days < 8.0 {
        println!("NICE! You can travel to {} without quarantine", country);
    } else {
        println!(
            "CRAP! You cannot travel to {}. If you do you need 14 days of compulsory quarantine when you come back",
            country
        );
    }
}

async fn get_country_data(
    country: &str,
) -> Result<Vec<country::Country>, Box<dyn std::error::Error>> {
    let country_url = "https://restcountries.eu/rest/v2/name/".to_string() + country;
    let country_data = reqwest::get(&country_url)
        .await?
        .json::<Vec<country::Country>>()
        .await?;
    return Ok(country_data);
}

async fn get_covid_data() -> Result<HashMap<String, Vec<DayData>>, Box<dyn std::error::Error>> {
    let covid_data = reqwest::get("https://pomber.github.io/covid19/timeseries.json")
        .await?
        .json::<HashMap<String, Vec<DayData>>>()
        .await?;
    return Ok(covid_data);
}

fn validate_arguments(args: &[String]) -> Result<(), &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    Ok(())
}

#[derive(Deserialize)]
struct DayData {
    confirmed: i32,
}
