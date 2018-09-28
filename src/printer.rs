use std::collections::HashMap;

use colored::*;

use mensa::{Day, Counter};


pub struct Printer {
    counter_colors: HashMap<String, Color>
}

impl Printer {
    pub fn new() -> Printer {
        let mut counter_colors: HashMap<String, Color> = HashMap::new();
        counter_colors.insert(String::from("komplett"), Color::Red);
        counter_colors.insert(String::from("vegetarisch"), Color::Blue);
        counter_colors.insert(String::from("freeflow"), Color::Yellow);
        counter_colors.insert(String::from("mensacafe"), Color::Green);
        //counter_colors.insert(String::from("mensacafe_abends"), Color::Purple);

        Printer {
            counter_colors
        }
    }

    fn get_color(&self, counter: &Counter) -> Color {
        self.counter_colors.get(&counter.id).unwrap_or(&Color::White).clone()
    }

    pub fn print_day(&self, day: &Day) {
        for counter in day.counters.iter() {
            self.print_counter(counter);
        }
    }

    pub fn print_counter(&self, counter: &Counter) {
        let color = self.get_color(counter);
        let bar = "#".color(color);

        println!("{} {} - {}", bar, counter.display_name.color(color), counter.description.color(color));
        for meal in counter.meals.iter() {
            println!("{} * {}", bar, meal.name.bold());
            if !meal.components.is_empty() {
                let components: Vec<String> = meal.components.iter().map(|c| c.name.clone()).collect();
                println!("{}    {}", bar, components.join(", "));
            }
            if let Some(prices) = &meal.prices {
                println!("{}    Students: {} €, Employees: {} €, Guests: {} €", bar, prices.students(), prices.employees(), prices.guests());
            }
        }
        println!();
    }
}


