import React, { Component } from 'react';
import {TextInput, Text, View, StyleSheet, Button, StatusBar, TouchableOpacity} from 'react-native';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import colors from '../../constants/Colors';

type RootStackParamList = {
    Login: undefined,
    Signup: undefined,
};
type Props = NativeStackScreenProps<RootStackParamList, 'Login'>;

const styles = StyleSheet.create({
    container: {
        alignItems: 'center',
        backgroundColor: colors.white,
        width: '100%',
        height: '100%',
    },
    title: {
        marginVertical: 75,
        alignItems:'center',
        fontSize: 95,
        fontWeight: '100',
    },
    loginContainer:{
        justifyContent: 'center',
        alignItems: 'center',
        flex: 1,
    },
    login:{
        height: 40,
        width: 150,
        margin: 7,
        borderWidth: 1,
        borderRadius: 15,
        padding: 10,
        alignItems:'center',
        justifyContent:'center',
    },
    textInput: {
        height: 50,
        width: 300,
        margin: 7,
        borderWidth: 1,
        borderRadius: 15,
        padding: 10,
    },
})

function Login({route, navigation}: Props) {
    return (
        <View style={styles.container}>
            <Text style={styles.title}>Log In</Text>
            <View style={styles.loginContainer}>
                <TextInput
                    style={styles.textInput}
                    // onChangeText={onChangeNumber} //calls when text is changed
                    placeholder="Enter login"
                    keyboardType="default"/>
                <TextInput
                    style={styles.textInput}
                    placeholder="Enter Password"
                    keyboardType="default"/>
                <TouchableOpacity style={styles.login}>
                    <Text>Log In</Text>
                </TouchableOpacity>
                <Text>Don't have a account? </Text><Text onPress={() => navigation.navigate('Signup')}>Sign up here</Text>
            </View>
        </View>
    );
}
export default Login;