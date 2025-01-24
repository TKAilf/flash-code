import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import "./mystyle.css";
import { info, error, attachConsole } from "tauri-plugin-log-api";
import ListSection from "./ListSection";
import MoveButtons from "./MoveButtons";
import PrimaryActionButtons from "./PrimaryActionButtons";
import { AppInfo } from "./types";
import { dialog } from "@tauri-apps/api";

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
    const [settingWebhookUrl, setSettingWebhookUrl] = useState<string>("");

    useEffect(() => {
        attachConsole();
        info("fetchWindows呼び出し開始");
        fetchWindows();
    }, []);

    const fetchWindows = async () => {
        try {
            const windows: AppInfo[] = await invoke("get_taskbar_apps");
            const url: string = await invoke("get_webhook_url");
            setAvailableItems(windows);
            setSettingWebhookUrl(url);
        } catch (e) {
            error(`get_taskbar_apps呼び出しでエラーが発生しました: ${e}`);
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

    const handleAddWebhookUrl = async () => {
        try {
            await invoke("update_webhook_url", { url: webhookUrl });
            setSettingWebhookUrl(webhookUrl);
        } catch (e) {
            error(`updated_webhook_url呼び出しでエラーが起きました: ${e}`);
        }
    };

    return (
        <div className="container">
            <h1>タスクバーの監視対象を選択</h1>
            <div className="list-container row">
                <ListSection
                    title="監視可能なタスク"
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
                    title="監視対象とするタスク"
                    items={monitoredItems}
                    selectedItem={selectedMonitoredItem}
                    onItemClick={handleMonitoredItemClick}
                />
            </div>
            <div className="webhook-container">
                <div className="set-webhook-url-group">
                    <input
                        type="text"
                        value={webhookUrl}
                        onChange={handleWebhookUrlChange}
                        placeholder="Discord Webhook URLを入力"
                    />
                    <button onClick={handleAddWebhookUrl}>設定</button>
                </div>
                <div className="setting-webhook-url-group">
                    設定中のURL: {settingWebhookUrl}
                </div>
            </div>
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
