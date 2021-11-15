#![allow(dead_code)]

use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::multipart;
use reqwest::{Body, Error, StatusCode};
use serde::Deserialize;
use tokio;
use tokio_util::codec::{BytesCodec, FramedRead};
use tokio::io::AsyncRead;

pub struct Arachnid {
    key: String,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct ArachnidResp {
    pub classification: String,
    pub distance: i32,
}
#[derive(Debug, Deserialize)]
pub struct ArachnidError {
    error: String,
}

impl Default for ArachnidError {
    fn default() -> Self {
        ArachnidError {
            error: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ArachnidResult {
    raw_resp: Vec<ArachnidResp>,
    err: ArachnidError,
    status_code: StatusCode,
    errored: bool,
    safe: bool,
}

impl Arachnid {
    pub fn new(key: String, url: String) -> Arachnid {
        Arachnid { key, url }
    }

    pub async fn check_file<T>(
        self: &Arachnid,
        file: T,
        file_name: String,
        mime: String,
    ) -> Result<ArachnidResult, Error>
    where
        T: AsyncRead + Sync + Send + 'static,
    {
        let stream = FramedRead::new(file, BytesCodec::new());
        let data = multipart::Part::stream(Body::wrap_stream(stream))
            .file_name(file_name)
            .mime_str(&mime)?;
        let form = multipart::Form::new().part("image", data);
        let client = reqwest::Client::new();
        let mut hd = HeaderMap::new();
        hd.insert(
            "Authorization",
            HeaderValue::from_str(self.key.as_str()).unwrap(),
        );
        let res = client
            .post(&self.url)
            .headers(hd)
            .multipart(form)
            .send()
            .await?;
        let stat = res.status();
        if stat == 200 {
            let resp: Vec<ArachnidResp> = res.json().await?;
            let len = (&resp).len();
            Ok(ArachnidResult {
                raw_resp: resp,
                err: ArachnidError::default(),
                status_code: stat,
                safe: len == 0,
                errored: false,
            })
        } else {
            let resp: ArachnidError = res.json().await.unwrap();
            Ok(ArachnidResult {
                raw_resp: vec![],
                err: resp,
                status_code: stat,
                safe: false,
                errored: true,
            })
        }
    }
}
