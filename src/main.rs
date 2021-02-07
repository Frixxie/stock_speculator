use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;
use std::io;
use std::string::String;

struct Article {
    source_name: String,
    author: String,
    title: String,
    desc: String,
    url: String,
    published_at: String,
    content: String,
}

impl Article {
    pub fn new(
        source_name: String,
        author: String,
        title: String,
        desc: String,
        url: String,
        published_at: String,
        content: String,
    ) -> Self {
        Self {
            source_name,
            author,
            title,
            desc,
            url,
            published_at,
            content,
        }
    }
}

struct News {
    status: String,
    total_results: i64,
    articles: Vec<Article>,
}

impl News {
    pub fn new(status: String, total_results: i64) -> Self {
        Self {
            status,
            total_results,
            articles: Vec::new(),
        }
    }
    pub fn add_article(
        &mut self,
        source_name: String,
        author: String,
        title: String,
        desc: String,
        url: String,
        published_at: String,
        content: String,
    ) {
        self.articles.push(Article::new(
            source_name,
            author,
            title,
            desc,
            url,
            published_at,
            content,
        ));
    }
}

fn get_news(client: &Client, term: &str, apikey: &str) -> News {
    let url = format!(
        "https://newsapi.org/v2/everything?q={}&pageSize=100&apiKey={}",
        term, apikey
    );

    let response = client.get(&url).send().unwrap().text().unwrap();

    let resp: Value = serde_json::from_str(&response).unwrap();

    let status = resp["status"].as_str().unwrap().to_string();

    let total_results = resp["totalResults"].as_i64().unwrap();

    let articles = resp["articles"].as_array().unwrap();

    let mut news = News::new(status, total_results);
    for i in articles {
        news.add_article(
            i["source"]["name"].to_string(),
            i["author"].to_string(),
            i["title"].to_string(),
            i["desc"].to_string(),
            i["url"].to_string(),
            i["published_at"].to_string(),
            i["content"].to_string(),
        );
    }
    news
}

fn main() -> Result<(), io::Error> {
    let api = fs::read_to_string("apikey")?;
    api.trim_matches(char::is_control).to_string();
    let client = Client::new();

    let news = get_news(&client, "Apple", &api);

    for article in &news.articles {
        println!("{}", article.title);
        println!("{}", article.content);
    }
    println!("{}", news.articles.len());

    Ok(())
}
