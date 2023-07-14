import {
    FrontendInstance as FrontendInstanceInternal,
    get_location, InitOptions,
    Result,
    Uuid
} from '@bubble/react-native-bubble-rust';
import * as native from '@bubble/react-native-bubble-rust';

export default class FrontendInstance {
    private readonly instance: FrontendInstanceInternal;

    private constructor(instance: FrontendInstanceInternal) {
        this.instance = instance;
    }

    static async init(options: InitOptions) {
        const instance = await native.init(options);
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

    get_groups() {
        return FrontendInstance.promiseify(native.get_groups(this.instance));
    }

    create_group() {
        return FrontendInstance.promiseify(native.create_group(this.instance));
    }

    update_group(new_uuid: string, name: string | null) {
        return FrontendInstance.promiseify(native.update_group(this.instance, new_uuid, name));
    }

    leave_group(group_uuid: Uuid) {
        return FrontendInstance.promiseify(native.leave_group(this.instance, group_uuid));
    }

    private static async promiseify<T, E>(promise: Promise<Result<T, E>>) {
        const res = await promise;
        if (res.success) {
            return res.value;
        }
        throw res.value;
    }
}