import FrontendInstance from "../lib/FrontendInstance";
import { makeAutoObservable } from "mobx"
import {Group, Status, Uuid} from "@bubble/react-native-bubble-rust";

class MainStore {
    get groups(): Group[] {
        return this._groups;
    }

    set groups(value: Group[]) {
        this._groups = value;
    }
    get status(): Status | null {
        return this._status;
    }

    set status(value: Status | null) {
        this._status = value;
    }

    get current_group(): Group | null {
        return this._current_group;
    }

    set current_group(value: Group | null) {
        this._current_group = value;
    }

    private _status: Status | null = null;
    private _current_group: Group | null = null;
    private _groups: Group[] = [];


    constructor() {
        makeAutoObservable(this);
    }
}

export default new MainStore();