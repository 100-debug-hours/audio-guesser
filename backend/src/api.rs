use actix_web::{web::{self, BytesMut}, Error, HttpResponse, client::Client};
use actix_multipart::Multipart;
use serde::{Deserialize, Serialize};
use reqwest::multipart::{self, Part};
use futures::StreamExt;
use std::borrow::Cow;
use std::error;
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecognizeRequestObj {
    payload: String
}

#[derive(Debug, Serialize)]
struct AuddRequestFindByLyricsObj<'a> {
    api_token: String,
    q: &'a str
}

#[derive(Debug, Serialize)]
struct AuddRequestFindByFileObj<'a> {
    api_token: String,
    audio: &'a str,
    r#return: &'a str
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

#[derive(Debug, Serialize, Deserialize)]
struct DeezerArtisrtObj {
    id: usize,
    name: String,
    link: String,
    picture: String,
    picture_small: String,
    picture_medium: String,
    picture_big: String,
    picture_xl: String,
    tracklist: String,
    r#type: String
}

#[derive(Debug, Serialize, Deserialize)]
struct DeezerAlbumObj {
    id: usize,
    //title: String,
    cover: String,
    cover_small: String,
    cover_medium: String,
    cover_big: String,
    cover_xl: String,
    tracklist: String,
    r#type: String
}

#[derive(Debug, Serialize, Deserialize)]
struct DeezerObj {
    id: usize,
    readable: bool,
    //title: String,
    title_short: String,
    title_version: String,
    link: String,
    duration: i32,
    rank: usize,
    explicit_lyrics: bool,
    preview: String,
    artist: DeezerArtisrtObj,
    album: DeezerAlbumObj,
    r#type: String
}

#[derive(Debug, Serialize, Deserialize)]
struct AuddSongDeezeResponseObj {
    title: String,
    artist: String,
    album: String,
    release_date: String,
    label: String,
    deezer: DeezerObj
}

#[derive(Debug, Serialize, Deserialize)]
struct AuddResponseFindFileObj {
    status: String,
    result: AuddSongDeezeResponseObj,
}

async fn find_music_by_query(query: &String, client: &Client) -> Result<AuddResponseFindLyricsObj, Error> {
    let mut res = client
        .post("https://api.audd.io/findLyrics/")
        .send_json(&AuddRequestFindByLyricsObj {
            api_token: crate::config::get_config().audd_io_token,
            q: query
        })
        .await
        .map_err(Error::from)?;
    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }
    Ok(serde_json::from_slice(&body).map_err(Error::from)?)
}

async fn find_music_by_file(file: Part) -> Result<JsonValue, Box<dyn error::Error>> {
    let client = reqwest::Client::new();
    let form = multipart::Form::new()
        .text("return", "deezer")
        .text("api_token", crate::config::get_config().audd_io_token)
        .part("file", file);

    let resp = client
        .post("https://api.audd.io/recognizeWithOffset/")
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;
    
    Ok(serde_json::from_str(&resp)?)
}

pub async fn recognize_text(
    item: web::Json<RecognizeRequestObj>,
    client: web::Data<Client>
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(find_music_by_query(&item.payload, &client).await.map_err(Error::from)?))
}

pub async fn recognize_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut body = BytesMut::new();
    let mut filename = String::new();
    let mut mime_type = String::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content = field.content_disposition().unwrap();
        if content.get_name().unwrap_or("") == "file" {
            filename = content.get_filename().unwrap_or("").to_string();
            mime_type = field.content_type().essence_str().to_string();
            while let Some(chunk) = field.next().await {
                body.extend_from_slice(&chunk?);
            }
            break;
        }
    }
    let mut f = Part::bytes(Cow::Owned(body.to_vec()));
    f = f.mime_str(&mime_type).unwrap();
    f = f.file_name(filename);
    let muson = find_music_by_file(f).await.unwrap();

    Ok(HttpResponse::Ok().json(muson))
}
