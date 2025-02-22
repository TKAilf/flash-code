import React from "react";

interface PrimaryActionButtonsProps {
    onMonitorAll: () => void;
    onMonitor: () => void;
    onStopMonitoring: () => void;
    onClose: () => void;
}

export const PrimaryActionButtons: React.FC<PrimaryActionButtonsProps> = ({
    onMonitorAll,
    onMonitor,
    onStopMonitoring,
    onClose,
}) => {
    return (
        <div className="primary-button-container">
            <button
                className="primary-action-button button-monitor-all"
                onClick={onMonitorAll}
            >
                全てを監視
            </button>
            <button
                className="primary-action-button button-monitor"
                onClick={onMonitor}
            >
                監視
            </button>
            <button
                className="primary-action-button button-stop-monitoring"
                onClick={onStopMonitoring}
            >
                監視停止
            </button>
            <button
                className="primary-action-button button-close"
                onClick={onClose}
            >
                閉じる
            </button>
        </div>
    );
};
