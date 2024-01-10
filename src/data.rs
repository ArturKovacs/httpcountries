
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

pub static COUNTRIES: Lazy<Vec<Country>> = Lazy::new(|| {
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
    #[serde(rename = "nativeName")]
    pub native_name: HashMap<String, NativeName>,
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
    #[serde(rename = "unMember")]
    pub un_member: bool,
    pub currencies: HashMap<String, Currency>,
    pub idd: Idd,
    pub capital: Vec<String>,
    #[serde(rename = "altSpellings")]
    pub alt_spellings: Vec<String>,
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
    #[serde(rename = "coatOfArms")]
    pub coat_of_arms: Flag,
    #[serde(rename = "startOfWeek")]
    pub start_of_week: String,
    #[serde(rename = "capitalInfo")]
    pub capital_info: CapitalInformation,
    #[serde(rename = "postalCode")]
    pub postal_code: HashMap<String, Option<String>>,
}
