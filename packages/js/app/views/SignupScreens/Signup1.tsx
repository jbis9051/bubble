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
    },titleContainer:{
        top: '12%',
        flex: 3,
        justifyContent: 'center',
    }, signupContainer:{
        flex: 4,
        justifyContent: 'center',
    }, signupButtonContainer:{
        flex: 2,
    },accountExistContainer:{
        flex: 1,
        alignItems: 'center',
        bottom: '2.6%',
    },
    title: {
        fontSize: 45,
        fontWeight: '400',
        color: colors.primary,
    },
    signupButton:{
        height: 50,
        width: 300,
        margin: 7,
        borderRadius: 15,
        padding: 10,
        alignItems:'center',
        justifyContent:'center',
        backgroundColor: colors.primary,
    },buttonText:{
        color: colors.white,
        fontWeight: '600',
    },noAccountText:{
        color: colors.primary,
        fontSize: 16,
        fontWeight: '300',
    },accountExistTextLink:{
        color: colors.primary,
        fontSize: 16,
        fontWeight: '300',
    }
})

function Signup({route, navigation}: Props) {
    return (
        <View style={styles.container}>
            <ImageBackground
                source={require('../../assets/SignUpBackground.png')}
                style={{height: '100%', width: '100%', alignItems: 'center',}}
            >
                <View style={styles.titleContainer}>
                    <Text style={styles.title}>Sign Up</Text>
                </View>
                <View style={styles.signupContainer}>
                    <TextInputBox
                        descriptor="Phone Number"
                        secure={false}
                        input={"telephoneNumber"}
                    />
                    <TextInputBox
                        descriptor="Username"
                        secure={true}
                        input={""}
                    />
                    <TextInputBox
                        descriptor="Password"
                        secure={true}
                        input={""}
                    />
                </View>
                <View style={styles.signupButtonContainer}>
                    <TouchableOpacity
                        style={styles.signupButton}
                        onPress={() => navigation.navigate('Signup2')}
                    >
                        <Text style={styles.buttonText}>Create Account</Text>
                    </TouchableOpacity>
                </View>
                <View style={styles.accountExistContainer}>
                    <Text style={styles.noAccountText}>Already have an account?</Text>
                    <TouchableOpacity
                        onPress={() => navigation.navigate('Login')}
                    >
                        <Text style={styles.accountExistTextLink}>
                            Sign In
                        </Text>
                    </TouchableOpacity>
                </View>
            </ImageBackground>
        </View>
    );
}
export default Signup;