use std::collections::HashMap;

use axum::{
    extract::{Path, Query, Request},
    http::{StatusCode, Uri},
    Json,
};
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

use crate::data::{Country, COUNTRIES};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paginated {
    #[serde(rename = "totalPageCount")]
    pub total_page_count: usize,

    /// None, if this is the last page
    pub next: Option<String>,
    pub items: Vec<Country>,
}

const ITEMS_PER_PAGE: usize = 20;

fn parse_query(query: &str) -> Result<HashMap<&str, &str>, ()> {
    let mut result = HashMap::new();
    let pairs = query.split("&");
    for pair in pairs {
        let mut key_value = pair.split("=");
        let key = key_value.next().ok_or(())?;
        let value = key_value.next().ok_or(())?;
        result.insert(key, value);
    }
    Ok(result)
}

fn query_to_string(query: &HashMap<&str, &str>) -> String {
    query
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<String>>()
        .join("&")
}

fn parse_maybe_query(query: Option<&str>) -> HashMap<&str, &str> {
    if let Some(query_str) = query {
        match parse_query(query_str) {
            Ok(query) => query,
            Err(()) => {
                log::info!("Failed to parse query string: {:?}", query_str);
                HashMap::new()
            }
        }
    } else {
        HashMap::new()
    }
}

fn paginate(uri: &Uri, items: &[Country]) -> Paginated {
    let page_count = (items.len() as f64 / ITEMS_PER_PAGE as f64).ceil() as usize;
    let mut query = parse_maybe_query(uri.query());
    let page = query
        .get("page")
        .and_then(|page| page.parse::<usize>().ok())
        .unwrap_or(1);
    let next_page_str = format!("{}", page + 1);
    let next = if page + 1 > page_count {
        None
    } else {
        query.insert("page", &next_page_str);
        Some(format!("{}?{}", uri.path(), query_to_string(&query)))
    };
    let page_items = items
        .iter()
        .skip((page - 1) * ITEMS_PER_PAGE)
        .take(ITEMS_PER_PAGE)
        .map(|country| country.clone())
        .collect();
    Paginated {
        total_page_count: page_count,
        next: next,
        items: page_items,
    }
}

pub async fn all(request: Request) -> Json<Paginated> {
    println!("{:?}", request.uri().query());
    Json(paginate(request.uri(), &*COUNTRIES))
}

pub async fn by_cca3(Path(cca3): Path<String>) -> Result<Json<Country>, StatusCode> {
    let country = COUNTRIES
        .iter()
        .filter(|country| country.cca3 == cca3)
        .next();

    if let Some(country) = country {
        Ok(Json(country.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn name(Path(name): Path<String>) -> Result<Json<Vec<Country>>, StatusCode> {
    let pattern = match RegexBuilder::new(&name).case_insensitive(true).build() {
        Ok(pattern) => pattern,
        Err(e) => {
            log::info!("User request produced: {:?}", e);
            // TODO: send the error message to the user
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let result = COUNTRIES
        .iter()
        .filter(|country| {
            pattern.is_match(&country.name.common) || pattern.is_match(&country.name.official)
        })
        .map(|coutry| coutry.clone())
        .collect();
    Ok(Json(result))
}

pub async fn capital(Path(name): Path<String>) -> Result<Json<Vec<Country>>, StatusCode> {
    let pattern = match RegexBuilder::new(&name).case_insensitive(true).build() {
        Ok(pattern) => pattern,
        Err(e) => {
            log::info!("User request produced: {:?}", e);
            // TODO: send the error message to the user
            return Err(StatusCode::BAD_REQUEST);
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
    Ok(Json(result))
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

pub async fn currency(Path(currency): Path<String>) -> Result<Json<Vec<Country>>, StatusCode> {
    let pattern = match RegexBuilder::new(&currency).case_insensitive(true).build() {
        Ok(pattern) => pattern,
        Err(e) => {
            log::info!("User request produced: {:?}", e);
            // TODO: send the error message to the user
            return Err(StatusCode::BAD_REQUEST);
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
    Ok(Json(result))
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
