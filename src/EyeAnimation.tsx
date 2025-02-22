import React from "react";
import "@fortawesome/fontawesome-free/css/all.min.css";

interface EyeAnimationProps {
    isMonitoring: boolean;
};

export const EyeAnimation: React.FC<EyeAnimationProps> = ({ isMonitoring }) => {
    const containerClass = isMonitoring ? "monitoring" : "not-monitoring";

    return (
        <div className={`eye-indicator ${containerClass}`}>
            <div className="icon-container">
                <i className="fas fa-eye"></i>
                <i className="fas fa-eye-slash"></i>
            </div>
            <span className="indicator-text">
                {isMonitoring ? "監視中" : "停止中"}
            </span>
        </div>
    );
};
