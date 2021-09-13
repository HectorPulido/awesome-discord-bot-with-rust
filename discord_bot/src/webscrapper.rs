use regex::Regex;

pub async fn get_metatags(url: &str) -> String {
    let re_title = Regex::new(r#"<title>(.+)</title>"#).unwrap();
    let re_description =
        Regex::new(r#"<meta.*name=".*description.*".*content="(.+)"\s?/?>"#).unwrap();

    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
    let resp = resp.as_str();

    let mut title = String::new();

    for cap in re_title.captures_iter(resp) {
        title = String::from(&cap[1]);
    }

    let mut description = String::new();

    for cap in re_description.captures_iter(resp) {
        description = String::from(&cap[1]);
    }

    let data: String = format!("{} | {} | {}", title, description, url);

    if data.len() < 499 {
        return data;
    }

    let data = data.chars();
    let data: String = data.into_iter().take(499).collect();
    return data;
}
