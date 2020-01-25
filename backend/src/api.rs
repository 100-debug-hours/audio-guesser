use actix_web::{web::{self, BytesMut}, Error, HttpResponse, client::Client};
use serde::{Deserialize, Serialize};
use futures::StreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecognizeRequestObj {
    payload: String,
    is_binary: bool
}

#[derive(Debug, Serialize)]
struct AuddRequestFindLyricsObj<'a> {
    api_token: String,
    q: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
struct AuddSongResponseObj {
    song_id: String,
    artist_id: String,
    title: String,
    title_with_featured: String,
    full_title: String,
    artist: String,
    lyrics: String
}

#[derive(Debug, Serialize, Deserialize)]
struct AuddResponseFindLyricsObj {
    status: String,
    result: Vec<AuddSongResponseObj>,
}

async fn find_music_by_query(query: &String, client: &Client) -> Result<AuddResponseFindLyricsObj, Error> {
    let mut res = client
        .post("https://api.audd.io/findLyrics/")
        .send_json(&AuddRequestFindLyricsObj {
            api_token: crate::config::get_config().audd_io_token,
            q: query
        })
        .await
        .map_err(Error::from)?;
    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }
    Ok(serde_json::from_slice(&body).unwrap())
}

pub async fn recognize_musics(
    item: web::Json<RecognizeRequestObj>,
    client: web::Data<Client>
) -> Result<HttpResponse, Error> {
    println!("model: {:?}", &item);
    if item.is_binary {
        Ok(HttpResponse::Ok().json(item.0))
    } else {
        let res = find_music_by_query(&item.payload, &client).await;
        Ok(HttpResponse::Ok().json(res.unwrap()))
    }
}
