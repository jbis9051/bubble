import React, { Component } from 'react';
import {Text, View, StyleSheet, ImageBackground, TouchableOpacity} from 'react-native';
import {NativeStackScreenProps} from '@react-navigation/native-stack';
import login from './LoginScreen';
import colors from '../constants/Colors';

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
        fontSize: 80,
        fontWeight: '300',
        justifyContent: 'center',
    },
    titleContainer:{
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
    }, /*info:{
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
        borderRadius: 30,
        padding: 10,
        alignItems:'center',
        justifyContent:'center',
        backgroundColor: colors.primary,
    },buttonText:{
        color: colors.white,
        fontWeight: '600',
    } */
})


function Splash({route, navigation}: Props) {
    setTimeout(() =>{
        navigation.navigate('Login')
        }, 2000
    );
    return (
        <View style={styles.container}>
            <ImageBackground
                source={require('../assets/background.png')}
                style={styles.backgroundImage}
            >
                <View style={styles.titleContainer}>
                    <Text style={styles.title}>Bubble</Text>
                </View>
            </ImageBackground>
        </View>
    );
}
export default Splash;