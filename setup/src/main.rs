use std::{
    process::Command,
    path::Path,
    time::{Duration, SystemTime}
};

use serde::{Deserialize, Serialize};
use tokio::{fs, time};
use futures::future;

use meilisearch_sdk::{
    client::Client,
    settings::Settings,
};

#[derive(Serialize, Deserialize)]
struct Hentai {
    id: u32,
    title: String,
    tags: Vec<String>,
    page: u16
}

#[derive(Deserialize)]
struct MeilisearchStatus {
    status: String
}

async fn ping() {
    let delay = Duration::from_secs(1);

    loop {
        time::sleep(delay).await;

        match reqwest::get("http://0.0.0.0:7700/health").await {
            Ok(response) => match response.text().await {
                Ok(body) => match serde_json::from_str::<MeilisearchStatus>(&body) {
                    Ok(status) => {
                        if status.status == "available" {
                            println!("Client connected");

                            break
                        }
                    },
                    Err(_) => {}
                },
                Err(_) => {}
            },
            Err(_) => {}
        }
    }
}

async fn download() {
    fs::create_dir("data").await.expect("Unable to create data directory");

    let mut ops = vec![];

    for i in 1..21 {
        ops.push(tokio::spawn(async move {
            let response = reqwest::get(format!("https://raw.githubusercontent.com/saltyaom-engine/hifumin-mirror/generated/searchable{}.json", i)).await;

            match response {
                Ok(res) => {
                    let contents = res.text().await.expect("Unable to parse content");
                    fs::write(format!("data/searchable{}.json", i), contents).await.expect("Unable to write file");

                    println!("{} done", i);
                },
                Err(_) => panic!("Unable to fetch"),
            }
        }));
    }

    future::join_all(ops).await;
}

async fn read() -> Vec<Hentai> {
    let mut files = fs::read_dir("data").await.expect("Unable to read folder");
    let mut ops = vec![];
    let mut hentais: Vec<Hentai> = vec![];

    while let Some(entry) =  files.next_entry().await.expect("End") {
        ops.push(
            tokio::spawn(async move {
                let path = entry
                    .path()
                    .to_str()
                    .expect("Unable to parse path")
                    .to_owned();

                let data = fs::read(path)
                    .await
                    .expect("Unable to read file")
                    .as_slice()
                    .to_owned();
                
                serde_json::from_slice::<Vec<Hentai>>(&data).expect("Unable to parse json")
            })
        );
    }

    let results = future::join_all(ops).await;

    for hentai in results {
        hentais.extend(hentai.expect("Unable to get hentai"));
    }

    hentais
}

#[tokio::main]
async fn main() {
    let mut meili = Command::new("./meilisearch")
        .spawn()
        .expect("failed to execute process");

    ping().await;

    if !Path::new("data").is_dir() {
        download().await;
    }

    let client = Client::new("http://0.0.0.0:7700", "masterKey");
    
    client
        .create_index("hentai", Some("id"))
        .await
        .expect("Unable to create index")
        .wait_for_completion(&client, None, None)
        .await
        .expect("Unable to create index");

    let index = client.index("hentai");

    let settings = Settings::new()
        .with_displayed_attributes(["id"])
        .with_sortable_attributes(["id"])
        .with_searchable_attributes(["tags", "title"])
        .with_filterable_attributes(["tags"])
        .with_ranking_rules([
            "words",
            "id:desc",
            "attribute",
            "proximity",
            "exactness",
            "typo"
        ]);

    index
        .set_settings(&settings)
        .await
        .expect("Unable to update settings")
        .wait_for_completion(&client, None, None)
        .await
        .expect("Update settings timeout");

    let hentais = read().await;

    let since_import = SystemTime::now();

    index
        .add_documents::<Hentai>(&hentais, Some("id"))
        .await
        .expect("Unable to add documents")
        .wait_for_completion(&client, Some(Duration::from_secs(10)), Some(Duration::from_secs(60 * 5)))
        .await
        .expect("Documents exceed timeout");

    let done = SystemTime::now()
        .duration_since(since_import)
        .expect("Time went backward");

    println!("Import done in: {:.2}s", done.as_secs_f32());

    meili.kill().expect("Unable to kill meilisearch process");
}
