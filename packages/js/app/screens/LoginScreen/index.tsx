import React, { Component } from 'react';
import {ImageBackground, Text, View, StyleSheet, Button, StatusBar, TouchableOpacity} from 'react-native';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import Header from '../../components/Header';
import TextInputBox from "../../components/TextInputBox";
import colors from '../../constants/Colors';

type RootStackParamList = {
    Login: undefined,
    Signup1: undefined,
    Signup2: undefined,
    Splash: undefined,
};
type Props = NativeStackScreenProps<RootStackParamList, 'Login'>;

const styles = StyleSheet.create({
    container: {
        alignItems: 'center',
        backgroundColor: colors.white,
        width: '100%',
        height: '100%',
    },
    backgroundImage:{
        alignItems: 'center',
    },
    title: {
        flex: 0.7,
        alignItems: 'center',
        fontSize: 45,
        fontWeight: '100',
    },
    textContainer:{
        flex: 3,
        justifyContent: 'center',
    },
    textInput: {
        borderTopColor: colors.white,
        borderRightColor: colors.white,
        borderLeftColor: colors.white,
        height: 50,
        width: 300,
        margin: 7,
        borderWidth: 1,
    },textInputDescriptors:{
        color: colors.darkGrey,
    },loginContainer: {
        flex: 5,
    },
    login:{
        height: 40,
        width: 200,
        margin: 7,
        borderWidth: 1,
        borderRadius: 15,
        padding: 10,
        alignItems:'center',
        justifyContent:'center',
    },forgot:{
        fontSize: 13,
        textAlign: 'center',
    }
})

function Login({route, navigation}: Props) {
    return (
        <View style={styles.container}>
            <ImageBackground
                source={require('../../constants/background.png')}
                style={styles.backgroundImage}
            >
                <Header page={'Splash'}/>
                <Text style={styles.title}>Welcome back</Text>
                <View style={styles.textContainer}>
                    <TextInputBox
                        descriptor='Username'
                        required={false}
                    />
                    <TextInputBox
                        descriptor='Password'
                        required={false}
                    />
                </View>
                <View style={styles.loginContainer}>
                    <TouchableOpacity style={styles.login}>
                        <Text>Log In</Text>
                    </TouchableOpacity>
                    <Text style={styles.forgot} onPress={() => {navigation.navigate('')}}>Forgot password?</Text>
                </View>
            </ImageBackground>
        </View>
    );
}
export default Login;