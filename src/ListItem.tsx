import React from "react";
import { AppInfo } from "./types";
import { logFrontend } from "./logger";

interface ListItemProps {
    item: AppInfo;
    isSelected: boolean;
    onClick: () => void;
}

export const ListItem: React.FC<ListItemProps> = ({
    item,
    isSelected,
    onClick,
}) => {
    return (
        <li
            className={`scroll-box ${isSelected ? "selected" : ""}`}
            onClick={onClick}
        >
            {item.icon && (
                <img
                    src={`data:image/png;base64,${item.icon}`}
                    alt=""
                    className="window-icon"
                    onError={() => logFrontend("warn", "failed to load icon")}
                />
            )}
            <span className="window-title">
                {item.name || "無題のウィンドウ"}
            </span>
        </li>
    );
};
