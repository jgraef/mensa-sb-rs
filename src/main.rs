extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate clap;
extern crate colored;


#[macro_use]
extern crate serde_derive;

mod mensa;
mod printer;

use clap::{App, Arg};

use printer::Printer;
use mensa::*;


fn print_base_data(client: Client) {
    println!("# Base Data");
    println!();
    println!("```");
    match client.get_base_data() {
        Ok(base_data) => println!("{:#?}", base_data),
        Err(e) => println!("Error: {}", e)
    }
    println!("```");
}

fn print_menu(client: Client) {
    println!("# Menu");
    println!();
    println!("```");
    match client.get_menu(&"sb".to_string()) {
        Ok(menu) => println!("{:#?}", menu),
        Err(e) => println!("Error: {}", e)
    }
    println!("```");
}


fn main() {
    let matches = App::new("mensaar-rs")
        .version("0.1")
        .author("Janosch Gräf <janosch.graef@cispa.saarland>")
        .about("Shows today's meals in your console")
        .arg(Arg::with_name("location")
            .short("l")
            .long("location")
            .value_name("LOCATION")
            .help("Choose your location (default: sb)")
            .takes_value(true))
        .arg(Arg::with_name("language")
            .short("L")
            .long("language")
            .value_name("LANG")
            .help("Select language: en or de. Default: de")
            .takes_value(true))
        .arg(Arg::with_name("tomorrow")
            .short("t")
            .long("tomorrow")
            .help("Show meals for tomorrow"))
        .get_matches();

    let location = matches.value_of("location").unwrap_or("sb").to_string();
    let language = matches.value_of("language").unwrap_or("de").to_string();
    let tomorrow = matches.is_present("tomorrow");

    let config = Config {
        language,
        ..Config::default()
    };
    let client = Client::new(config);

    if location == "?" {
        // print valid locations

        match client.get_base_data() {
            Ok(base_data) => {
                println!("Valid locations are:");
                for (id, location) in base_data.locations.iter() {
                    println!("  * {}: {}", id, location.display_name);
                }
            }
            Err(e) => eprintln!("Could not retrieve base data: {}", e)
        }
    }
    else {
        // print meals

        match client.get_menu(&location) {
            Ok(menu) => {
                let day = if !tomorrow {menu.today()} else {menu.tomorrow()};
                match menu.today() {
                    Some(day) => {
                        let p = Printer::new();
                        p.print_day(day);
                    },
                    None => eprintln!("Menu didn't contain data for today")
                }
            },
            Err(e) => eprintln!("Could not retrieve mensa menu: {}", e)
        }
    }
}
