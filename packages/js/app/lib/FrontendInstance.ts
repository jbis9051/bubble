import {
    FrontendInstance as FrontendInstanceInternal,
    get_location,
    Result,
    Uuid
} from '@bubble/react-native-bubble-rust';
import * as native from '@bubble/react-native-bubble-rust';

export default class FrontendInstance {
    private readonly instance: FrontendInstanceInternal;

    private constructor(instance: FrontendInstanceInternal) {
        this.instance = instance;
    }

    static async init(dataDir: string) {
        const instance = await native.init(dataDir);
        if (instance.success) {
            return new FrontendInstance(instance.value);
        }
        throw instance.value;
    }

    status() {
        return FrontendInstance.promiseify(native.status(this.instance));
    }

    login(username_or_email: string, password: string) {
        return FrontendInstance.promiseify(native.login(this.instance, username_or_email, password));
    }

    register(username: string, password: string, name: string, email: string) {
        return FrontendInstance.promiseify(native.register(this.instance, username, password, name, email));
    }

    forgot(email: string) {
        return FrontendInstance.promiseify(native.forgot(this.instance, email));
    }

    get_location(group_uuid: Uuid, client: Uuid, before_timestamp: number, amount: number) {
        return FrontendInstance.promiseify(native.get_location(this.instance, group_uuid, client, before_timestamp, amount));
    }

    private static async promiseify<T, E>(promise: Promise<Result<T, E>>) {
        const res = await promise;
        if (res.success) {
            return res.value;
        }
        throw res.value;
    }


}