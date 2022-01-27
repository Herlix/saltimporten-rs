use chrono::{Date, Datelike, Local, TimeZone};
use scraper::{Html, Selector};
use std::error::Error;

use crate::{Lunch, LunchDay};

const URL: &str = "https://www.saltimporten.com";

pub async fn menu_scraped() -> Result<Lunch, Box<dyn Error>> {
    let resp = reqwest::get(URL).await?.text().await?;
    let fragment = Html::parse_document(&resp);

    let date_selector = Selector::parse("div[class='date']").unwrap();
    let meal_selector = Selector::parse("div[class='meal']").unwrap();

    let mut norm = Vec::new();
    let mut meals = fragment.select(&meal_selector);
    for d in fragment.select(&date_selector) {
        let date = parse_date(d.inner_html().trim());
        let meal = meals.next().unwrap().inner_html();
        norm.push(LunchDay {
            date: date,
            desc: meal,
        });
    }

    let lunch = Lunch {
        veg: meals.next().unwrap().inner_html(),
        norm: norm,
    };

    assert!(lunch.norm.len() == 5);

    Ok(lunch)
}

fn parse_date(input: &str) -> Date<Local> {
    let now = Local::now();
    let year = now.year();
    let split: Vec<u32> = input
        .split("/")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    Local.ymd(year, split[1], split[0])
}
