use reqwest::Client;

type asyncError = Box<dyn std::error::Error + Send + Sync + 'static>;
#[tokio::main]
pub async fn main() -> Result<(), asyncError> {
    main_program().await?;

    Ok(())
}

async fn get_ip_info(ip: &str) -> Result<(), asyncError> {
    let client = Client::new();

    let res = match client
        .get(format!("http://ip-api.com/json/{}", ip))
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => {
            println!("Please check your internet connection");
            prompt("Press any key to exit");
            return Ok(());
        }
    };
    let json = match res.json::<serde_json::Value>().await {
        Ok(json) => json,
        Err(_) => {
            println!("Please check your internet connection");
            prompt("Press any key to exit");
            return Ok(());
        }
    };

    let status = json["status"].as_str().unwrap();
    if status == "fail" {
        let message = json["message"].as_str().unwrap();
        match message {
            "private range" => {
                println!("The IP you provided is a private IP that cannot be retrieved")
            }
            "reserved range" => {
                println!("The IP you provided is a reserved IP that cannot be retrieved")
            }
            "invalid query" => {
                println!("The IP you provided is invalid")
            }
            _ => {}
        }
    } else {
        let country = json["country"].as_str().unwrap();
        let region = json["regionName"].as_str().unwrap();
        let city = json["city"].as_str().unwrap();
        let zip = json["zip"].as_str().unwrap();
        let lat = json["lat"].as_f64().unwrap();
        let lon = json["lon"].as_f64().unwrap();
        let isp = json["isp"].as_str().unwrap();

        println!("Country (Country name): {}", country);
        println!("Region (Region/state name): {}", region);
        println!("City (City name): {}", city);
        println!("Zip (Zip code): {}", zip);
        println!("Latitude: {}", lat);
        println!("Longitude: {}", lon);
        println!("ISP (Isp name): {}", isp);
    }
    Ok(())
}

fn prompt(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

async fn main_program() -> Result<String, asyncError> {
    let ip = &prompt("Enter an IP address (Enter nothing to get your own ip info): ");
    get_ip_info(ip).await?;
    println!("Results for IP: {}", ip);
    println!("Press 1 to get another IP");
    println!("Press any other key to exit");
    let choice = prompt("Enter your choice: ");
    Box::pin(match_choice(&choice)).await?;
    Ok(choice)
}
async fn match_choice(choice: &str) -> Result<(), asyncError> {
    match choice.trim() {
        "1" => {
            main_program().await?;
        }
        _ => std::process::exit(0),
    }
    Ok(())
}
