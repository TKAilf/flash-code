import React from "react";

interface ConfigContainerProps {
    webhookUrl: string;
    handleWebhookUrlChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    handleSetWebhookUrl: () => void;
    currentWebhookUrl: string;
    threshold: string;
    handleThresholdChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    handleSetThreshold: () => void;
    currentThreshold: string;
    interval: string;
    handleIntervalChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    handleSetInterval: () => void;
    currentInterval: string;
}

export const ConfigContainer: React.FC<ConfigContainerProps> = ({
    webhookUrl,
    handleWebhookUrlChange,
    handleSetWebhookUrl,
    currentWebhookUrl,
    threshold,
    handleThresholdChange,
    handleSetThreshold,
    currentThreshold,
    interval,
    handleIntervalChange,
    handleSetInterval,
    currentInterval,
}) => {
    return (
        <div className="config-container">
            <div className="set-config-group">
                <input
                    type="text"
                    value={webhookUrl}
                    onChange={handleWebhookUrlChange}
                    placeholder="Discord Webhook URLを入力"
                />
                <button onClick={handleSetWebhookUrl}>設定</button>
            </div>
            <div className="current-value-group">
                設定中のURL: {currentWebhookUrl}
            </div>
            <div className="set-config-group">
                <input
                    type="text"
                    value={threshold}
                    onChange={handleThresholdChange}
                    placeholder="しきい値を入力"
                />
                <button onClick={handleSetThreshold}>設定</button>
            </div>
            <div className="current-value-group">
                設定中のしきい値: {currentThreshold}
            </div>
            <div className="set-config-group">
                <input
                    type="text"
                    value={interval}
                    onChange={handleIntervalChange}
                    placeholder="監視間隔（ms）を入力"
                />
                <button onClick={handleSetInterval}>設定</button>
            </div>
            <div className="current-value-group">
                設定中の監視間隔（ms）: {currentInterval}
            </div>
        </div>
    );
};
