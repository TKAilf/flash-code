import React from "react";

interface ConfigContainerProps {
    webhookUrl: string;
    handleWebhookUrlChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    handleSetWebhookUrl: () => void;
    currentWebhookUrl: string;
    threshold: string;
    handleThresholdTextChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    handleThresholdSelectChange: (
        e: React.ChangeEvent<HTMLSelectElement>
    ) => void;
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
    handleThresholdTextChange,
    handleThresholdSelectChange,
    handleSetThreshold,
    currentThreshold,
    interval,
    handleIntervalChange,
    handleSetInterval,
    currentInterval,
}) => {
    const [isDetailedThreshold, setIsDetailedThreshold] = React.useState(false);
    const toggleThresholdMode = () => setIsDetailedThreshold((prev) => !prev);

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
            <div className="toggle-group">
                <span className="toggle-description">2. しきい値のモード設定</span>
                <span className="toggle-text-before">簡易</span>
                <label className="toggle-button">
                    <input
                        type="checkbox"
                        checked={isDetailedThreshold}
                        onChange={toggleThresholdMode}
                    />
                </label>
                <span className="toggle-text-after">詳細</span>
            </div>
            <div className="set-config-group">
                {isDetailedThreshold ? (
                    <input
                        value={threshold}
                        onChange={(e) =>
                            handleThresholdTextChange(
                                e as unknown as React.ChangeEvent<HTMLInputElement>
                            )
                        }
                        placeholder="しきい値を入力"
                    />
                ) : (
                    <select
                        value={threshold}
                        onChange={handleThresholdSelectChange}
                    >
                        <option value="0.75">高精度</option>
                        <option value="0.50">自動（推奨）</option>
                        <option value="0.25">低精度</option>
                    </select>
                )}
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
