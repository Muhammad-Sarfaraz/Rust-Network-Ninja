use reqwest;

pub async fn get_geolocation() {
    // Fetch location from IP
    let ip_api_url = "https://api64.ipify.org?format=json";
    let ip_response = reqwest::get(ip_api_url).await?;
    let ip_data: serde_json::Value = ip_response.json().await?;
    println!("Location from IP: {:#?}", ip_data);

    // Fetch location
    let location_api_url = "http://ip-api.com/json";
    let location_response = reqwest::get(location_api_url).await?;
    let location_data: serde_json::Value = location_response.json().await?;
    println!("Location: {:#?}", location_data);

    Ok(())
}