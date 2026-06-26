import { invoke } from "@tauri-apps/api/tauri";

type LogLevel = "debug" | "info" | "warn" | "error";

export const logFrontend = (level: LogLevel, message: string) => {
    const logger = level === "error" ? console.error : level === "warn" ? console.warn : console.info;
    logger(message);

    void invoke("log_from_frontend", { level, message }).catch((e) => {
        console.error(`failed to write frontend log: ${e}`);
    });
};
