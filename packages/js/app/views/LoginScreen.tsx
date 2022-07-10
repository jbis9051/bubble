import React, { Component } from 'react';
import {ImageBackground, Text, View, StyleSheet, Button, StatusBar, TouchableOpacity} from 'react-native';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import Header from '../components/Header';
import TextInputBox from "../components/TextInputBox";
import colors from '../constants/Colors';

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
        justifyContent: 'center',
        backgroundColor: colors.white,
        width: '100%',
        height: '100%',
    },titleContainer:{
        top: '12%',
        flex: 3,
        justifyContent: 'center',
    },
    textContainer:{
        flex: 1.25,
        justifyContent: 'center',
    },
    loginContainer: {
        flex: 1.5,
        justifyContent: 'center',
    },noAccountContainer:{
        flex: 1.5,
        alignItems: 'center',
        justifyContent: 'center',
    },
    title: {
        fontSize: 45,
        fontWeight: '400',
        color: colors.primary,
    },
    textInputDescriptors:{
        color: colors.darkGrey,
    },
    login:{
        height: 50,
        width: 300,
        margin: 7,
        borderRadius: 25,
        padding: 10,
        alignItems:'center',
        justifyContent:'center',
        backgroundColor: colors.primary,
    },forgot:{
        fontWeight: '200',
        fontSize: 13,
        textAlign: 'center',
    },otherLoginButtons:{
        height: 40,
        width: 40,
        margin: 7,
        borderWidth: 1,
        borderRadius: 20,
        padding: 10,
        alignItems: 'center',
        justifyContent: 'center',
    },buttonText:{
        color: colors.white,
        fontWeight: '600',
    },noAccountText:{
        color: colors.primary,
        fontSize: 16,
        fontWeight: '300',
    },noAccountTextLink:{
        color: colors.primary,
        fontSize: 16,
        fontWeight: '300',
    }
})

function Login({route, navigation}: Props) {
    return (
        <View style={styles.container}>
            <ImageBackground
                source={require('../assets/LoginBackground.png')}
                style={{height: '100%', width: '100%', alignItems: 'center'}}
            >
                <View style={styles.titleContainer}>
                    <Text style={styles.title}>Sign In</Text>
                </View>
                <View style={styles.textContainer}>
                    <TextInputBox
                        descriptor='Username'
                        secure={false}
                        input={""}
                    />
                    <TextInputBox
                        descriptor='Password'
                        secure={true}
                        input={""}
                    />
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
            </ImageBackground>
        </View>
    );
}
export default Login;