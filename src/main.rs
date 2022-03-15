use std::collections::HashMap;
use std::env;

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

    let json: serde_json::Value = serde_json::from_str(&resp).expect("JSON was not well-formatted");

    for obj in json.as_array() {
        println!("adasdasdasdasd: {:#?}", obj);
    }

    println!("{:#?}", json);
    Ok(())
}

async fn get_robot_file(
    timestamp: String,
    url: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = format!("http://web.archive.org/web/{}if_/{}", timestamp, url);
    let resp = reqwest::get(base_url).await?.text().await?;

    let json: serde_json::Value = serde_json::from_str(&resp).expect("JSON was not well-formatted");

    println!("{:#?}", json);

    Ok(String::from("test"))
}
