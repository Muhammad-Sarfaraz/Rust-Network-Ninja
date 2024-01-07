use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::borrow::Cow;

pub async fn check_url_availability(url: &str) {
    let client = reqwest::Client::new();
    let progress_bar = ProgressBar::new_spinner();

    progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .tick_chars("/|\\- "),
    );

    match client.get(url).send().await {
        Ok(_) => {
            progress_bar.finish_with_message("URL is up!");
        }
        Err(err) => {
            progress_bar.finish_with_message(Cow::Owned(err.to_string()));
        }
    }
}