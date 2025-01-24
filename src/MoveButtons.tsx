import React from "react";
import { FaPlus, FaMinus, FaSync } from "react-icons/fa";

interface MoveButtonsProps {
    onAddClick: () => void;
    onRemoveClick: () => void;
    onrefreshClick: () => void;
}

const MoveButtons: React.FC<MoveButtonsProps> = ({
    onAddClick,
    onRemoveClick,
    onrefreshClick,
}) => {
    return (
        <div className="button-move-container">
            <button
                className="moveButton"
                onClick={onAddClick}
                title="選択したウィンドウを追加"
            >
                <FaPlus size={20} />
            </button>
            <button
                className="moveButton"
                onClick={onRemoveClick}
                title="選択したウィンドウを削除"
            >
                <FaMinus size={20} />
            </button>
            <button
                className="moveButton"
                onClick={onrefreshClick}
                title="更新"
            >
                <FaSync size={20} />
            </button>
        </div>
    );
};

export default MoveButtons;
