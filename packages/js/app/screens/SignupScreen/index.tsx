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
        alignItems:'center',
        fontSize: 30,
        fontWeight: '100',
    },
    signupContainer:{
        top: 20,
        justifyContent: 'center',
    },
    signupButton:{
        top: 30,
        height: 40,
        width: 150,
        margin: 7,
        borderWidth: 1,
        borderRadius: 15,
        padding: 10,
        alignItems:'center',
    },
    textInput: {
        borderTopColor: colors.white,
        borderRightColor: colors.white,
        borderLeftColor: colors.white,
        height: 50,
        width: 300,
        margin: 7,
        borderWidth: 1,
        padding: 10,
    },
    textInputDescriptors:{
        color: colors.darkGrey,
        top: 10,
        left: 7,
    },
})

function Signup({route, navigation}: Props) {
    return (
        <View style={styles.container}>
            <Header page={'Splash'}/>
            <Text style={styles.title}>Enter username and password</Text>
            <View style={styles.signupContainer}>
                <Text style={styles.textInputDescriptors}>Username</Text>
                <TextInput
                    style={styles.textInput}
                    // onChangeText={onChangeNumber} //calls when text is changed
                    keyboardType="default"/>
                <Text style={styles.textInputDescriptors}>Password</Text>
                <TextInput
                    style={styles.textInput}
                    keyboardType="default"/>
            </View>
            <TouchableOpacity style={styles.signupButton}>
                <Text>Sign up</Text>
            </TouchableOpacity>
        </View>
    );
}
export default Signup;