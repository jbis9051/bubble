import React, { Component } from 'react';
import {TextInput, Text, View, StyleSheet, TouchableOpacity} from 'react-native';
import {NativeStackScreenProps} from "@react-navigation/native-stack";
import Header from '../../components/Header';
import colors from '../../constants/Colors';

type RootStackParamList = {
    Login: undefined,
    Signup: undefined,
    Splash: undefined,
};
type Props = NativeStackScreenProps<RootStackParamList, 'Signup'>;

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

function Signup({route, navigation}: Props) {
    return (
        <View style={styles.container}>
            <Header page={'Splash'}/>
            <Text style={styles.title}>Sign Up</Text>
            <View style={styles.loginContainer}>
                <TextInput
                    style={styles.textInput}
                    // onChangeText={onChangeNumber} //calls when text is changed
                    placeholder="Enter your username"
                    keyboardType="default"/>
                <TextInput
                    style={styles.textInput}
                    placeholder="Enter your password"
                    keyboardType="default"/>
                <TouchableOpacity style={styles.login}>
                    <Text>Sign up</Text>
                </TouchableOpacity>
                <Text>Already have an account?</Text><Text onPress={() => navigation.navigate('Login')}>Log in here</Text>
            </View>
        </View>
    );
}
export default Signup;