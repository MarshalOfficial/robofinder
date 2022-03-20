use regex::Regex;
use serde::*;
use std::any::Any;
use std::collections::*;
use std::env;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct TimeRecord {
    timeStamp: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("Pass the domain name in the parameter!");
    }

    //let domain = Arc::new(&args[1]);
    let domain = &args[1];
    //&args[1];
    //let arg: &'static str = Box::leak(&args[1]);

    // println!("{:?}", args);

    let base_url = format!("https://web.archive.org/cdx/search/cdx?url={}/robots.txt&output=json&filter=statuscode:200&fl=timestamp,original&collapse=digest","amazon.com"); //domain
    let resp = reqwest::get(base_url).await?.text().await?;

    let json: Vec<TimeRecord> = serde_json::from_str(&resp).expect("JSON was not well-formatted");
    let mut handles = Vec::new();
    let mut result: HashMap<char, usize> = HashMap::new();

    for obj in json {
        if obj.url == "original" {
            continue;
        }

        // println!("timeStamp: {:#?}, Url: {:#?}", obj.timeStamp, obj.url);
        let handle = tokio::spawn(async {
            get_robot_file(&String::from("amazon.com"), obj.timeStamp, obj.url).await;
        });
        handles.push(handle);
        // get_robot_file(domain, obj.timeStamp, obj.url).await?;
    }

    for handle in handles {
        let map = handle.await.unwrap();
    }

    //println!("{:#?}", json);
    Ok(())
}

async fn get_robot_file(
    base_domain: &String,
    timestamp: String,
    url: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = format!("http://web.archive.org/web/{}if_/{}", timestamp, url);
    // println!("second call url: {:#?}", base_url);
    let resp = reqwest::get(base_url).await?.text().await?;

    if resp.is_empty() {
        return Ok(String::from("ok"));
    }

    // let items = resp.split("\n");
    let items: Vec<&str> = resp.trim().split("\n").collect();

    // let _regex = Regex::new(r"(?i)(disallow|allow)(\\s?)\\:(\\s?)(.*)").unwrap();

    for item in items {
        // println!("before check::::: {:#?}", item);
        //if _regex.is_match(item) {
        if item.to_lowercase().contains("disallow:") || item.to_lowercase().contains("allow:") {
            // println!("regex match::::: {:#?}", item);
            let sub_item: Vec<&str> = item.split(": ").collect();
            if sub_item.capacity() <= 1 {
                continue;
            }

            let temp = &sub_item[1]
                .replace("\n", "")
                .replace("*", "")
                .replace("\r", "");

            if !temp.is_empty() {
                let mut result = String::new();
                result.push_str(base_domain);
                result.push_str(temp);
                // println!("base item: {:#?}", &item);
                println!("{:#?}", &result);
            }
        }
    }

    // println!("Response: {:#?}", resp);

    // let json: String = serde_json::from_str(&resp).expect("JSON was not well-formatted");

    // println!("{:#?}", json);

    Ok(String::from("ok"))
}
