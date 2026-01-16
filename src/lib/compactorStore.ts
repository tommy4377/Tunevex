import { writable } from 'svelte/store';

export interface CompactorState {
    path: string;
    isScanning: boolean;
    isCompressing: boolean;
    scanResult: any;
    compressionAlgo: number;
    progressCurrent: number;
    showProgress: boolean;
    currentFile: string;
    statusMsg: string;
    statusType: "info" | "success" | "error";
    logs: string[];
}

const initialState: CompactorState = {
    path: "C:\\Games",
    isScanning: false,
    isCompressing: false,
    scanResult: null,
    compressionAlgo: 0, // XPRESS4K
    progressCurrent: 0,
    showProgress: false,
    currentFile: "",
    statusMsg: "",
    statusType: "info",
    logs: []
};

export const compactorStore = writable<CompactorState>(initialState);

export const addLog = (msg: string) => {
    compactorStore.update(s => ({
        ...s,
        logs: [...s.logs, `[${new Date().toLocaleTimeString()}] ${msg}`].slice(-100)
    }));
};
