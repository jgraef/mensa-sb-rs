use std::collections::HashMap;
use chrono::{DateTime, Utc};

use reqwest;


/*pub enum Language {
    De,
    En
}

impl Into<String> for Language {
    fn into(self) -> String {
        match self {
            Language::De => String::from("de"),
            Language::En => String::from("en"),
        }
    }
}

impl From<String> for Language {
    fn from(s: String) -> Self {
        match s.as_str() {
            "de" => Language::De,
            "en" => Language::En,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}*/



/*pub enum Place {
    Hom,
    HtwGtb,
    Mensagarten,
    MusikSb,
    Sb,
    HtwCrb,
    HtwCas,
    Other(String)
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, self.into())
    }
}

impl Into<&str> for Place {
    fn into(self) -> &str {
        match self {
            Place::Hom => "hom",
            Place::HtwGtb => "htwgtb",
            Place::Mensagarten => "mensagarten",
            Place::MusikSb => "musiksb",
            Place::Sb => "sb",
            Place::HtwCrb => "htwcrb",
            Place::HtwCas => "htwcas",
            Place::Other(s) => &s
        }
    }
}

impl From<&str> for Place {
    fn from(s: &str) -> Self {
        match s {
            "hom" => Place::Hom,
            "htwgtb" => Place::HtwGtb,
            "mensagarten" => Place::Mensagarten,
            "musiksb" => Place::MusikSb,
            "sb" => Place::Sb,
            "htwcrb" => Place::HtwCrb,
            "htwcas" => Place::HtwCas,
            _ => Place::Other(s.clone())
        }
    }
}*/


pub struct Config {
    pub api_key: String,
    pub api_version: u32,
    pub app_version: u32,
    pub language: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            api_key: "TFtD8CTykAXXwrW4WBU4".into(), // API key from mensaar.de
            api_version: 1,
            app_version: 1,
            language: String::from("de")
        }
    }
}


pub struct Client {
    base_url: String,
    http: reqwest::Client
}

impl Client {
    pub fn new(config: Config) -> Client {
        let http = reqwest::Client::new();
        let base_url = format!("https://mensaar.de/api/{}/{}/{}/{}", config.api_version, config.api_key, config.app_version, config.language);

        Client {
            base_url,
            http
        }
    }

    pub fn get_base_data(&self) -> Result<BaseData, reqwest::Error> {
        Ok(self.http.get(format!("{}/getBaseData", self.base_url).as_str()).send()?.json()?)
    }

    pub fn get_menu(&self, location: &String) -> Result<Menu, reqwest::Error> {
        Ok(self.http.get(format!("{}/getMenu/{}", self.base_url, location).as_str()).send()?.json()?)
    }
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BaseData {
    pub notices: HashMap<String, Notice>,
    pub locations: HashMap<String, Location>,
    pub known_meals: HashMap<String, KnownMeal>,
    pub price_tiers: HashMap<String, PriceTier>,
    pub global_message: Option<GlobalMessage>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Notice {
    pub display_name: String,
    pub is_allergen: bool,
    pub is_negated: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub display_name: String,
    pub description: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KnownMeal {
    pub display_name: String,
    pub last_offered: Option<String> // TODO: should be a Date
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceTier {
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GlobalMessage {
    pub title: String,
    pub text: String
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub days: Vec<Day>
}

impl Menu {
    pub fn today(&self) -> Option<&Day> {
        let date = Utc::now().date().and_hms(0, 0, 0);
        for day in self.days.iter() {
            if day.date == date {
                return Some(day);
            }
        }
        None
    }
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    pub date: DateTime<Utc>,
    pub counters: Vec<Counter>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub opening_hours: Option<OpeningHours>,
    pub color: Option<Color>,
    pub meals: Vec<Meal>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpeningHours {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meal {
    pub known_meal_id: Option<String>,
    pub name: String,
    pub notices: Vec<String>,
    pub components: Vec<Component>,
    pub prices: Option<Prices>,
    pub pricing_notice: Option<String>,
    pub category: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub name: String,
    pub notices: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    pub g: String,
    pub s: String,
    pub m: String,
}

pub enum PriceCategory {
    Guest,
    Student,
    Employee
}

impl Prices {
    pub fn guests(&self) -> String {
        self.g.clone()
    }

    pub fn students(&self) -> String {
        self.s.clone()
    }

    pub fn employees(&self) -> String {
        self.m.clone()
    }

    pub fn for_category(&self, cat: PriceCategory) -> String {
        match cat {
            PriceCategory::Guest => self.guests(),
            PriceCategory::Student => self.students(),
            PriceCategory::Employee => self.employees()
        }
    }
}
