/* WARNING: This file is auto-generated. Do not modify. */

import { NativeModules, Platform } from 'react-native';
import { Result, Uuid, FrontendInstance, Base64 } from './index';

const LINKING_ERROR =
    `The package 'react-native-bubble-rust' doesn't seem to be linked. Make sure: \n\n${Platform.select(
        { ios: "- You have run 'pod install'\n", default: '' }
    )}- You rebuilt the app after installing the package\n` +
    `- You are not using Expo Go\n`;

export const RustInterop = NativeModules.Bubble
    ? NativeModules.Bubble
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR);
              },
          }
      );

/* ---------------- STRUCT DEFINITIONS ------------------- */

export interface Location {
    latitude: number,
    longitude: number,
    timestamp: number,
}
export interface UserOut {
    uuid: Uuid,
    username: string,
    name: string,
    primary_client_uuid: Uuid | null,
    identity: Base64,
}
export interface AccountData {
    domain: string,
    user_uuid: Uuid,
    client_uuid: Uuid | null,
}
export interface Status {
    domain: string,
    data_directory: string,
    account_data: AccountData | null,
}
export interface UserGroupInfo {
    info: UserOut,
    clients: Uuid[],
}
export interface Group {
    uuid: Uuid,
    name: string | null,
    image: number[] | null,
    members: { [key: Uuid]: UserGroupInfo },
}

/* ---------------- FUNCTION DEFINITIONS ------------------- */

export function receive_messages(instance: FrontendInstance,): Promise<Result<void, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'receive_messages',
        args: {},
    })).then((res: string) => JSON.parse(res));
}

export function replace_key_packages(instance: FrontendInstance,): Promise<Result<void, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'replace_key_packages',
        args: {},
    })).then((res: string) => JSON.parse(res));
}

export function register(instance: FrontendInstance,username: string , password: string , name: string , email: string ): Promise<Result<void, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'register',
        args: {username, password, name, email},
    })).then((res: string) => JSON.parse(res));
}

export function login(instance: FrontendInstance,username_or_email: string , password: string ): Promise<Result<Uuid, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'login',
        args: {username_or_email, password},
    })).then((res: string) => JSON.parse(res));
}

export function logout(instance: FrontendInstance,): Promise<Result<void, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'logout',
        args: {},
    })).then((res: string) => JSON.parse(res));
}

export function forgot(instance: FrontendInstance,email: string ): Promise<Result<void, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'forgot',
        args: {email},
    })).then((res: string) => JSON.parse(res));
}

export function confirm(instance: FrontendInstance,token: Uuid ): Promise<Result<Uuid, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'confirm',
        args: {token},
    })).then((res: string) => JSON.parse(res));
}

export function forgot_confirm(instance: FrontendInstance,password: string , token: Uuid ): Promise<Result<void, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'forgot_confirm',
        args: {password, token},
    })).then((res: string) => JSON.parse(res));
}

export function forgot_check(instance: FrontendInstance,token: Uuid ): Promise<Result<boolean, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'forgot_check',
        args: {token},
    })).then((res: string) => JSON.parse(res));
}

export function search(instance: FrontendInstance,query: string ): Promise<Result<UserOut[], Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'search',
        args: {query},
    })).then((res: string) => JSON.parse(res));
}

export function status(instance: FrontendInstance,): Promise<Result<Status, void>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'status',
        args: {},
    })).then((res: string) => JSON.parse(res));
}

export function get_groups(instance: FrontendInstance,): Promise<Result<Group[], Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'get_groups',
        args: {},
    })).then((res: string) => JSON.parse(res));
}

export function create_group(instance: FrontendInstance,): Promise<Result<Uuid, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'create_group',
        args: {},
    })).then((res: string) => JSON.parse(res));
}

export function add_member(instance: FrontendInstance,group_uuid: Uuid , user_uuid: Uuid ): Promise<Result<void, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'add_member',
        args: {group_uuid, user_uuid},
    })).then((res: string) => JSON.parse(res));
}

export function remove_member(instance: FrontendInstance,group_uuid: Uuid , user_uuid: Uuid ): Promise<Result<void, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'remove_member',
        args: {group_uuid, user_uuid},
    })).then((res: string) => JSON.parse(res));
}

export function leave_group(instance: FrontendInstance,group_uuid: Uuid ): Promise<Result<void, Error>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'leave_group',
        args: {group_uuid},
    })).then((res: string) => JSON.parse(res));
}

export function get_location(instance: FrontendInstance,group_uuid: Uuid , client: Uuid , before_timestamp: number , amount: number ): Promise<Result<Location[], void>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'get_location',
        args: {group_uuid, client, before_timestamp, amount},
    })).then((res: string) => JSON.parse(res));
}

export function get_num_location(instance: FrontendInstance,group_uuid: Uuid , client: Uuid , from_timestamp: number , to_timestamp: number ): Promise<Result<number, void>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'get_num_location',
        args: {group_uuid, client, from_timestamp, to_timestamp},
    })).then((res: string) => JSON.parse(res));
}

export function send_location(instance: FrontendInstance,group_uuid: Uuid , longitude: number , latitude: number , timestamp: number ): Promise<Result<void, void>> {
    return RustInterop.call(JSON.stringify({
        instance,
        method: 'send_location',
        args: {group_uuid, longitude, latitude, timestamp},
    })).then((res: string) => JSON.parse(res));
}

