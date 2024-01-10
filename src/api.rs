use std::collections::HashMap;

use axum::{extract::Path, http::StatusCode, Json};
use once_cell::sync::Lazy;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

static COUNTRIES: Lazy<Vec<Country>> = Lazy::new(|| {
    let json_str = std::fs::read_to_string("resource/countries-v3.1.json").unwrap();
    serde_json::from_str(&json_str).unwrap()
});

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NativeName {
    pub common: String,
    pub official: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Name {
    pub common: String,
    pub official: String,
    pub nativeName: HashMap<String, NativeName>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Currency {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Idd {
    pub root: String,
    pub suffixes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Car {
    pub signs: Vec<String>,
    pub side: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapitalInformation {
    pub latlng: Option<[f64; 2]>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flag {
    pub png: String,
    pub svg: String,
    pub alt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    pub name: Name,
    pub tld: Vec<String>,
    pub cca2: String,
    pub ccn3: String,
    pub cca3: String,
    pub cioc: String,
    pub independent: Option<bool>,
    pub status: String,
    pub unMember: bool,
    pub currencies: HashMap<String, Currency>,
    pub idd: Idd,
    pub capital: Vec<String>,
    pub altSpellings: Vec<String>,
    pub region: String,
    pub subregion: String,
    pub languages: HashMap<String, String>,
    pub translations: HashMap<String, NativeName>,
    pub latlng: [f64; 2],
    pub landlocked: bool,
    pub borders: Vec<String>,
    pub area: f64,
    pub demonyms: HashMap<String, HashMap<String, String>>,
    pub flag: String,
    pub maps: HashMap<String, String>,
    pub population: i64,
    pub gini: HashMap<String, f64>,
    pub fifa: String,
    pub car: Car,
    pub timezones: Vec<String>,
    pub continents: Vec<String>,

    pub flags: Flag,
    pub coatOfArms: Flag,
    pub startOfWeek: String,
    pub capitalInfo: CapitalInformation,
    pub postalCode: HashMap<String, Option<String>>,
}

pub async fn all() -> Json<Vec<Country>> {
    Json(COUNTRIES.clone())
}


pub async fn name(Path(name): Path<String>) -> (StatusCode, Json<Vec<Country>>) {
    let pattern = match RegexBuilder::new(&name).case_insensitive(true).build() {
        Ok(pattern) => pattern,
        Err(e) => {
            log::info!("User request produced: {:?}", e);
            // TODO: send the error message to the user
            return (StatusCode::BAD_REQUEST, Json(Vec::new()));
        }
    };
    let result = COUNTRIES
        .iter()
        .filter(|country| {
            pattern.is_match(&country.name.common) || pattern.is_match(&country.name.official)
        })
        .map(|coutry| coutry.clone())
        .collect();
    (StatusCode::OK, Json(result))
}


pub async fn capital(Path(name): Path<String>) -> (StatusCode, Json<Vec<Country>>) {
    let pattern = match RegexBuilder::new(&name).case_insensitive(true).build() {
        Ok(pattern) => pattern,
        Err(e) => {
            log::info!("User request produced: {:?}", e);
            // TODO: send the error message to the user
            return (StatusCode::BAD_REQUEST, Json(Vec::new()));
        }
    };
    let result = COUNTRIES
        .iter()
        .filter(|country| {
            country
                .capital
                .iter()
                .any(|capital| pattern.is_match(&capital))
        })
        .map(|coutry| coutry.clone())
        .collect();
    (StatusCode::OK, Json(result))
}


pub async fn language(Path(language): Path<String>) -> Json<Vec<Country>> {
    let lowercase_language = language.to_lowercase();
    let result = COUNTRIES
        .iter()
        .filter(|country| {
            country.languages.iter().any(|(abbreviation, full_name)| {
                abbreviation.to_lowercase() == lowercase_language
                    || full_name.to_lowercase() == lowercase_language
            })
        })
        .map(|coutry| coutry.clone())
        .collect();
    Json(result)
}


pub async fn currency(Path(currency): Path<String>) -> (StatusCode, Json<Vec<Country>>) {
    let pattern = match RegexBuilder::new(&currency).case_insensitive(true).build() {
        Ok(pattern) => pattern,
        Err(e) => {
            log::info!("User request produced: {:?}", e);
            // TODO: send the error message to the user
            return (StatusCode::BAD_REQUEST, Json(Vec::new()));
        }
    };
    let result = COUNTRIES
        .iter()
        .filter(|country| {
            country.currencies.iter().any(|(abbreviation, currency)| {
                pattern.is_match(&abbreviation) || pattern.is_match(&currency.name)
            })
        })
        .map(|coutry| coutry.clone())
        .collect();
    (StatusCode::OK, Json(result))
}


pub async fn callingcode(Path(calling_code): Path<String>) -> Json<Vec<Country>> {
    let result = COUNTRIES
        .iter()
        .filter(|country| {
            if country.idd.root.is_empty() {
                return false;
            }
            country.idd.suffixes.iter().any(|suffix| {
                let curr_code = format!("{}{}", country.idd.root, suffix);
                curr_code[1..] == calling_code
            })
        })
        .map(|coutry| coutry.clone())
        .collect();
    Json(result)
}
