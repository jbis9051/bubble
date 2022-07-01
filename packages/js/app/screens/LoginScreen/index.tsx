import React, { Component } from 'react';
import {TextInput, Text, View, StyleSheet, Button, StatusBar, TouchableOpacity} from 'react-native';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import Header from '../../components/Header';
import colors from '../../constants/Colors';

type RootStackParamList = {
    Login: undefined,
    Signup: undefined,
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
    title: {
        marginVertical: 75,
        alignItems: 'center',
        fontSize: 60,
        fontWeight: '100',
    },
    textContainer:{
        top: 10,
        justifyContent: 'center',
        alignItems: 'center',
    },
    textInput: {
        height: 50,
        width: 300,
        margin: 7,
        borderWidth: 1,
        borderRadius: 15,
        padding: 10,
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
        fontSize: 15,
        textAlign: 'center',
    }
})


function Login({route, navigation}: Props) {
    return (
        <View style={styles.container}>
            <Header page={'Splash'}/>
            <Text style={styles.title}>Welcome back</Text>
            <View style={styles.textContainer}>
                <TextInput
                    style={styles.textInput}
                    // onChangeText={onChangeNumber} //calls when text is changed
                    placeholder="Username"
                    keyboardType="default"
                    autoFocus={true}/>
                <TextInput
                    style={styles.textInput}
                    placeholder="Password"
                    keyboardType="default"/>
            </View>
            <View style={{top: 20,}}>
                <TouchableOpacity style={styles.login}>
                    <Text>Log In</Text>
                </TouchableOpacity>
                <Text style={styles.forgot} onPress={() => {navigation.navigate('Signup')}}>Forgot password?</Text>
            </View>
        </View>
    );
}
export default Login;