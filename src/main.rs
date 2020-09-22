mod country;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};

async fn calculate(info: web::Path<String>) -> impl Responder {
    let country = info.to_owned();
    let covid_data = get_covid_data().await.unwrap();
    let country_data = get_country_data(&country).await.unwrap();

    let number_of_days = covid_data[&country].iter().count();
    let number_of_infections = covid_data[&country][number_of_days - 1].confirmed
        - covid_data[&country][number_of_days - 14].confirmed;
    let infections_per_14_days =
        number_of_infections as f32 * 100000.0 / country_data[0].population as f32;
    let mut result: String = "Total number of infections in the last 14 days ".to_owned();
    result.push_str(&infections_per_14_days.to_string());
    HttpResponse::Ok().body(result)
}

async fn get_list_of_infections_per_14_days(
    info: web::Path<String>,
) -> Result<web::Json<CountryInfectionsData>> {
    let country = info.to_owned();
    let covid_data = get_covid_data().await.unwrap();
    let country_data = get_country_data(&country).await.unwrap();
    let mut country_infection_data = CountryInfectionsData {
        infections_per_14_days: Vec::new(),
    };

    let number_of_days = covid_data[&country].iter().count();
    for i in 14..number_of_days {
        let number_of_infections =
            covid_data[&country][i - 1].confirmed - covid_data[&country][i - 14].confirmed;
        let infections_per_14_days =
            number_of_infections as f32 * 100000.0 / country_data[0].population as f32;
        country_infection_data
            .infections_per_14_days
            .push(infections_per_14_days);
    }
    Ok(web::Json(country_infection_data))
}

async fn list_countries() -> Result<web::Json<CountryList>> {
    let covid_data = get_covid_data().await.unwrap();
    let mut country_list = CountryList {
        countries: Vec::new(),
    };
    for (country, val) in covid_data.iter() {
        country_list.countries.push(country.to_string());
    }

    Ok(web::Json(country_list))
}

async fn welcome() -> Result<String> {
    Ok(format!("Welcome"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::new().supports_credentials().finish())
            .route("/", web::get().to(welcome))
            .route("/listcountries", web::get().to(list_countries))
            .route("/calculate/{country}", web::get().to(calculate))
            .route(
                "/infectionsperday/{country}",
                web::get().to(get_list_of_infections_per_14_days),
            )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
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

#[derive(Deserialize)]
struct DayData {
    confirmed: i32,
}

#[derive(Serialize)]
struct CountryInfectionsData {
    infections_per_14_days: Vec<f32>,
}

#[derive(Serialize)]
struct CountryList {
    countries: Vec<String>,
}
