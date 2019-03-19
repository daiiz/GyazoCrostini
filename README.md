# GyazoSlate

Gyazo uploader for Pixel Slate (Crostini)

UNOFFICIAL/EXPERIMENTAL

## Chrome OS Settings
ダウンロードファイルの保存先として、「Linux ファイル」下を指定する。\
このディレクトリを監視しておき、OS標準スクリーンショット機能などによって保存された画像ぽいファイルを都度Gyazoにuploadしていく。

![](https://gyazo.com/1de432aa86f6cba5b2aa06f2bb646811/raw)

## Preparation
### Gyazo access token
Gyazoアプリを登録して、access tokenを取得する。取り扱い注意。\
https://gyazo.com/oauth/applications

src/key.rs に、Gyazo access tokenを記述する。
```rust
pub(crate) const GYAZO_ACCESS_TOKEN: &'static str = "your-gyazo-access-token";
pub(crate) const SCREENSHOT_DIR: &'static str = "/home/daizmg/CrDownloads";
pub(crate) const SCREENSHOT_DESC: &'static str = "uploaded from #PixelSlate";
```

### SCREENSHOT_DIR
src/main.rs の`SCREENSHOT_DIR`を、先ほど指定した保存先パスに書き換える。

## Build
```
$ sh build.sh
```

## Run
```
$ sh start.sh
```
