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
    handleIntervalTextChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    handleIntervalSelectChange: (
        e: React.ChangeEvent<HTMLSelectElement>
    ) => void;
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
    handleIntervalTextChange,
    handleIntervalSelectChange,
    handleSetInterval,
    currentInterval,
}) => {
    const [isDetailedThreshold, setIsDetailedThreshold] = React.useState(false);
    const toggleThresholdMode = () => setIsDetailedThreshold((prev) => !prev);
    const [isDetailedInterval, setIsDetailedInterval] = React.useState(false);
    const toggleIntervalMode = () => setIsDetailedInterval((prev) => !prev);

    return (
        <div className="cover-config-container">
            <div className="hover-config-container">
                <div className="header-text">各種設定項目</div>
                <div className="config-container">
                    <div className="title-toggle-group">
                        <span className="config-title">
                            1. Webhook URLの設定
                        </span>
                    </div>
                    <div className="config-group">
                        <div className="set-config-group">
                            <input
                                type="text"
                                value={webhookUrl}
                                onChange={handleWebhookUrlChange}
                                placeholder="Discord Webhook URLを入力"
                            />
                            <button onClick={handleSetWebhookUrl}>設定</button>
                        </div>
                        <div className="current-value">
                            設定中のURL: {currentWebhookUrl}
                        </div>
                    </div>
                    <div className="title-toggle-group">
                        <span className="config-title">2. しきい値の設定</span>
                        <div className="toggle-group">
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
                    </div>
                    <div className="config-group">
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
                                <div className="config-select">
                                    <select
                                        value={threshold}
                                        onChange={handleThresholdSelectChange}
                                    >
                                        <option value="">-- 選択してください --</option>
                                        <option value="0.020">高精度</option>
                                        <option value="0.050">
                                            自動（推奨）
                                        </option>
                                        <option value="0.100">低精度</option>
                                    </select>
                                </div>
                            )}
                            <button onClick={handleSetThreshold}>設定</button>
                        </div>
                        <div className="current-value">
                            設定中のしきい値: {currentThreshold}
                        </div>
                    </div>
                    <div className="title-toggle-group">
                        <span className="config-title">
                            3. 監視間隔（ms）の設定
                        </span>
                        <div className="toggle-group">
                            <span className="toggle-text-before">簡易</span>
                            <label className="toggle-button">
                                <input
                                    type="checkbox"
                                    checked={isDetailedInterval}
                                    onChange={toggleIntervalMode}
                                />
                            </label>
                            <span className="toggle-text-after">詳細</span>
                        </div>
                    </div>
                    <div className="config-group">
                        <div className="set-config-group">
                            {isDetailedInterval ? (
                                <input
                                    value={interval}
                                    onChange={(e) =>
                                        handleIntervalTextChange(
                                            e as unknown as React.ChangeEvent<HTMLInputElement>
                                        )
                                    }
                                    placeholder="監視間隔（ms）を入力"
                                />
                            ) : (
                                <div className="config-select">
                                    <select
                                        value={interval}
                                        onChange={handleIntervalSelectChange}
                                    >
                                        <option value="">-- 選択してください --</option>
                                        <option value="1000">
                                            高速（高負荷）
                                        </option>
                                        <option value="3000">
                                            自動（推奨）
                                        </option>
                                        <option value="5000">
                                            低速（低負荷）
                                        </option>
                                    </select>
                                </div>
                            )}
                            <button onClick={handleSetInterval}>設定</button>
                        </div>
                        <div className="current-value">
                            設定中の監視間隔（ms）: {currentInterval}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};
