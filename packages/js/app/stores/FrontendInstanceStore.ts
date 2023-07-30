import { makeAutoObservable } from 'mobx';
import { InitOptions } from '@bubble/react-native-bubble-rust';
import FrontendInstance from '../lib/FrontendInstance';

class FrontendInstanceStore {
    _instance: FrontendInstance | null = null;

    constructor() {
        makeAutoObservable(this);
    }

    async init(options: InitOptions) {
        this._instance = await FrontendInstance.init(options);
    }

    get instance() {
        if (this._instance === null) {
            throw new Error('Frontend instance not initialized');
        }
        return this._instance;
    }

    isInitialized() {
        return this._instance !== null;
    }
}

export default new FrontendInstanceStore();
