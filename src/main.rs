
mod mister_scraper;

#[tokio::main]
async fn main() {

    let url1 = String::from("https://aidanb446.github.io");

    let scraper = mister_scraper::Scraper{url : url1}; 
    
    let doc = scraper.scrape().await.unwrap();

    let text = scraper.get_element(String::from("header"), doc);
   
    println!("{}", text);


}

