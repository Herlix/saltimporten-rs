use chrono::{Date, Datelike, Local};
use std::error::Error;

// Get weekly lunch from web-scraping
mod scraped;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    scraped::menu_scraped().await?.print();
    Ok(())
}

#[derive(Debug)]
pub struct Lunch {
    veg: String,
    norm: Vec<LunchDay>,
}

#[derive(Debug)]
pub struct LunchDay {
    date: Date<Local>,
    desc: String,
}

impl Lunch {
    fn print(&self) {
        println!("Vegetariskt       {}", self.veg);

        for lun in &self.norm {
            println!(
                "{} - {}    {}",
                lun.date.format("%d/%m").to_string(),
                lun.in_swe(),
                lun.desc
            );
        }
    }
}

impl LunchDay {
    fn in_swe(&self) -> &str {
        return match self.date.weekday().to_string().as_str() {
            "Mon" => "Måndag",
            "Tue" => "Tisdag",
            "Wed" => "Onsdag",
            "Thu" => "Torsdag",
            "Fri" => "Fredag",
            _ => "",
        };
    }
}
