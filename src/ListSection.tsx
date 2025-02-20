import React from "react";
import ListItem from "./ListItem";
import { AppInfo } from "./types";

interface ListSectionProps {
    title: string;
    items: AppInfo[];
    selectedItem: { item: AppInfo | null; index: number };
    onItemClick: (item: AppInfo, index: number) => void;
}

const ListSection: React.FC<ListSectionProps> = ({
    title,
    items,
    selectedItem,
    onItemClick,
}) => {
    return (
        <div className="list-section">
            <div className="header-text">{title}</div>
            <ul className="list-view scroll-box">
                {items.map((item, index) => (
                    <ListItem
                        key={index}
                        item={item}
                        isSelected={item === selectedItem.item}
                        onClick={() => onItemClick(item, index)}
                    />
                ))}
            </ul>
        </div>
    );
};

export default ListSection;
