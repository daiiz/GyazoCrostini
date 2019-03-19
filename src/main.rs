extern crate notify;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate open;

mod key;

use serde_derive::{Deserialize};
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;
// use std::thread;
use key::*;

fn main() {
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();
    watcher.watch(SCREENSHOT_DIR, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            // 操作とファイルパスを取得
            Ok(RawEvent{path: Some(path), op: Ok(op), cookie: _}) => {
                // 書き込み終了だけに興味がある
                if format!("{:?}", op) == "CLOSE_WRITE" {
                    println!("wrote: {:?}", path);
                    let path_str = path.to_str().unwrap();
                    // 画像ファイルだけに興味がある
                    if is_image_path(&path_str) {
                        uploader(&path_str);
                    }
                }
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

// TODO:
fn is_image_path (path: &str) -> bool {
    path.ends_with(".png") ||
    path.ends_with(".gif") ||
    path.ends_with(".jpg") || path.ends_with(".jpeg")
}

#[derive(Deserialize, Debug)]
struct GyazoImage {
    permalink_url: String,
}

// https://gyazo.com/api/docs/image
fn uploader(path: &str) {
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .text("access_token", GYAZO_ACCESS_TOKEN)
        .text("desc", SCREENSHOT_DESC)
        .file("imagedata", path)
        .unwrap();

    let api_url = "https://upload.gyazo.com/api/upload";
    let mut res = client.post(api_url)
        .multipart(form)
        .send()
        .unwrap();

    if res.status().is_success() {
        if let Ok(data) = res.json::<GyazoImage>() {
            let image_url = data.permalink_url;
            println!("uploaded!: {}", image_url);
            open::that(image_url).unwrap();
        }
    } else {
        panic!("Something else happened. Status: {:?}", res.status());
    }
}

