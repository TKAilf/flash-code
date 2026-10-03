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
- 監視開始時に対象ウィンドウを最小化するかどうかの設定
- 監視停止、一覧更新、アプリ終了

## 動作環境

- Windows 10 以降
- Node.js 20 LTS 推奨
- Rust stable
- npm
- Microsoft Edge WebView2 Runtime

Node.js 24 でも動く可能性はありますが、このプロジェクトでは Tauri v1 と Vite 5 系を前提にしています。Vite 8 系は Rolldown の native binding を読み込むため、Windows の Application Control policy にブロックされる環境では起動できないことがあります。

MSI 作成時の WebView2 設定は `skip` です。これはビルド時に Microsoft の WebView2 Bootstrapper をダウンロードしないための設定です。配布先 PC に WebView2 Runtime が入っていない場合は、アプリ起動前に別途インストールしてください。

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

## リリースビルド

```powershell
npm.cmd run tauri build
```

生成物は `src-tauri\target\release\bundle\msi` に出力されます。

インストール済みのアプリを更新する場合は、`package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` のバージョンを上げてから MSI を作成してください。同じバージョンの MSI を再実行しただけでは、Windows Installer の扱いにより古い EXE が残る場合があります。

インストール後の起動で `localhost` への接続拒否が表示される場合は、開発用ビルドまたは古いインストール済み EXE が起動されています。いったん「アプリと機能」から `flash-code` をアンインストールし、上記コマンドで作成した最新バージョンの MSI をインストールしてください。

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
  "LINE_ENABLED": "false",
  "LINE_CHANNEL_ACCESS_TOKEN": "",
  "LINE_TARGET": "",
  "THRESHOLD": "0.050",
  "INTERVAL": "1000",
  "MINIMIZE_ON_MONITOR_START": "true"
}
```

| 項目 | 内容 |
| --- | --- |
| `DISCORD_WEBHOOK_URL` | Discord Webhook URL。空の場合、Discord 通知は送信しません。 |
| `LINE_ENABLED` | LINE 通知の有効状態。`"true"` の場合だけ送信します。既定値は `"false"` です。 |
| `LINE_CHANNEL_ACCESS_TOKEN` | LINE Messaging API のチャネルアクセストークン。空の場合、LINE 通知は送信しません。 |
| `LINE_TARGET` | LINE の送信先 ID。空の場合、LINE 通知は送信しません。 |
| `THRESHOLD` | 画像差分しきい値。`0.0` から `1.0` の有限数を指定します。 |
| `INTERVAL` | 監視間隔。ミリ秒単位で、`100` 以上を指定します。 |
| `MINIMIZE_ON_MONITOR_START` | 監視開始時に対象ウィンドウを最小化するかどうか。`"true"` の場合は最小化します。既定値は `"true"` です。 |

### LINE 通知の設定

アプリ内の「LINE 設定ガイド」タブで、図と段階別の手順を確認できます。LINE 設定欄の「設定方法を見る」からも開けます。タブを切り替えても入力途中の値は保持されます。

LINE は、自分の Bot を用意できる方向けの任意機能です。Discord のみでも利用できます。共通の Bot やトークンはアプリから提供しません。配布先の利用者も、それぞれ自分の LINE 公式アカウントと Messaging API チャネルを用意してください。

自分の LINE に通知する手順：

1. [LINE 公式の開始手順](https://developers.line.biz/ja/docs/messaging-api/getting-started/)に従い、LINE Official Account Manager で公式アカウントを作成し、Messaging API を有効にします。
2. [LINE Developers Console](https://developers.line.biz/console/)で、その Messaging API チャネルを開きます。
3. ［Messaging API設定］の［チャネルアクセストークン（長期）］から発行し、アプリの `Channel Access Token` に貼り付けます。これは Bot として送信する権限を証明する秘密情報です。Channel Secret やチャネル ID とは異なります。[トークンの公式説明](https://developers.line.biz/ja/docs/basics/channel-access-token/)
4. 同じチャネルの［チャネル基本設定］にある［あなたのユーザーID］を `Target ID` に貼り付けます。これは受信する自分の ID で、表示名や友だち検索用の LINE ID ではありません。表示されない場合は Business ID に LINE アカウントを連携してください。[ユーザー ID の公式説明](https://developers.line.biz/ja/docs/messaging-api/getting-user-ids/)
5. 自分の LINE でその公式アカウントを友だち追加し、ブロックされていないことを確認します。
6. アプリの LINE Bot を On にして、両方の値を入力後に「Set LINE」を押します。アイコン変化を検知した際に通知します。このボタン自体はテスト送信しません。

保存後、トークン入力欄は空になります。`Token: Configured` が設定済みの表示です。トークン欄を空にして「Set LINE」を押すと保存済みトークンを維持します。ターゲット ID は入力値で更新するため、空にすると通知できなくなります。LINE が Off、またはどちらかが未設定の場合も送信しません。

自分宛ての通知だけなら Webhook サーバーは不要です。グループ宛ての場合は Bot をグループに参加させ、別途受信した Webhook の `source.groupId` を使います。このアプリは Webhook の受信や ID の自動取得には対応していません。[グループトークの公式説明](https://developers.line.biz/ja/docs/messaging-api/group-chats/)

トークンは設定ファイルに保存されます。画面でマスクされていても、トークンやトークン入りの `appsettings.json` を他人に渡したり、公開リポジトリ・配布物に含めたりしないでください。

このアプリの通知は、月間送信通数にカウントされるプッシュメッセージです。LINE のプランの送信上限に達すると通知できません。最新の料金と上限は [Messaging API の料金](https://developers.line.biz/ja/docs/messaging-api/pricing/)を確認してください。

## 検知方式

監視開始時に対象アイコン領域の初期画像を取得し、指定間隔ごとに現在画像と比較します。画像サイズが異なる場合は変化ありと判定します。画像サイズが同じ場合は RGB 差分を正規化し、しきい値を超え、かつ特定領域のオレンジ色比率が条件を満たす場合に変化ありと判定します。

既定では、監視開始時に対象ウィンドウを最小化します。これは、対象アプリがアクティブな状態ではタスクバー通知点滅が発生せず、画像差分として検知できない場合があるためです。設定で無効化できます。

これはタスクバー通知の点滅を想定した簡易検知です。すべてのアプリやすべての通知表現に対して正確に動作する保証はありません。

## プライバシー

このアプリは、タスクバーアイコンの画像変化を検知するために画面上のアイコン領域をキャプチャします。通知時には検知したアプリ名を Discord または LINE に送信します。設定値が空の通知先には送信しません。

## 既知の制限

- タスクバー上のボタン探索はウィンドウタイトルとの一致に依存します。
- タイトルが空、短すぎる、またはタスクバー表示名と一致しない場合は検知できないことがあります。
- `MINIMIZE_ON_MONITOR_START` が `"true"` の場合、監視中は対象ウィンドウを最小化し、検知時に復元します。
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
