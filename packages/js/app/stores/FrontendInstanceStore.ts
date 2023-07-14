import FrontendInstance from "../lib/FrontendInstance";
import { makeAutoObservable } from "mobx"

class FrontendInstanceStore {
    _instance: FrontendInstance | null = null;

    constructor() {
        makeAutoObservable(this);
    }

    async init(dataDir: string) {
        this._instance = await FrontendInstance.init(dataDir);
    }

    get instance() {
        if (this._instance === null) {
            throw new Error("Frontend instance not initialized");
        }
        return this._instance;
    }

    isInitialized() {
        return this._instance !== null;
    }
}

export default new FrontendInstanceStore();