import React from "react";
import { FaQuestionCircle } from "react-icons/fa";

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

    return (
        <div className="cover-config-container">
            <div className="hover-config-container">
                <div className="header-text">Settings</div>
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
                    <div
                        className={`config-group collapsible-config ${
                            lineEnabled ? "expanded" : "collapsed"
                        }`}
                    >
                        <div className="set-config-group stacked-config-group">
                            <input
                                type="password"
                                value={lineChannelAccessToken}
                                onChange={handleLineChannelAccessTokenChange}
                                placeholder="LINE Channel Access Token"
                                disabled={!lineEnabled}
                            />
                            <input
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
    );
};
