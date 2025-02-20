import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { info, error, attachConsole } from "tauri-plugin-log-api";
import { dialog } from "@tauri-apps/api";
import ListSection from "./ListSection";
import MoveButtons from "./MoveButtons";
import PrimaryActionButtons from "./PrimaryActionButtons";
import { AppInfo } from "./types";
import { ConfigContainer } from "./ConfigContainer";
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
    const [currentWebhookUrl, setCurrentWebhookUrl] = useState<string>("");
    const [currentThreshold, setCurrentThreshold] = useState<string>("");
    const [currentInterval, setCurrentInterval] = useState<string>("");

    useEffect(() => {
        attachConsole();
        info("fetchWindows呼び出し開始");
        fetchWindows();
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
            error(`get_taskbar_apps呼び出しでエラーが発生しました: ${e}`);
        }
    };

    const fetchGetConfig = async () => {
        try {
            const url: string = await invoke("get_webhook_url");
            const threshold: string = await invoke("get_threshold");
            const interval: string = await invoke("get_interval");
            setCurrentWebhookUrl(url);
            setCurrentThreshold(threshold);
            setCurrentInterval(interval);
        } catch (e) {
            error(`get_config呼び出しでエラーが発生しました: ${e}`);
        }
    };

    const refresh_list = async () => {
        try {
            const confirmed = await dialog.ask(
                "監視を停止し、リストを最新状態へ更新します",
                { title: "警告" }
            );
            if (!confirmed) return;
            await invoke("stop_monitoring");
            setMonitoredItems([]);
            fetchWindows();
        } catch (e) {
            error(`stop_monitoring呼び出しでエラーが起きました: ${e}`);
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

    const startMonitoring = async (apps: AppInfo[]) => {
        try {
            await invoke("start_monitoring", { apps });
            await dialog.message("監視を開始しました", { title: "監視開始" });
        } catch (e) {
            error(`start_monitoring呼び出しでエラーが起きました: ${e}`);
        }
    };

    const handleMonitorAll = async () => {
        const newAvailableItems = availableItems.filter((item) => {
            setMonitoredItems((prevMonitoredItems) => [
                ...prevMonitoredItems,
                item,
            ]);
            return false; // すべてのアイテムを削除するためにfalseを返す
        });

        setAvailableItems(newAvailableItems);

        await startMonitoring(monitoredItems);
    };

    const handleMonitor = async () => {
        if (monitoredItems.length === 0) {
            await dialog.message("監視対象がありません", { title: "情報" });
            return;
        }
        await startMonitoring(monitoredItems);
    };

    const handleStopMonitoring = async () => {
        try {
            await invoke("stop_monitoring");
            await dialog.message("監視を停止しました", { title: "監視停止" });
        } catch (e) {
            error(`stop_monitoring呼び出しでエラーが起きました: ${e}`);
        }
    };

    const handleClose = async () => {
        try {
            await invoke("stop_monitoring");
            window.close();
        } catch (e) {
            error(`close呼び出しでエラーが起きました: ${e}`);
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

    const handleSetWebhookUrl = async () => {
        try {
            await invoke("update_webhook_url", { url: webhookUrl });
            setCurrentWebhookUrl(webhookUrl);
        } catch (e) {
            error(`updated_webhook_url呼び出しでエラーが起きました: ${e}`);
        }
    };

    const handleSetThreshold = async () => {
        try {
            await invoke("update_threshold", { threshold: threshold });
            setCurrentThreshold(threshold);
        } catch (e) {
            error(`update_threshold呼び出しでエラーが起きました: ${e}`);
        }
    };

    const handleSetInterval = async () => {
        try {
            await invoke("update_interval", { interval: interval });
            setCurrentInterval(interval);
        } catch (e) {
            error(`update_interval呼び出しでエラーが起きました: ${e}`);
        }
    };

    return (
        <div className="container">
            <h1>アプリのビジュアル監視</h1>
            <div className="list-container">
                <ListSection
                    title="監視可能"
                    items={availableItems}
                    selectedItem={selectedAvailableItem}
                    onItemClick={handleAvailableItemClick}
                />
                <MoveButtons
                    onAddClick={moveToMonitored}
                    onRemoveClick={moveToAvailable}
                    onrefreshClick={refresh_list}
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
            <PrimaryActionButtons
                onMonitorAll={handleMonitorAll}
                onMonitor={handleMonitor}
                onStopMonitoring={handleStopMonitoring}
                onClose={handleClose}
            />
        </div>
    );
}

export default App;
