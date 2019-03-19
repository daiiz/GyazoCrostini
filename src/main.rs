extern crate notify;
extern crate reqwest;

mod key;

use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;
// use std::thread;
use key::*;

const SCREENSHOT_DIR: &'static str = "/home/daizmg/CrDownloads";
const SCREENSHOT_DESC: &'static str = "uploaded from #PixelSlate";

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

fn is_image_path (path: &str) -> bool {
    path.ends_with(".png") ||
    path.ends_with(".gif") ||
    path.ends_with(".jpg") || path.ends_with(".jpeg")
}

// https://gyazo.com/api/docs/image
fn uploader(path: &str) {
    let client = reqwest::Client::new();
    // TODO: ファイル名くらい載せたい
    let form = reqwest::multipart::Form::new()
        .text("access_token", GYAZO_ACCESS_TOKEN)
        .text("desc", SCREENSHOT_DESC)
        .file("imagedata", path)
        .unwrap();

    let api_url = "https://upload.gyazo.com/api/upload";
    let res = client.post(api_url)
        .multipart(form)
        .send()
        .unwrap();
    println!("uploaded: {:?}", res);
}

