extern crate reqwest;
extern crate scraper;
extern crate csv;

use scraper::{Html,Selector};

fn main(){

    println!("welcome to the world");
    scrape_team_data("https://www.worldometers.info/coronavirus/#countries");

}
fn scrape_team_data(url:&str){
    let mut req=reqwest::get(url).unwrap();
    assert!(req.status().is_success());
    let doc_body = Html::parse_document(&req.text().unwrap());
    let team = Selector::parse(".even:nth-child(8) td:nth-child(7) , .even:nth-child(8) td:nth-child(5) , .even:nth-child(8) .sorting_1 , .even:nth-child(8) td:nth-child(2) , .row_continent+ .odd .sorting_1 , .row_continent+ .odd td:nth-child(5) , .row_continent+ .odd td:nth-child(7) , .row_continent+ .odd td:nth-child(2) , .sorting:nth-child(7) , .sorting:nth-child(5) , .sorting_desc , .sorting_disabled+ .sorting").unwrap();

    for team in doc_body.select(&team){
        let teams = team.text().collect::<Vec<_>>();
        println!("{}",teams[0]);

    } 
        
    

}