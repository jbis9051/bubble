


export class UserService {
    static async signup({ email, username, password, name }:
        { email: string, username: string, password: string, name: string }) {}

    static async emailConfirm({ token }: { token: string }) {}

    static async login({ username, password }: { username: string, password: string }) {}

    static async logout({ token }: { token: string }) {}

    static async forgotPassword({ email }: { email: string }) {}

    // forgot password routes ommitted because they are not called from the app
}