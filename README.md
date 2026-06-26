# flash-code

Windows のタスクバーに表示されているアプリケーションアイコンを監視し、アイコン画像に変化があった場合に通知する Tauri アプリです。バックエンドは Rust、フロントエンドは React / TypeScript で実装しています。

## 目的

タスクバー上の通知点滅やバッジ表示など、画面を常時見ていないと気づきにくい視覚変化を検知し、Discord Webhook に通知します。設定がある場合は LINE Messaging API にも通知します。

## 主な機能

- タスクバーに表示されているウィンドウ一覧の取得
- 監視対象ウィンドウの選択
- 監視対象アイコン領域の定期キャプチャ
- 初回キャプチャ画像と現在画像の差分比較
- Discord Webhook 通知
- LINE Messaging API 通知
- 監視間隔と画像差分しきい値の設定
- 監視停止、一覧更新、アプリ終了

## 動作環境

- Windows 10 以降
- Node.js 20 LTS 推奨
- Rust stable
- npm

Node.js 24 でも動く可能性はありますが、このプロジェクトでは Tauri v1 と Vite 5 系を前提にしています。Vite 8 系は Rolldown の native binding を読み込むため、Windows の Application Control policy にブロックされる環境では起動できないことがあります。

## セットアップ

初回、または Vite / Rolldown の native binding エラーが出た場合は、依存関係を作り直します。

```powershell
Remove-Item -Recurse -Force node_modules
Remove-Item -Force package-lock.json
npm.cmd install
cd src-tauri
cargo build
cd ..
npm.cmd run tauri dev
```

`package.json` では `vite` を `5.4.21` に固定しています。`package-lock.json` が古い Vite 8 系を保持している場合、`node_modules` だけを消しても再発するため、`package-lock.json` も削除してから `npm.cmd install` してください。

PowerShell の実行ポリシーで `npm` が止まる場合は、`npm` ではなく `npm.cmd` を使います。

## 使い方

1. アプリを起動します。
2. 左側の「監視候補」から監視したいウィンドウを選び、追加ボタンで右側の「監視対象」に移します。
3. Discord Webhook URL を設定します。
4. 必要に応じて画像しきい値と監視間隔を設定します。
5. 「監視」または「全てを監視」を押します。
6. アイコン変化が検知されると通知が送信されます。
7. 「監視停止」または「閉じる」で監視を停止します。

## 設定

設定ファイルは Tauri のアプリ設定ディレクトリに `appsettings.json` として作成されます。

```json
{
  "DISCORD_WEBHOOK_URL": "",
  "LINE_CHANNEL_ACCESS_TOKEN": "",
  "LINE_TARGET": "",
  "THRESHOLD": "0.050",
  "INTERVAL": "1000"
}
```

| 項目 | 内容 |
| --- | --- |
| `DISCORD_WEBHOOK_URL` | Discord Webhook URL。空の場合、Discord 通知は送信しません。 |
| `LINE_CHANNEL_ACCESS_TOKEN` | LINE Messaging API のチャネルアクセストークン。空の場合、LINE 通知は送信しません。 |
| `LINE_TARGET` | LINE の送信先 ID。空の場合、LINE 通知は送信しません。 |
| `THRESHOLD` | 画像差分しきい値。`0.0` から `1.0` の有限数を指定します。 |
| `INTERVAL` | 監視間隔。ミリ秒単位で、`100` 以上を指定します。 |

## 検知方式

監視開始時に対象アイコン領域の初期画像を取得し、指定間隔ごとに現在画像と比較します。画像サイズが異なる場合は変化ありと判定します。画像サイズが同じ場合は RGB 差分を正規化し、しきい値を超え、かつ特定領域のオレンジ色比率が条件を満たす場合に変化ありと判定します。

これはタスクバー通知の点滅を想定した簡易検知です。すべてのアプリやすべての通知表現に対して正確に動作する保証はありません。

## プライバシー

このアプリは、タスクバーアイコンの画像変化を検知するために画面上のアイコン領域をキャプチャします。通知時には検知したアプリ名を Discord または LINE に送信します。設定値が空の通知先には送信しません。

## 既知の制限

- タスクバー上のボタン探索はウィンドウタイトルとの一致に依存します。
- タイトルが空、短すぎる、またはタスクバー表示名と一致しない場合は検知できないことがあります。
- 監視中は対象ウィンドウを最小化し、検知時に復元します。
- 高頻度の監視間隔は CPU / GDI リソース負荷を増やします。
- Linux / macOS は対象外です。

## 検証

```powershell
npm.cmd run build
cd src-tauri
cargo fmt --check
cargo check
cargo test
```

## ライセンス

MIT License です。詳細は [LICENSE](./LICENSE) を参照してください。
