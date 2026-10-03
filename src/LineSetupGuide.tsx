import React from "react";
import { open } from "@tauri-apps/api/shell";

const officialLinks = [
    { label: "公式の開始手順", url: "https://developers.line.biz/ja/docs/messaging-api/getting-started/" },
    { label: "LINE Developers Console", url: "https://developers.line.biz/console/" },
    { label: "ユーザー ID の取得", url: "https://developers.line.biz/ja/docs/messaging-api/getting-user-ids/" },
    { label: "料金・送信上限", url: "https://developers.line.biz/ja/docs/messaging-api/pricing/" },
];

const steps = [
    { title: "自分の Bot を用意する", location: "LINE Official Account Manager", text: "公式アカウントを作成し、設定から Messaging API を有効にします。その後、LINE Developers Console で対象のチャネルを開きます。" },
    { title: "送信するためのトークンを取得", location: "Messaging API設定 → チャネルアクセストークン（長期）", text: "トークンを発行してコピーします。設定画面の Channel Access Token に貼り付けます。Channel Secret やチャネル ID とは別の値です。" },
    { title: "通知を受け取る自分の ID を取得", location: "チャネル基本設定 → あなたのユーザーID", text: "同じチャネルに表示される ID をコピーし、Target ID に貼り付けます。友だち検索用の LINE ID ではありません。表示されない場合は Business ID に自分の LINE アカウントを連携してください。" },
    { title: "友だち追加して設定を保存", location: "自分の LINE → このアプリの設定", text: "公式アカウントを友だち追加します。このアプリで LINE Bot を On にし、両方の値を入力して Set LINE を押します。監視中にアイコンの変化を検知すると通知します。保存ボタンはテスト送信しません。" },
];

export function LineSetupGuide({ onBack }: { onBack: () => void }) {
    const [linkError, setLinkError] = React.useState("");
    const openOfficialLink = async (url: string) => {
        setLinkError("");
        try {
            await open(url);
        } catch {
            setLinkError(`ブラウザーを開けませんでした。次の URL をブラウザーに貼り付けてください：${url}`);
        }
    };

    return (
        <div className="line-setup-guide">
            <h2>自分の LINE に通知する</h2>
            <p>ご自身の Bot を使う任意機能です。Discord のみでも利用できます。</p>
            <figure className="line-guide-map">
                <figcaption>取得する値と貼り付け先（概念図）</figcaption>
                <div><span>Bot の送信権限<br /><strong>チャネルアクセストークン</strong></span><span aria-hidden="true">→</span><code>Channel Access Token</code></div>
                <div><span>通知を受け取る自分<br /><strong>あなたのユーザーID</strong></span><span aria-hidden="true">→</span><code>Target ID</code></div>
            </figure>
            <ol className="line-guide-steps">
                {steps.map((step, index) => (
                    <li key={step.title}>
                        <span className="line-guide-number" aria-hidden="true">{index + 1}</span>
                        <div><h3>{step.title}</h3><p className="line-guide-location">{step.location}</p><p>{step.text}</p></div>
                    </li>
                ))}
            </ol>
            <details className="line-guide-notes">
                <summary>保存後の表示・利用上の注意</summary>
                <ul>
                    <li>保存後はトークン欄が空になります。設定済みなら再入力は不要です。空欄で保存すると既存のトークンを維持します。</li>
                    <li>宛先を空にして保存すると通知できません。LINE が Off、またはトークンが未設定の場合も送信しません。</li>
                    <li>各利用者が自分の Bot を用意します。トークンやトークン入りの設定ファイルを他人に渡さないでください。</li>
                    <li>自分宛ての通知には Webhook サーバーは不要です。グループ宛ては別途 Webhook からグループ ID を取得する必要があります。このアプリは ID の自動取得には対応していません。</li>
                    <li>通知は LINE の月間送信通数にカウントされます。上限に達すると送信できません。</li>
                </ul>
            </details>
            <nav className="line-guide-links" aria-label="LINE の公式情報">
                {officialLinks.map((link) => <button key={link.url} type="button" onClick={() => void openOfficialLink(link.url)}>{link.label} ↗</button>)}
            </nav>
            <p className="line-guide-caption">公式情報は外部ブラウザーで開きます。画面構成が変わった場合は公式手順をご確認ください。</p>
            {linkError && <p role="alert">{linkError}</p>}
            <div className="line-guide-actions">
                <button type="button" onClick={onBack}>設定に戻る</button>
            </div>
        </div>
    );
}
