use chrono::prelude::*;
use reqwest::blocking::Client;
use serde_json::Value;
use std::convert::{TryFrom, TryInto};
use std::fs;
use std::io;

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

#[derive(Debug)]
enum Status {
    Ok,
    Err,
}

impl TryFrom<String> for Status {
    type Error = &'static str;

    fn try_from(status: String) -> Result<Self, Self::Error> {
        if status == "ok" {
            Ok(Status::Ok)
        } else if status == "error" {
            Ok(Status::Err)
        } else {
            Err("Status parsing failed")
        }
    }
}

struct News {
    status: Status,
    total_results: u32,
    articles: Vec<Article>,
}

impl News {
    pub fn new(status: Status, total_results: u32) -> Self {
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
    pub fn get_num_articles(&self) -> usize {
        self.articles.len()
    }
    pub fn print_articles(&self) {
        for article in &self.articles {
            println!("{}", article.source_name);
            println!("{}", article.author);
            println!("{}", article.title);
            println!("{}", article.published_at);
            println!(
                "{:?}",
                DateTime::parse_from_rfc3339(&article.published_at)
                    .unwrap()
                    .timestamp()
            );
            println!("{}", article.desc);
            println!("{}", article.url);
            println!("{}", article.content);
        }
    }
}

fn get_news(client: &Client, term: &str, apikey: &str) -> Result<News, io::Error> {
    let url = format!(
        "https://newsapi.org/v2/everything?q={}&pageSize=100&sortBy=popularity&apiKey={}",
        term, apikey
    );
    let resp: Value = serde_json::from_str(&client.get(&url).send().unwrap().text().unwrap())?;

    let status: Status = resp["status"].to_string().try_into().unwrap();

    let total_results = resp["totalResults"].as_u64().unwrap();

    let articles = resp["articles"].as_array().unwrap();

    let mut news = News::new(status, total_results as u32);
    for article in articles {
        news.add_article(
            article["source"]["name"].to_string(),
            article["author"].to_string(),
            article["title"].to_string(),
            article["description"].to_string(),
            article["url"].to_string(),
            article["publishedAt"]
                .to_string()
                .trim_matches('"')
                .to_owned(),
            article["content"].to_string(),
        );
    }
    Ok(news)
}

fn main() -> Result<(), io::Error> {
    let api = fs::read_to_string("apikey")?;
    api.trim_matches(char::is_control).to_string();
    let client = Client::new();

    let news = get_news(&client, "Apple", &api).unwrap();

    println!("{:?}, {}", news.status, news.total_results);

    news.print_articles();

    println!("{}", news.get_num_articles());

    let pub_at: String = "2021-01-09T04:55:00Z".to_owned();
    let dt = DateTime::parse_from_rfc3339(&pub_at).unwrap();
    println!("{:?}", dt.timestamp());

    Ok(())
}
