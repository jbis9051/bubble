import React, { useEffect, useState } from 'react';
import { LoggingService } from './logging';
import { useDispatch } from 'react-redux';
import { setAuth } from '../../redux/slices/authSlice';
import { uuid } from './group';

export class UserService {
    // static async signup({ email, username, password, name }:
    //     { email: string, username: string, password: string, name: string }) {}

    // static async emailConfirm({ token }: { token: string }) {}

    // static async login({ username, password }: { username: string, password: string }) {}

    // static async logout({ token }: { token: string }) {}

    // static async forgotPassword({ email }: { email: string }) {}

    static async register(username: string, password: string, name: string) {}
    static async login(email: string, password: string) {}
    static async logout() {}
    static async forgot(email: string) {}

    static async retrieveSession(): Promise<UserLocal | null> {
        return { name: 'test', user_uuid: 'user-uuid-1', primary_client_uuid: 'client-uuid-1' };
        // return null;
    }
}

export interface UserLocal {
    name: string;
    primary_client_uuid: uuid;
    user_uuid: uuid;
}

export function useSession() {
    const [loaded, setLoaded] = useState(false);
    const dispatch = useDispatch();

    useEffect(() => {
        UserService.retrieveSession()
            .then((u) => {
                dispatch(setAuth(u));
                setLoaded(true);
            })
            .catch((e) => {
                LoggingService.error(e);
            });
    }, []);

    return { loaded };
}

export const UserContext = React.createContext<UserLocal | null>(null);
