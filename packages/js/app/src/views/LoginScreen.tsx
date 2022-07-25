import React from 'react';
import { Text, View, StyleSheet, TouchableOpacity } from 'react-native';
import type { createStackNavigator } from '@react-navigation/stack';
import TextInputBox from '../components/TextInputBox';
import colors from '../constants/colors';
import LoginBackground from '../assets/LoginBackground.svg';

type RootStackParamList = {
    Login: undefined;
    Signup1: undefined;
    Signup2: undefined;
    Splash: undefined;
};

type Props = createStackNavigator<RootStackParamList, 'Login'>;

const styles = StyleSheet.create({
    container: {
        alignItems: 'center',
        justifyContent: 'center',
        backgroundColor: colors.white,
        width: '100%',
        height: '100%',
    },
    titleContainer: {
        top: '12%',
        flex: 3,
        justifyContent: 'center',
    },
    textContainer: {
        flex: 1.6,
        width: '80%',
        justifyContent: 'center',
    },
    loginContainer: {
        width: '80%',
        flex: 1.5,
        justifyContent: 'center',
    },
    noAccountContainer: {
        flex: 1.25,
        alignItems: 'center',
        justifyContent: 'center',
    },
    title: {
        fontSize: 45,
        fontWeight: '400',
        color: colors.primary,
    },
    textInputDescriptors: {
        color: colors.darkGrey,
    },
    login: {
        height: 50,
        margin: 7,
        borderRadius: 25,
        padding: 10,
        alignItems: 'center',
        justifyContent: 'center',
        backgroundColor: colors.primary,
    },
    forgot: {
        fontWeight: '200',
        fontSize: 13,
        textAlign: 'center',
    },
    otherLoginButtons: {
        height: 40,
        width: 40,
        margin: 7,
        borderWidth: 1,
        borderRadius: 20,
        padding: 10,
    },
    buttonText: {
        fontSize: 14,
        color: colors.white,
        fontWeight: '600',
    },
    noAccountText: {
        color: colors.primary,
        fontSize: 16,
        fontWeight: '300',
    },
    noAccountTextLink: {
        color: colors.primary,
        fontSize: 16,
        fontWeight: '300',
    },
});

const fetchLogin = async () => {
    try {
        const res = await fetch('/user/signup', {
            method: 'POST',
        });
        const json = await res.json();
        return json;
    } catch (error) {
        console.error(error);
    }
    return null;
};

function Login({ route, navigation }: Props) {
    return (
        <View style={styles.container}>
            <LoginBackground
                height={'100%'}
                width={'100%'}
                style={{ position: 'absolute' }}
            />
            <View style={styles.titleContainer}>
                <Text style={styles.title}>Sign In</Text>
            </View>
            <View style={styles.textContainer}>
                <TextInputBox descriptor="Username" params={''} />
                <TextInputBox descriptor="Password" params={'password'} />
            </View>
            <View style={styles.loginContainer}>
                <TouchableOpacity style={styles.login}>
                    <Text style={styles.buttonText}>Sign In</Text>
                </TouchableOpacity>
            </View>
            <View style={styles.noAccountContainer}>
                <Text style={styles.noAccountText}>Don't have an account?</Text>
                <TouchableOpacity
                    onPress={() => navigation.navigate('Signup1')}
                >
                    <Text style={styles.noAccountTextLink}>Sign up</Text>
                </TouchableOpacity>
            </View>
        </View>
    );
}
export default Login;
