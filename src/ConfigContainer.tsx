import React from "react";
import { FaQuestionCircle } from "react-icons/fa";
import { LineSetupGuide } from "./LineSetupGuide";

interface ConfigContainerProps {
    webhookUrl: string;
    handleWebhookUrlChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    handleSetWebhookUrl: () => void;
    currentWebhookUrl: string;
    lineEnabled: boolean;
    handleLineEnabledChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    lineChannelAccessToken: string;
    handleLineChannelAccessTokenChange: (
        e: React.ChangeEvent<HTMLInputElement>
    ) => void;
    lineTarget: string;
    handleLineTargetChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    handleSetLineConfig: () => void;
    currentLineChannelAccessTokenConfigured: boolean;
    currentLineTarget: string;
    minimizeOnMonitorStart: boolean;
    isMonitoring: boolean;
    handleMinimizeOnMonitorStartChange: (
        e: React.ChangeEvent<HTMLInputElement>
    ) => void;
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
    lineEnabled,
    handleLineEnabledChange,
    lineChannelAccessToken,
    handleLineChannelAccessTokenChange,
    lineTarget,
    handleLineTargetChange,
    handleSetLineConfig,
    currentLineChannelAccessTokenConfigured,
    currentLineTarget,
    minimizeOnMonitorStart,
    isMonitoring,
    handleMinimizeOnMonitorStartChange,
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
    const [isMinimizeHelpOpen, setIsMinimizeHelpOpen] =
        React.useState(false);
    const minimizeHelpId = "minimize-on-monitor-start-help";
    const [activeTab, setActiveTab] = React.useState<"settings" | "line-guide">("settings");
    const showTab = (tab: "settings" | "line-guide") => {
        setActiveTab(tab);
        requestAnimationFrame(() => {
            document.getElementById(`${tab}-tab`)?.focus();
            document.getElementById("settings-tabs")?.scrollIntoView({ block: "start" });
        });
    };

