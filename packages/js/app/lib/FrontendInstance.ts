import {
    FrontendInstance as FrontendInstanceInternal,
    get_location,
    InitOptions,
    Result,
    Uuid,
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

    static getAppDir() {
       return native.getAppDir();
    }

    status() {
        console.log('status');
        return FrontendInstance.promiseify(native.status(this.instance));
    }

    login(username_or_email: string, password: string) {
        console.log('login');
        return FrontendInstance.promiseify(
            native.login(this.instance, username_or_email, password)
        );
    }

    register(username: string, password: string, name: string, email: string) {
        console.log('register');
        return FrontendInstance.promiseify(
            native.register(this.instance, username, password, name, email)
        );
    }

    forgot(email: string) {
        console.log('forgot');
        return FrontendInstance.promiseify(native.forgot(this.instance, email));
    }

    get_location(
        group_uuid: Uuid,
        client: Uuid,
        before_timestamp: number,
        amount: number
    ) {
        console.log('get_location');
        return FrontendInstance.promiseify(
            native.get_location(
                this.instance,
                group_uuid,
                client,
                before_timestamp,
                amount
            )
        );
    }

    get_groups() {
        console.log('get_groups');
        return FrontendInstance.promiseify(native.get_groups(this.instance));
    }

    create_group() {
        console.log('create_group');
        return FrontendInstance.promiseify(native.create_group(this.instance));
    }

    update_group(new_uuid: string, name: string | null) {
        console.log('update_group');
        return FrontendInstance.promiseify(
            native.update_group(this.instance, new_uuid, name)
        );
    }

    leave_group(group_uuid: Uuid) {
        console.log('leave_group');
        return FrontendInstance.promiseify(
            native.leave_group(this.instance, group_uuid)
        );
    }

    send_group_status(group_uuid: Uuid) {
        console.log('send_group_status');
        return FrontendInstance.promiseify(
            native.send_group_status(this.instance, group_uuid)
        );
    }

    search(query: string) {
        console.log('search');
        return FrontendInstance.promiseify(native.search(this.instance, query));
    }

    add_member(group_uuid: Uuid, user_uuid: Uuid) {
        console.log('add_member');
        return FrontendInstance.promiseify(
            native.add_member(this.instance, group_uuid, user_uuid)
        );
    }

    replace_key_packages() {
        console.log('replace_key_packages');
        return FrontendInstance.promiseify(
            native.replace_key_packages(this.instance)
        );
    }

    receive_messages() {
        console.log('receive_messages');
        return FrontendInstance.promiseify(
            native.receive_messages(this.instance)
        );
    }

    logout() {
        console.log('logout');
        return native.logout(this.instance);
    }

    private static async promiseify<T, E>(promise: Promise<Result<T, E>>) {
        const res = await promise;
        console.log(res);
        if (res.success) {
            return res.value;
        }
        throw res.value;
    }

    remove_member(group_uuid: Uuid, user_uuid: Uuid) {
        console.log('remove_member');
        return FrontendInstance.promiseify(
            native.remove_member(this.instance, group_uuid, user_uuid)
        );
    }
}
