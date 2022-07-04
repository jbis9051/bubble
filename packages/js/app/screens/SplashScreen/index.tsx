import React, { Component } from 'react';
import {Text, View, StyleSheet, ImageBackground, TouchableOpacity} from 'react-native';
import {NativeStackScreenProps} from '@react-navigation/native-stack';
import colors from '../../constants/Colors';

type RootStackParamList = {
    Login: undefined,
    Signup1: undefined,
    Signup2: undefined,
    Splash: undefined,
};

type Props = NativeStackScreenProps<RootStackParamList, 'Splash'>;

const styles = StyleSheet.create({
    container: {
        borderWidth: 1,
        flexDirection: 'column',
        alignItems: 'center',
        backgroundColor: colors.white,
        width: '100%',
        height: '100%',
    },backgroundImage:{
        height: '100%',
        width: '100%',
    },
    title: {
        fontSize: 95,
        fontWeight: '100',
    },
    titleContainer:{
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
    }, info:{
        flex: 1,
        textAlign: 'center',
    },buttonsContainer:{
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
    },
    buttons:{
            height: 50,
            width: 250,
            margin: 7,
            borderWidth: 1,
            borderRadius: 15,
            padding: 10,
            alignItems:'center',
            justifyContent:'center',
    },
})

function Splash({route, navigation}: Props) {
    return (
        <View style={styles.container}>
            <ImageBackground
                source={require('../../assets/background.png')}
                style={styles.backgroundImage}
            >
                <View style={styles.titleContainer}>
                    <Text style={styles.title}>Bubble</Text>
                </View>
                <View style={styles.buttonsContainer}>
                    <TouchableOpacity
                        style={styles.buttons}
                        onPress={() => {navigation.navigate('Signup1')}}>
                        <Text>Sign up</Text>
                    </TouchableOpacity>

                    <TouchableOpacity
                        style={styles.buttons}
                        onPress={() => {navigation.navigate('Login')}}>
                        <Text>Log in</Text>
                    </TouchableOpacity>
                </View>
            </ImageBackground>
        </View>
    );
}
export default Splash;