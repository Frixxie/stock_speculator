use chrono::prelude::*;
use reqwest::blocking::Client;
use serde_json::Value;
use std::convert::{TryFrom, TryInto};
use std::fmt;
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

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            self.source_name,
            self.author,
            self.title,
            self.desc,
            self.url,
            self.published_at,
            self.content
        )
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
            println!("{}", article);
        }
    }
}

fn get_news(client: &Client, term: &str, apikey: &str, page_size: u32) -> Result<News, io::Error> {
    let url = format!(
        "https://newsapi.org/v2/everything?q={}&pageSize={}&sortBy=popularity&apiKey={}",
        term, page_size, apikey
    );
    let resp: Value = serde_json::from_str(&client.get(&url).send().unwrap().text().unwrap())?;

    let status: Status = resp["status"]
        .to_string()
        .trim_matches('"')
        .to_owned()
        .try_into()
        .unwrap();

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

fn get_symbol(client: &Client, term: &str, apikey: &str) -> Vec<(String, String)> {
    let url = format!(
        "https://alphavantage.co/query?function=SYMBOL_SEARCH&keywords={}&apikey={}",
        term, apikey
    );
    let res: Value =
        serde_json::from_str(&client.get(&url).send().unwrap().text().unwrap()).unwrap();

    let reses = res["bestMatches"].as_array().unwrap();

    let mut res: Vec<(String, String)> = Vec::<(String, String)>::new(); 

    for tmp in reses {
        res.push((tmp["symbol"].to_string(), tmp["name"].to_string()));
    }

    res
}

fn main() -> Result<(), io::Error> {
    let apikey = fs::read_to_string("apikey_newsapi")?;
    apikey.trim_matches(char::is_control).to_string();
    let client = Client::new();

    let search_term = "Apple".to_owned();

    // let news = get_news(&client, &search_term, &apikey, 100).unwrap();

    // println!("{:?}, {}", news.status, news.total_results);

    // news.print_articles();

    // println!("{}", news.get_num_articles());

    Ok(())
}
