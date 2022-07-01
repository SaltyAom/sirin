use std::{
    fs::File, 
    io::{prelude::Read, BufReader, BufRead},
    process::{Command, Stdio, exit, Child},
    thread,
    collections::HashMap,
    env::current_dir
};

use tokio::{self, task::JoinHandle};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use meilisearch_sdk::{
    client::Client,
    settings::Settings,
};

use reqwest;


#[derive(Serialize, Deserialize, Debug)]
pub struct Hentai {
    pub id: u32,
    pub title: String,
    pub tags: Vec<String>,
    pub page: u16,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    status: String
}

fn start_listener() -> Child {
    println!("{}/meilisearch", current_dir()
        .unwrap()
        .to_str()
        .unwrap()
    );

    let mut child = Command::new("./meilisearch")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start meilisearch");

    println!("Started process: {}", child.id());

    start_process(&mut child);

    child
}

fn start_process(child: &mut Child) {
    let stdout = child.stdout.take().unwrap();

    thread::spawn(move || {
        let mut f = BufReader::new(stdout);

        loop {
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {}
                Err(e) => println!("an error!: {:?}", e),
            }
        }
    });
}

pub async fn setup() {
    let client = reqwest::Client::new();

    // interval 1s
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

    loop {
        interval.tick().await;

        match client.get("http://localhost:7700/health").send().await {
            Ok(response) => {
                match response.json::<Status>().await {
                    Ok(res) => {
                        if res.status == "available" {
                            break;
                        }        
                    },
                    Err(_) => {
                        continue;
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
}

pub async fn create_client() {
    let meilisearch = Client::new("http://localhost:7700", "masterKey");

    let mut handler: Vec<JoinHandle<Vec<Hentai>>> = vec![];

    for iteration in 1..21 {
        handler.push(tokio::spawn(async move {
            return import_search(iteration)
                .await
                .expect("Unable to import search");
        }));
    }

    let res = join_all(handler).await;
    let documents: Vec<Hentai> = res
        .into_iter()
        .flat_map(|arr| arr.expect("Array is empty"))
        .collect();

    println!("Load {} fragments", documents.len());

    let engine = meilisearch.index("hentai");

    engine.set_settings(
        &Settings {
            displayed_attributes: Some(vec!["id".to_owned()]),
            sortable_attributes: Some(vec!["id".to_owned()]),
            searchable_attributes: Some(vec!["tags".to_owned(), "title".to_owned()]),
            filterable_attributes: Some(vec!["tags".to_owned()]),
            ranking_rules: Some(vec![
                "words".to_owned(),
                "id:desc".to_owned(),
                "attribute".to_owned(),
                "proximity".to_owned(),
                "exactness".to_owned(),
                "typo".to_owned()
            ]),
            stop_words: None,
            distinct_attribute: None,
            synonyms: Some(HashMap::from([
                ("yaoi".to_owned(), vec!["males only".to_owned()]),
                ("yuri".to_owned(), vec!["females only".to_owned()])
            ]))
        }
    )
        .await
        .expect("Unable to update index")
        .wait_for_completion(&meilisearch, None, None)
        .await
        .expect("Unable to wait for index");

    engine
        .add_documents(&documents, Some("id"))
        .await
        .expect("Unable to add documents to batch")
        .wait_for_completion(&meilisearch, None, None)
        .await
        .expect("Unable to join the remote server");
}

pub async fn import_search(batch: u8) -> Result<Vec<Hentai>, std::io::Error> {
    let mut file = File::open(format!("data/searchable{}.json", batch))?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let document: Vec<Hentai> = serde_json::from_str(&content)?;

    Ok(document)
}

#[tokio::main]
async fn main() {
    let mut child = start_listener();

    setup().await;

    println!("Setup done");

    create_client().await;
    
    println!("Import done");
    
    child.kill().unwrap();
    exit(0);
}
