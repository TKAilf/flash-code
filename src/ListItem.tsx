import React from "react";
import { AppInfo } from "./types";
import { warn } from "tauri-plugin-log-api";

interface ListItemProps {
    item: AppInfo;
    isSelected: boolean;
    onClick: () => void;
}

export const ListItem: React.FC<ListItemProps> = ({ item, isSelected, onClick }) => {
    return (
        <li
            className={`scroll-box ${isSelected ? "selected" : ""}`}
            onClick={onClick}
        >
            {item.icon && (
                <img
                    src={`data:image/png;base64,${item.icon}`}
                    alt="icon"
                    className="window-icon"
                    onError={() => warn("アイコンの読み込みに失敗しました。")}
                />
            )}
            <span className="window-title">
                {item.name || "無題のウィンドウ"}
            </span>
        </li>
    );
};
