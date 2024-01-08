use reqwest;

#[derive(Debug, serde::Deserialize)]
struct GeoLocation {
    ip: String,
    city: String,
    region: String,
    country: String,
    loc: String,
    org: String,
    postal: String,
}

async fn get_geolocation(api_key: &str) -> Result<GeoLocation, reqwest::Error> {
    let api_url = format!("http://ipinfo.io?token={}", api_key);

    // Make a GET request to the IPinfo API
    let response = reqwest::get(&api_url).await?;

    // Deserialize the JSON response into the GeoLocation struct
    let geolocation: GeoLocation = response.json().await?;

    Ok(geolocation)
}