use regex::Regex;

pub async fn get_metatags(url: &str) -> String {
    let re_title = Regex::new(r#"<title>(.+)</title>"#).unwrap();
    let re_description =
        Regex::new(r#"<meta.*name=".*description.*".*content="(.+)"\s?/?>"#).unwrap();

    let client = reqwest::Client::new();

    let resp = client
        .get(url)
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
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

    println!("{}", title);

    if data.len() < 499 {
        return data;
    }

    let data = data.chars();
    let data: String = data.into_iter().take(499).collect();
    return data;
}

pub fn get_links(s: &str) -> Vec<String> {
    let re_title = Regex::new(r#"(^|\s|\n)*(https?://[^\s]+)($|\s|\n)*"#).unwrap();

    let mut vec = Vec::new();

    for cap in re_title.captures_iter(s) {
        vec.push(String::from(&cap[2]));
    }

    return vec;
}
