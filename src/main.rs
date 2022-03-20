use regex::Regex;
use serde::*;
use std::collections::*;
use std::env;

#[derive(Serialize, Deserialize)]
struct TimeRecord {
    timeStamp: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if (args.len() == 1) {
        panic!("Pass the domain name in the parameter!");
    }
    let domain: &String = &args[1];

    // println!("{:?}", args);

    let base_url = format!("https://web.archive.org/cdx/search/cdx?url={}/robots.txt&output=json&filter=statuscode:200&fl=timestamp,original&collapse=digest",domain);
    let resp = reqwest::get(base_url).await?.text().await?;

    let json: Vec<TimeRecord> = serde_json::from_str(&resp).expect("JSON was not well-formatted");

    for obj in json {
        if obj.url == "original" {
            continue;
        }

        // println!("timeStamp: {:#?}, Url: {:#?}", obj.timeStamp, obj.url);
        get_robot_file(obj.timeStamp, obj.url).await?;
    }

    //println!("{:#?}", json);
    Ok(())
}

async fn get_robot_file(
    timestamp: String,
    mut url: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = format!("http://web.archive.org/web/{}if_/{}", timestamp, url);
    // println!("second call url: {:#?}", base_url);
    let resp = reqwest::get(base_url).await?.text().await?;

    if resp.is_empty() {
        return Ok(String::from("ok"));
    }

    // let items = resp.split("\n");
    let items: Vec<&str> = resp.split("\n").collect();

    let _regex = Regex::new(r#"(?i)(disallow|allow)(\\s?)\\:(\\s?)(.*)""#).unwrap();

    for item in items {
        if _regex.is_match(&item) {
            let sub_item: Vec<&str> = item.split(": ").collect();
            println!("is fucking match: {:#?}", &url.push_str(sub_item[1]));
        }
    }

    // println!("Response: {:#?}", resp);

    // let json: String = serde_json::from_str(&resp).expect("JSON was not well-formatted");

    // println!("{:#?}", json);

    Ok(String::from("ok"))
}
