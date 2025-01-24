# タスクバーアイコン点滅検知アプリ
```Note
The English version of this document was generated by ChatGPT.  
Due to the nature of AI-generated translations, there may be differences in nuance compared to the original Japanese text.  
Thank you for your understanding.
```

このアプリケーションは、タスクバーに表示されている他のアプリケーションのアイコンを監視し、点滅などのビジュアル変化を検知した際に Discord Bot を通じて通知を行うツールです。バックエンドに Rust、フロントエンドに React を使用した Tauri フレームワークで開発されています。

## 特徴

-   タスクバーアイコンの監視: 指定したアプリケーションのタスクバーアイコンをリアルタイムで監視します
-   点滅の検知: アイコンのビジュアル変化を検知し、点滅状態を判定します
-   Discord 通知: 検知した変化を Discord Bot にメッセージとして送信し、ユーザーに即時通知します
-   直感的な UI: 2 つのリストビューと「監視」や「停止」ボタン等で簡単に操作できます
-   軽量設計: Rust と React による効率的な設計で、システムリソースへの負荷を最小限に抑えます

## 動作環境

-   対応 OS: Windows 10 以降
-   開発言語:
    -   バックエンド: Rust 最新の Stable バージョンを推奨
    -   フロントエンド: React
-   フレームワーク: Tauri
-   必要権限: ユーザー権限（管理者権限は不要）

## インストール

1. リポジトリのクローン

    git clone https://github.com/TKAilf/flash-code.git

2. ディレクトリの移動

    cd flash-code

3. 依存関係のインストール

    - バックエンド

        cd src-tauri
        cargo install

    - フロントエンド

        cd ../
        npm install

4. アプリケーションのビルドと実行

    npm run tauri dev

5. Discord Bot の設定

    Discord Bot の Webhook URL を設定します。
    サービス内の Discord Bot URL 設定用テキストエリアに、Webhook URL を入力し、設定ボタンを押下してください。

## 使い方

1. アプリケーションの起動

    上記のコマンドでアプリケーションを起動します。

2. 監視対象の設定

    左側のリストビューに現在のタスクバーに表示されているアプリケーションが一覧表示されます。監視したいアプリケーションを選択し、右側のリストビューに移動させます。

3. Discord Bot の設定

    Discord Bot の Webhook URL を設定します。
    サービス内の Discord Bot URL 設定用テキストエリアに、Webhook URL を入力し、設定ボタンを押下してください。

4. 監視の開始

    「全てを監視」または「監視」ボタンを押下すると、右側のリストビューにあるアプリケーションの監視が開始されます。

5. 通知の確認

    タスクバーアイコンの変化が検知されると、設定した Discord チャンネルにメッセージが送信されます。

6. 監視の停止と終了

    「監視停止」または「閉じる」ボタンを押下すると、すべての監視が停止されます。

## 設定

-   監視間隔の調整

    サービス内の「監視時間」 の値を変更することで、アイコンのキャプチャ間隔を調整できます（ミリ秒単位）。

-   しきい値の設定

    サービス内の「画像のしきい値」 を調整することで、検知精度を変更できます。

## 注意事項

-   プライバシーへの配慮

    本アプリケーションは、タスクバーアイコンのビジュアル変化のみを監視します。ユーザーのプライバシーに関わる情報は一切収集・送信しません。

-   システム負荷

    定期的な画像キャプチャと比較を行うため、CPU 使用率が上がる場合があります。負荷が高いと感じる場合は、monitor_interval の値を大きくしてください。

-   セキュリティソフトとの競合

    一部のセキュリティソフトウェアは、本アプリケーションの動作を制限する可能性があります。その場合は、アプリケーションをセキュリティソフトの除外リストに追加してください。

## ライセンス

このアプリケーションは MIT ライセンスの下で提供されています。詳細は LICENSE ファイルをご覧ください。

## 貢献

バグ報告や機能提案は、Issue Tracker までお願いします。プルリクエストも歓迎します。

## 開発者向け情報

### 必要な環境

-   バックエンド
    -   Rust 最新の Stable バージョン
    -   Cargo
-   フロントエンド
    -   Node.js 20 以上
    -   npm または yarn
-   その他
    -   Tauri CLI
        cargo install tauri-cli

### ビルド手順

