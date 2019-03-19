# GyazoCrostini

Gyazo uploader for Crostini

UNOFFICIAL/EXPERIMENTAL

Chrome OS標準スクリーンショット機能によって保存された画像ファイルを都度Gyazoにuploadするプログラム

[![](https://img.youtube.com/vi/KtDG-SfLn9I/0.jpg)](https://www.youtube.com/watch?v=KtDG-SfLn9I)

## Chrome OS Settings
ダウンロードファイルの保存先として、「Linux ファイル」下を指定する。\
プログラムを起動すると、このディレクトリの変更が監視される。

![](https://gyazo.com/1de432aa86f6cba5b2aa06f2bb646811/raw)

## Preparation
Gyazoアプリを登録して、access tokenを取得する。取り扱い注意。\
https://gyazo.com/oauth/applications

src/key.rs に、Gyazo access tokenを記述する。
```rust
pub(crate) const GYAZO_ACCESS_TOKEN: &'static str = "your-gyazo-access-token";
pub(crate) const SCREENSHOT_DIR: &'static str = "/home/daizmg/CrDownloads";
pub(crate) const SCREENSHOT_DESC: &'static str = "uploaded from #PixelSlate";
```

## Build
```
$ sh build.sh
```

## Run
```
$ sh start.sh
```
