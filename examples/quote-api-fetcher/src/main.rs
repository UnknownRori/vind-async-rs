#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Quote {
    #[serde(rename(deserialize = "_id"))]
    id: String,
    content: String,
    author: String,
    tags: Vec<String>,
    #[serde(rename(deserialize = "dateModified"))]
    date_modified: String,
    #[serde(rename(deserialize = "dateAdded"))]
    date_added: String,
}

async fn fetch(url: &str) -> attohttpc::Result<attohttpc::Response> {
    attohttpc::get(url).send()
}

async fn deserialize<'a, T: serde::Deserialize<'a>>(str: &'a str) -> T {
    serde_json::from_str::<T>(str).unwrap()
}

#[vind_async::main]
async fn main() {
    println!("------------");

    let response = fetch("https://api.quotable.io/random").await.unwrap();
    let response = response.text().unwrap();
    let response: Quote = deserialize::<Quote>(&response).await;
    dbg!(response);

    println!("------------");

    let response1 = fetch("https://api.quotable.io/random");
    let response2 = fetch("https://api.quotable.io/random");
    let (response1, response2) = futures::join!(response1, response2);

    let response_text1 = response1.unwrap().text().unwrap();
    let response_text2 = response2.unwrap().text().unwrap();

    let response1 = deserialize::<Quote>(&response_text1);
    let response2 = deserialize::<Quote>(&response_text2);

    let (quote1, quote2) = futures::join!(response1, response2);
    dbg!(quote1, quote2);
}