1. リポジトリをクローン

    git clone https://github.com/TKAilf/flash-code.git

2. 依存関係のインストール

    cd flash-code
    npm install
    cd src-tauri
    cargo build

3. アプリケーションのビルド

    npm run tauri build

4. 実行ファイルの確認

    ビルドが成功すると、src-tauri/target/release フォルダに実行ファイルが生成されます。

## 連絡先

質問やお問い合わせは、以下のメールアドレスまでお願いします。

-   Email: ktoshiki0511@yahoo.co.jp

---

最終更新日: 2025 年 01 月 24 日


--------------------------------------------------------------------------------



# Taskbar Icon Flash Detector(Translated using ChatGPT)

This application monitors the icons of other applications displayed on the taskbar and notifies you via a Discord Bot when it detects any visual changes such as flashing. It is built using Rust for the backend and React for the frontend, within the Tauri framework.

## Features

- Taskbar Icon Monitoring  
  Monitors the icons of specified applications on the taskbar in real time.
- Flash Detection  
  Detects visual changes in the icons and determines if they are in a flashing state.
- Discord Notifications  
  Sends detected changes as messages to a Discord Bot, providing immediate alerts to the user.
- Intuitive UI  
  Offers two list views and simple buttons such as “Monitor” and “Stop” for easy operation.
- Lightweight Design  
  Built with Rust and React for efficient performance, minimizing system resource usage.

## System Requirements

- Supported OS: Windows 10 or later
- Languages:
  - Backend: Rust (latest stable version recommended)
  - Frontend: React
- Framework: Tauri
- Required Permissions: User privileges (no administrator rights required)

## Installation

1. Clone the repository  
    git clone https://github.com/TKAilf/flash-code.git

2. Change to the project directory  
    cd flash-code

3. Install dependencies

   - Backend  
       cd src-tauri  
       cargo install

   - Frontend  
       cd ../  
       npm install

4. Build and run the application  
    npm run tauri dev

5. Configure the Discord Bot  
   Set the Webhook URL for the Discord Bot. In the service, enter your Webhook URL in the text area for setting the Discord Bot URL and click the settings button.

## Usage

1. Start the application  
   Run the command above to launch the application.

2. Set up monitoring targets  
   The left list view displays the applications currently shown on the taskbar. Select the applications you want to monitor and move them to the right list view.

3. Configure the Discord Bot  
   Enter your Discord Bot Webhook URL in the text area for setting the Discord Bot URL in the service, then click the settings button.

4. Begin monitoring  
   Click the “Monitor All” or “Monitor” button to start monitoring the applications in the right list view.

5. Check notifications  
   When any change in the taskbar icon is detected, a message will be sent to the configured Discord channel.

6. Stop monitoring and exit  
   Click the “Stop Monitoring” or “Close” button to stop all monitoring.

## Settings

- Adjust Monitoring Interval  
  You can change the value in the “Monitoring Time” field within the service to adjust the icon capture interval in milliseconds.
- Threshold Settings  
  You can modify the “Image Threshold” within the service to change the detection accuracy.

## Notes

- Privacy Considerations  
  This application only monitors the visual changes of taskbar icons. It does not collect or transmit any personal information.
- System Load  
  Because it performs periodic image capturing and comparison, CPU usage may increase. If you notice high load, try increasing the monitor_interval value.
- Conflicts with Security Software  
  Some security software may restrict this application's operation. If that happens, add the application to your security software's exclusion list.

## License

This application is provided under the MIT License. For more details, please see the LICENSE file.

## Contributing

Please report bugs or suggest new features in the Issue Tracker. Pull requests are also welcome.

## Developer Information

### Required Environment

- Backend
  - Rust (latest stable version)
  - Cargo
- Frontend
  - Node.js 20 or later
  - npm or yarn
- Other
  - Tauri CLI  
    cargo install tauri-cli

### Build Procedure

1. Clone the repository  
    git clone https://github.com/TKAilf/flash-code.git

2. Install dependencies  
    cd flash-code  
    npm install  
    cd src-tauri  
    cargo build

3. Build the application  
    npm run tauri build

4. Check the executable  
    After a successful build, the executable file will be generated in src-tauri/target/release.

## Contact

For questions or inquiries, please email:

- Email: ktoshiki0511@yahoo.co.jp

Last Updated: January 24, 2025
