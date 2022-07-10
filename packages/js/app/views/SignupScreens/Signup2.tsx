import React, { Component } from 'react';
import {TextInput, Text, View, StyleSheet, ImageBackground, TouchableOpacity} from 'react-native';
import {NativeStackScreenProps} from "@react-navigation/native-stack";
import Header from '../../components/Header';
import TextInputBox from "../../components/TextInputBox";
import colors from '../../constants/Colors';

type RootStackParamList = {
    Login: undefined,
    Signup1: undefined,
    Signup2: undefined,
    Splash: undefined,
};

type Props = NativeStackScreenProps<RootStackParamList, 'Signup2'>;

const styles = StyleSheet.create({
    container: {
        alignItems: 'center',
        backgroundColor: colors.white,
        width: '100%',
        height: '100%',
    },
    backgroundImage:{
        width: '100%',
        height: '100%',
        alignItems: 'center',
    },titleContainer:{
        borderWidth: 1,
        top: '12%',
        flex: 1.75,
        justifyContent: 'center',
    }, signupContainer:{
        borderWidth: 1,
        flex: 2.5,
        justifyContent: 'center',
    },signupButtonContainer:{
        borderWidth: 1,
        flex: 1.5,
    },
    title: {
        fontSize: 45,
        fontWeight: '400',
        color: colors.primary,
    },
    signupButton:{
        height: 40,
        width: 150,
        margin: 7,
        borderWidth: 1,
        borderRadius: 15,
        padding: 10,
        alignItems:'center',
    },
})

function Signup({route, navigation}: Props) {
    return (
        <View style={styles.container}>
            <ImageBackground
                source={require('../../assets/SignUp2.png')}
                style={styles.backgroundImage}
            >
                <Header page={'Signup1'}/>
                <View style={styles.titleContainer}>
                    <Text style={styles.title}>Sign Up</Text>
                </View>
                <View style={styles.signupContainer}>
                    <TextInputBox
                        descriptor="Username"
                        secure={true}
                    />
                    <TextInputBox
                        descriptor="Password"
                        secure={true}
                    />
                    <TextInputBox
                        descriptor="Confirm Password"
                        secure={true}
                    />
                </View>
                <View style={styles.signupButtonContainer}>
                   <TouchableOpacity style={styles.signupButton}>
                       <Text>Sign Up</Text>
                   </TouchableOpacity>
                </View>
            </ImageBackground>
        </View>
    );
}
export default Signup;