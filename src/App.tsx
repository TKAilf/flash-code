import React, { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog } from "@tauri-apps/api";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { ConfigContainer } from "./ConfigContainer";
import { EyeAnimation } from "./EyeAnimation";
import { ListSection } from "./ListSection";
import { MoveButtons } from "./MoveButtons";
import { PrimaryActionButtons } from "./PrimaryActionButtons";
import { AppInfo } from "./types";
import { logFrontend } from "./logger";
import "./App.css";
import "./mystyle.css";

function App() {
    const [availableItems, setAvailableItems] = useState<AppInfo[]>([]);
    const [monitoredItems, setMonitoredItems] = useState<AppInfo[]>([]);
    const [selectedAvailableItem, setSelectedAvailableItem] = useState<{
        item: AppInfo | null;
        index: number;
    }>({ item: null, index: -1 });
    const [selectedMonitoredItem, setSelectedMonitoredItem] = useState<{
        item: AppInfo | null;
        index: number;
    }>({ item: null, index: -1 });
    const [webhookUrl, setWebhookUrl] = useState<string>("");
    const [threshold, setThreshold] = useState<string>("");
    const [interval, setInterval] = useState<string>("");
    const [lineChannelAccessToken, setLineChannelAccessToken] =
        useState<string>("");
    const [lineTarget, setLineTarget] = useState<string>("");
    const [currentWebhookUrl, setCurrentWebhookUrl] = useState<string>("");
    const [currentThreshold, setCurrentThreshold] = useState<string>("");
    const [currentInterval, setCurrentInterval] = useState<string>("");
    const [lineEnabled, setLineEnabled] = useState(false);
    const [minimizeOnMonitorStart, setMinimizeOnMonitorStart] = useState(true);
    const [
        currentLineChannelAccessTokenConfigured,
        setCurrentLineChannelAccessTokenConfigured,
    ] = useState(false);
    const [currentLineTarget, setCurrentLineTarget] = useState<string>("");
    const [isMonitoring, setIsMonitoring] = useState(false);
    const monitoredItemsRef = useRef<AppInfo[]>([]);

    useEffect(() => {
        monitoredItemsRef.current = monitoredItems;
    }, [monitoredItems]);

    useEffect(() => {
        (async () => {
            logFrontend("info", "fetchWindows called");
            try {
                await fetchWindows();
            } catch (e) {
                logFrontend("error", `fetchWindows failed: ${e}`);
            }
        })();
    }, []);

    const fetchWindows = async () => {
        await fetchGetTaskbarApps();
        await fetchGetConfig();
    };

    const fetchGetTaskbarApps = async () => {
        try {
            const windows: AppInfo[] = await invoke("get_taskbar_apps");
            setAvailableItems(windows);
        } catch (e) {
            logFrontend("error", `get_taskbar_apps failed: ${e}`);
        }
    };

    const fetchGetConfig = async () => {
        try {
            const url: string = await invoke("get_webhook_url");
            const threshold: string = await invoke("get_threshold");
            const interval: string = await invoke("get_interval");
            const lineEnabledValue: string = await invoke("get_line_enabled");
            const lineTokenConfigured: boolean = await invoke(
                "get_line_channel_access_token_configured"
            );
            const lineTargetValue: string = await invoke("get_line_target");
            const minimizeOnStartValue: boolean = await invoke(
                "get_minimize_on_monitor_start"
            );
            setCurrentWebhookUrl(url);
            setCurrentThreshold(threshold);
            setCurrentInterval(interval);
            setLineEnabled(lineEnabledValue === "true");
            setMinimizeOnMonitorStart(minimizeOnStartValue);
            setCurrentLineChannelAccessTokenConfigured(lineTokenConfigured);
            setCurrentLineTarget(lineTargetValue);
            setLineTarget(lineTargetValue);
        } catch (e) {
            logFrontend("error", `get_config failed: ${e}`);
        }
    };

    useEffect(() => {
        let unlisten: UnlistenFn | undefined;
        (async () => {
            try {
                unlisten = await listen("monitoring_stopped", async () => {
                    logFrontend("info", "monitoring_stopped event received");
                    await invoke("stop_monitoring", {
                        apps: [...monitoredItemsRef.current],
                    });
                    setIsMonitoring(false);
                });
            } catch (e) {
                logFrontend("error", `failed to register listener: ${e}`);
            }
        })();

        return () => {
            if (unlisten) {
                try {
                    unlisten();
                } catch (e) {
                    logFrontend("error", `failed to remove listener: ${e}`);
                }
            }
        };
    }, []);

    const refreshList = async () => {
        try {
            const confirmed = await dialog.ask(
                "監視を停止し、リストを最新の状態に更新します。",
                { title: "確認" }
            );
            if (!confirmed) return;
            await invoke("stop_monitoring", { apps: [...monitoredItems] });
            setIsMonitoring(false);
            setMonitoredItems([]);
            initSelectedAvailableItem();
            initSelectedMonitoredItem();
            await fetchWindows();
        } catch (e) {
            logFrontend("error", `refresh list failed: ${e}`);
        }
    };

    const handleMove = (
        fromList: AppInfo[],
        toList: AppInfo[],
        selected: { item: AppInfo | null; index: number },
        setFromList: React.Dispatch<React.SetStateAction<AppInfo[]>>,
        setToList: React.Dispatch<React.SetStateAction<AppInfo[]>>,
        initFromSelected: () => void,
        initToSelected: () => void
    ) => {
        if (fromList.length === 0 || selected.index < 0 || !selected.item)
            return;

        const item = selected.item;
        const updatedFromList = fromList.filter((_, i) => i !== selected.index);
        const updatedToList = [...toList, item];

        setFromList(updatedFromList);
        setToList(updatedToList);

        initFromSelected();
        initToSelected();
    };

    const handleAvailableItemClick = (item: AppInfo, index: number) => {
        setSelectedAvailableItem({ item, index });
        initSelectedMonitoredItem();
    };

    const handleMonitoredItemClick = (item: AppInfo, index: number) => {
        setSelectedMonitoredItem({ item, index });
        initSelectedAvailableItem();
    };

    const initSelectedAvailableItem = () => {
        setSelectedAvailableItem({ item: null, index: -1 });
    };

    const initSelectedMonitoredItem = () => {
        setSelectedMonitoredItem({ item: null, index: -1 });
    };

    const moveToMonitored = async () => {
        handleMove(
            availableItems,
            monitoredItems,
            selectedAvailableItem,
            setAvailableItems,
            setMonitoredItems,
            initSelectedAvailableItem,
            initSelectedMonitoredItem
        );
    };

    const moveToAvailable = async () => {
        handleMove(
            monitoredItems,
            availableItems,
            selectedMonitoredItem,
            setMonitoredItems,
            setAvailableItems,
            initSelectedMonitoredItem,
            initSelectedAvailableItem
        );
    };

    const startMonitoring = async (apps = monitoredItems) => {
        try {
            await invoke("start_monitoring", { apps: [...apps] });
            setIsMonitoring(true);
        } catch (e) {
            logFrontend("error", `start_monitoring failed: ${e}`);
        }
    };

    const handleMonitorAll = async () => {
        const nextMonitoredItems = [...monitoredItems, ...availableItems];
        setMonitoredItems(nextMonitoredItems);
        setAvailableItems([]);
        initSelectedAvailableItem();
        initSelectedMonitoredItem();

        await startMonitoring(nextMonitoredItems);
    };

    const handleMonitor = async () => {
        if (monitoredItems.length === 0) {
            await dialog.message("監視対象がありません。", { title: "情報" });
            return;
        }
        await startMonitoring();
    };

    const handleStopMonitoring = async () => {
        try {
            await invoke("stop_monitoring", { apps: [...monitoredItems] });
            setIsMonitoring(false);
        } catch (e) {
            logFrontend("error", `stop_monitoring failed: ${e}`);
        }
    };

    const handleClose = async () => {
        try {
            await invoke("stop_monitoring", { apps: [...monitoredItems] });
            setIsMonitoring(false);
            window.close();
        } catch (e) {
            logFrontend("error", `close failed: ${e}`);
        }
    };

    const handleWebhookUrlChange = (
        event: React.ChangeEvent<HTMLInputElement>
    ) => {
        setWebhookUrl(event.target.value);
    };

    const handleThresholdTextChange = (
        event: React.ChangeEvent<HTMLInputElement>
    ) => {
        setThreshold(event.target.value);
    };

    const handleThresholdSelectChange = (
        event: React.ChangeEvent<HTMLSelectElement>
    ) => {
        setThreshold(event.target.value);
    };

    const handleIntervalTextChange = (
        event: React.ChangeEvent<HTMLInputElement>
    ) => {
        setInterval(event.target.value);
    };

    const handleIntervalSelectChange = (
        event: React.ChangeEvent<HTMLSelectElement>
    ) => {
        setInterval(event.target.value);
    };

    const handleLineEnabledChange = async (
        event: React.ChangeEvent<HTMLInputElement>
    ) => {
        const enabled = event.target.checked;
        try {
            await invoke("update_line_enabled", {
                enabled: enabled ? "true" : "false",
            });
            setLineEnabled(enabled);
        } catch (e) {
            logFrontend("error", `update_line_enabled failed: ${e}`);
        }
    };

    const handleMinimizeOnMonitorStartChange = async (
        event: React.ChangeEvent<HTMLInputElement>
    ) => {
        const enabled = event.target.checked;
        try {
            await invoke("update_minimize_on_monitor_start", {
                enabled,
            });
            setMinimizeOnMonitorStart(enabled);
        } catch (e) {
            logFrontend(
                "error",
                `update_minimize_on_monitor_start failed: ${e}`
            );
        }
    };

    const handleLineChannelAccessTokenChange = (
        event: React.ChangeEvent<HTMLInputElement>
    ) => {
        setLineChannelAccessToken(event.target.value);
    };

    const handleLineTargetChange = (
        event: React.ChangeEvent<HTMLInputElement>
    ) => {
        setLineTarget(event.target.value);
    };

    const handleSetWebhookUrl = async () => {
        try {
            await invoke("update_webhook_url", { url: webhookUrl });
            setCurrentWebhookUrl(webhookUrl);
        } catch (e) {
            logFrontend("error", `update_webhook_url failed: ${e}`);
        }
    };

    const handleSetThreshold = async () => {
        try {
            await invoke("update_threshold", { threshold });
            setCurrentThreshold(threshold);
        } catch (e) {
            logFrontend("error", `update_threshold failed: ${e}`);
        }
    };

    const handleSetInterval = async () => {
        try {
            await invoke("update_interval", { interval });
            setCurrentInterval(interval);
        } catch (e) {
            logFrontend("error", `update_interval failed: ${e}`);
        }
    };

    const handleSetLineConfig = async () => {
        try {
            const trimmedLineChannelAccessToken =
                lineChannelAccessToken.trim();
            const trimmedLineTarget = lineTarget.trim();

            if (trimmedLineChannelAccessToken !== "") {
                await invoke("update_line_channel_access_token", {
                    token: trimmedLineChannelAccessToken,
                });
                setCurrentLineChannelAccessTokenConfigured(true);
                setLineChannelAccessToken("");
            }
            await invoke("update_line_target", { target: trimmedLineTarget });
            setLineTarget(trimmedLineTarget);
            setCurrentLineTarget(trimmedLineTarget);
        } catch (e) {
            logFrontend("error", `update_line_config failed: ${e}`);
        }
    };

    return (
        <div className="container">
            <div className="page">
                <h1 className="h1">タスクバーアイコン監視</h1>
                <div className="list-container">
                    <ListSection
                        title="監視候補"
                        items={availableItems}
                        selectedItem={selectedAvailableItem}
                        onItemClick={handleAvailableItemClick}
                    />
                    <MoveButtons
                        onAddClick={moveToMonitored}
                        onRemoveClick={moveToAvailable}
                        onrefreshClick={refreshList}
                    />
                    <ListSection
                        title="監視対象"
                        items={monitoredItems}
                        selectedItem={selectedMonitoredItem}
                        onItemClick={handleMonitoredItemClick}
                    />
                </div>
                <ConfigContainer
                    webhookUrl={webhookUrl}
                    handleWebhookUrlChange={handleWebhookUrlChange}
                    handleSetWebhookUrl={handleSetWebhookUrl}
                    currentWebhookUrl={currentWebhookUrl}
                    lineEnabled={lineEnabled}
                    handleLineEnabledChange={handleLineEnabledChange}
                    lineChannelAccessToken={lineChannelAccessToken}
                    handleLineChannelAccessTokenChange={
                        handleLineChannelAccessTokenChange
                    }
                    lineTarget={lineTarget}
                    handleLineTargetChange={handleLineTargetChange}
                    handleSetLineConfig={handleSetLineConfig}
                    currentLineChannelAccessTokenConfigured={
                        currentLineChannelAccessTokenConfigured
                    }
                    currentLineTarget={currentLineTarget}
                    minimizeOnMonitorStart={minimizeOnMonitorStart}
                    isMonitoring={isMonitoring}
                    handleMinimizeOnMonitorStartChange={
                        handleMinimizeOnMonitorStartChange
                    }
                    threshold={threshold}
                    handleThresholdTextChange={handleThresholdTextChange}
                    handleThresholdSelectChange={handleThresholdSelectChange}
                    handleSetThreshold={handleSetThreshold}
                    currentThreshold={currentThreshold}
                    interval={interval}
                    handleIntervalTextChange={handleIntervalTextChange}
                    handleIntervalSelectChange={handleIntervalSelectChange}
                    handleSetInterval={handleSetInterval}
                    currentInterval={currentInterval}
                />
                <EyeAnimation isMonitoring={isMonitoring} />
                <PrimaryActionButtons
                    onMonitorAll={handleMonitorAll}
                    onMonitor={handleMonitor}
                    onStopMonitoring={handleStopMonitoring}
                    onClose={handleClose}
                />
            </div>
        </div>
    );
}

export default App;