    return (
        <div className="cover-config-container">
            <div className="hover-config-container">
                <div className="header-text">Settings</div>
                <div id="settings-tabs" className="settings-tabs" role="tablist" aria-label="設定とガイド">
                    {(["settings", "line-guide"] as const).map((tab) => (
                        <button key={tab} id={`${tab}-tab`} type="button" role="tab"
                            aria-selected={activeTab === tab} aria-controls={`${tab}-panel`}
                            tabIndex={activeTab === tab ? 0 : -1}
                            onClick={() => showTab(tab)}
                            onKeyDown={(event) => {
                                if (["ArrowLeft", "ArrowRight", "Home", "End"].includes(event.key)) {
                                    event.preventDefault();
                                    showTab(event.key === "Home" ? "settings" : event.key === "End" ? "line-guide" : tab === "settings" ? "line-guide" : "settings");
                                }
                            }}>
                            {tab === "settings" ? "設定" : "LINE 設定ガイド"}
                        </button>
                    ))}
                </div>
                <div id="line-guide-panel" role="tabpanel" aria-labelledby="line-guide-tab" hidden={activeTab !== "line-guide"}>
                    <LineSetupGuide onBack={() => showTab("settings")} />
                </div>
                <div id="settings-panel" role="tabpanel" aria-labelledby="settings-tab" hidden={activeTab !== "settings"}>
                <div className="config-container">
                    <div className="title-toggle-group">
                        <span className="config-title">
                            1. Discord Webhook URL
                        </span>
                    </div>
                    <div className="config-group">
                        <div className="set-config-group">
                            <input
                                type="text"
                                value={webhookUrl}
                                onChange={handleWebhookUrlChange}
                                placeholder="Discord Webhook URL"
                            />
                            <button onClick={handleSetWebhookUrl}>Set</button>
                        </div>
                        <div className="current-value">
                            Current URL: {currentWebhookUrl || "Not configured"}
                        </div>
                    </div>

                    <div className="title-toggle-group">
                        <span className="config-title">2. LINE Bot</span>
                        <div className="toggle-group">
                            <span className="toggle-text-before">Off</span>
                            <label className="toggle-button">
                                <input
                                    type="checkbox"
                                    checked={lineEnabled}
                                    aria-label="LINE Bot を有効にする"
                                    onChange={handleLineEnabledChange}
                                />
                            </label>
                            <span className="toggle-text-after">On</span>
                        </div>
                    </div>
                    <div className="line-config-guide">
                        <button type="button" onClick={() => showTab("line-guide")}>設定方法を見る →</button>
                    </div>
                    <div
                        className={`config-group collapsible-config ${
                            lineEnabled ? "expanded" : "collapsed"
                        }`}
                    >
                        <div className="set-config-group stacked-config-group">
                            <label htmlFor="line-channel-access-token">Channel Access Token（Bot の認証情報）</label>
                            <input
                                id="line-channel-access-token"
                                aria-describedby="line-token-help"
                                type="password"
                                value={lineChannelAccessToken}
                                onChange={handleLineChannelAccessTokenChange}
                                placeholder="LINE Channel Access Token"
                                disabled={!lineEnabled}
                            />
                            <p id="line-token-help" className="line-field-help">{currentLineChannelAccessTokenConfigured ? "設定済み（変更する場合のみ入力）" : "未設定"}</p>
                            <label htmlFor="line-target">Target ID（通知の宛先）</label>
                            <input
                                id="line-target"
                                type="text"
                                value={lineTarget}
                                onChange={handleLineTargetChange}
                                placeholder="LINE Target ID"
                                disabled={!lineEnabled}
                            />
                            <button
                                onClick={handleSetLineConfig}
                                disabled={!lineEnabled}
                            >
                                Set LINE
                            </button>
                        </div>
                        <div className="current-value">
                            Token:{" "}
                            {currentLineChannelAccessTokenConfigured
                                ? "Configured"
                                : "Not configured"}
                        </div>
                        <div className="current-value">
                            Target: {currentLineTarget || "Not configured"}
                        </div>
                    </div>

                    <div className="title-toggle-group">
                        <span className="config-title">
                            3. Minimize target on start
                        </span>
                        <div className="help-popover">
                            <button
                                type="button"
                                className="help-button"
                                aria-label="監視開始時の最小化について"
                                aria-describedby={minimizeHelpId}
                                aria-expanded={isMinimizeHelpOpen}
                                aria-controls={minimizeHelpId}
                                onClick={() =>
                                    setIsMinimizeHelpOpen((prev) => !prev)
                                }
                                onFocus={() => setIsMinimizeHelpOpen(true)}
                                onBlur={() => setIsMinimizeHelpOpen(false)}
                                onMouseEnter={() =>
                                    setIsMinimizeHelpOpen(true)
                                }
                                onMouseLeave={() =>
                                    setIsMinimizeHelpOpen(false)
                                }
                            >
                                <FaQuestionCircle size={18} />
                            </button>
                            <div
                                id={minimizeHelpId}
                                className={`help-content ${
                                    isMinimizeHelpOpen ? "open" : ""
                                }`}
                                role="tooltip"
                                aria-hidden={!isMinimizeHelpOpen}
                            >
                                このアプリはタスクバーアイコンの視覚変化を画像として検知します。対象アプリがアクティブなままだと通知点滅が発生しない場合があるため、既定では監視開始時に対象を最小化します。作業中のウィンドウ状態を変えたくない場合は Off にしてください。
                            </div>
                        </div>
                        <div className="toggle-group">
                            <span className="toggle-text-before">Off</span>
                            <label className="toggle-button">
                                <input
                                    type="checkbox"
                                    checked={minimizeOnMonitorStart}
                                    disabled={isMonitoring}
                                    aria-label="監視開始時に対象ウィンドウを最小化する"
                                    onChange={
                                        handleMinimizeOnMonitorStartChange
                                    }
                                />
                            </label>
                            <span className="toggle-text-after">On</span>
                        </div>
                    </div>
                    <div className="config-group">
                        <div className="current-value">
                            Current behavior:{" "}
                            {minimizeOnMonitorStart
                                ? "Minimize monitored windows when monitoring starts"
                                : "Keep monitored windows as they are"}
                        </div>
                    </div>

                    <div className="title-toggle-group">
                        <span className="config-title">4. Image threshold</span>
                        <div className="toggle-group">
                            <span className="toggle-text-before">Simple</span>
                            <label className="toggle-button">
                                <input
                                    type="checkbox"
                                    checked={isDetailedThreshold}
                                    aria-label="画像しきい値の詳細入力を有効にする"
                                    onChange={toggleThresholdMode}
                                />
                            </label>
                            <span className="toggle-text-after">Detail</span>
                        </div>
                    </div>
                    <div className="config-group">
                        <div className="set-config-group">
                            {isDetailedThreshold ? (
                                <input
                                    type="number"
                                    min="0"
                                    max="1"
                                    step="0.001"
                                    value={threshold}
                                    onChange={handleThresholdTextChange}
                                    placeholder="0.001 - 1.000"
                                />
                            ) : (
                                <div className="config-select">
                                    <select
                                        value={threshold}
                                        onChange={handleThresholdSelectChange}
                                    >
                                        <option value="">-- Select --</option>
                                        <option value="0.020">High precision</option>
                                        <option value="0.040">Standard</option>
                                        <option value="0.060">Low precision</option>
                                    </select>
                                </div>
                            )}
                            <button onClick={handleSetThreshold}>Set</button>
                        </div>
                        <div className="current-value">
                            Current threshold: {currentThreshold}
                        </div>
                    </div>

                    <div className="title-toggle-group">
                        <span className="config-title">5. Interval (ms)</span>
                        <div className="toggle-group">
                            <span className="toggle-text-before">Simple</span>
                            <label className="toggle-button">
                                <input
                                    type="checkbox"
                                    checked={isDetailedInterval}
                                    aria-label="監視間隔の詳細入力を有効にする"
                                    onChange={toggleIntervalMode}
                                />
                            </label>
                            <span className="toggle-text-after">Detail</span>
                        </div>
                    </div>
                    <div className="config-group">
                        <div className="set-config-group">
                            {isDetailedInterval ? (
                                <input
                                    type="number"
                                    min="100"
                                    step="100"
                                    value={interval}
                                    onChange={handleIntervalTextChange}
                                    placeholder="100 or more"
                                />
                            ) : (
                                <div className="config-select">
                                    <select
                                        value={interval}
                                        onChange={handleIntervalSelectChange}
                                    >
                                        <option value="">-- Select --</option>
                                        <option value="500">Fast</option>
                                        <option value="1000">Standard</option>
                                        <option value="3000">Low load</option>
                                    </select>
                                </div>
                            )}
                            <button onClick={handleSetInterval}>Set</button>
                        </div>
                        <div className="current-value">
                            Current interval (ms): {currentInterval}
                        </div>
                    </div>
                </div>
                </div>
            </div>
        </div>
    );
};
