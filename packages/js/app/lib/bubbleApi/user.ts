import React, { useEffect, useState } from 'react';
import { LoggingService } from './logging';

export class UserService {
    // static async signup({ email, username, password, name }:
    //     { email: string, username: string, password: string, name: string }) {}

    // static async emailConfirm({ token }: { token: string }) {}

    // static async login({ username, password }: { username: string, password: string }) {}

    // static async logout({ token }: { token: string }) {}

    // static async forgotPassword({ email }: { email: string }) {}

    static async register(username: string, password: string, name: string) {}
    static async login(username: string, password: string) {}
    static async logout() {}
    static async forgot(email: string) {}

    static async retrieveSession(): Promise<UserLocal | null> {
        return null;
    }
}

export interface UserLocal {
    name: string;
}

export function useSession() {
    const [user, setUser] = useState<UserLocal | null>(null);
    const [loaded, setLoaded] = useState(false);

    useEffect(() => {
        UserService.retrieveSession()
            .then((u) => {
                setUser(u);
                setLoaded(true);
            })
            .catch((e) => {
                LoggingService.error(e);
            });
    }, []);

    return { user, setUser, loaded, setLoaded };
}

export const UserContext = React.createContext<UserLocal | null>(null);
