import React, { Component } from 'react';
import {TextInput, Text, View, StyleSheet, TouchableOpacity, ImageBackground} from 'react-native';
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
type Props = NativeStackScreenProps<RootStackParamList, 'Signup1'>;

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
        flex: 0.5,
        alignItems:'center',
        fontSize: 30,
        fontWeight: '100',
    },
    signupContainer:{
        flex: 6,
        justifyContent: 'center',
    },signupButtonContainer:{
        flex: 4,
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
                source={require('../../assets/background.png')}
                style={styles.backgroundImage}
            >
                <Header page={'Splash'}/>
                <Text style={styles.title}>Enter Account Details</Text>
                <View style={styles.signupContainer}>
                    <TextInputBox
                        descriptor="Phone Number"
                        required={true}
                    />
                    <TextInputBox
                        descriptor="Email"
                        required={false}
                    />
                    <TextInputBox
                        descriptor="First Name"
                        required={true}
                    />
                    <TextInputBox
                        descriptor="Last Name"
                        required={true}
                    />
                </View>
                <View style={styles.signupButtonContainer}>
                    <TouchableOpacity
                        style={styles.signupButton}
                        onPress={() => navigation.navigate('Signup2')}
                    >
                        <Text>Next</Text>
                    </TouchableOpacity>
                </View>
            </ImageBackground>
        </View>
    );
}
export default Signup;