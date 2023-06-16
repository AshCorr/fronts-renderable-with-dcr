use std::fs::{self};

use serde_json::Value;

const DCR_CHECK: &str = "All rights reserved. (modern)";
const FRONTEND_CHECK: &str = "class=\"facia-page\"";

fn main() {
    let contents = fs::read_to_string("config.json").unwrap();

    let config: Value = serde_json::from_str(contents.as_str()).unwrap();
    let config = config.as_object().unwrap().get("fronts").unwrap().as_object().unwrap();
    let fronts = config.keys();

    let mut supported_by_dcr: usize = 0;
    let mut not_supported_by_dcr: usize = 0;
    

    for front in fronts {
        let front = format!("https://www.theguardian.com/{}", front);
        println!("Checking {}", front);
        let page = reqwest::blocking::get(&front).unwrap().text().unwrap();

        if page.contains(DCR_CHECK) {
            println!("{} supported by DCR", front);
            supported_by_dcr += 1;
        } else if page.contains(FRONTEND_CHECK) {
            println!("{} not supported by DCR", front);
            not_supported_by_dcr += 1;
        } else {
            println!("{} not a front", front);
        }
    } 

    
    println!("{} supported by DCR, {} not supported by DCR", supported_by_dcr, not_supported_by_dcr);
}
