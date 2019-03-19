# GyazoSlate

Gyazo uploader for Pixel Slate (Crostini)

UNOFFICIAL/EXPERIMENTAL

## Chrome OS Settings
ダウンロードファイルの保存先として、「Linux ファイル」下を指定する。\
このディレクトリを監視して、画像ぽいファイルが保存されたら都度Gyazoにuploadしていく。

![](https://gyazo.com/1de432aa86f6cba5b2aa06f2bb646811/raw)

## Preparation
### Gyazo access token
Gyazoアプリを登録して、access tokenを取得する。取り扱い注意。
https://gyazo.com/oauth/applications

src/key.rs に、Gyazo access tokenを記述する。
```rust
pub(crate) const GYAZO_ACCESS_TOKEN: &'static str = "your-gyazo-access-token";
```

### SCREENSHOT_DIR
src/main.rs の`SCREENSHOT_DIR`にChrome OSに指定した保存先パスに書き換える。

### SCREENSHOT_DEVICE_PIXEL_RATIO
(うまく効かない。今後の課題。) \
Pixel Slate以外のChrome OSマシンを使用している場合は適切な値を設定する。
Chrome Dev Toolsにて`window.devicePixelRatio`を実行すると確認できる。


## Build
```
$ cargo build
```

## Run
```
$ cargo run
```
