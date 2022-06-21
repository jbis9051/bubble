import React, { Component } from 'react';
import {Text, View, StyleSheet, StatusBar, TouchableOpacity} from 'react-native';
import {NativeStackScreenProps} from "@react-navigation/native-stack";
import colors from '../../constants/Colors';
import TempLogo from '../../../assets/tempLife256Logo.svg';

type RootStackParamList = {
    Login: undefined,
    Signup: undefined,
    Splash: undefined,
};
type Props = NativeStackScreenProps<RootStackParamList, 'Splash'>;

const styles = StyleSheet.create({
    container: {
        flexDirection: 'column',
        alignItems: 'center',
        backgroundColor: colors.white,
        width: '100%',
        height: '100%',
    },logo:{
        flex:1,
        height: 100,
        width: 100,
    },
    title: {
        flex:1,
        alignItems:'center',
        fontSize: 95,
        fontWeight: '100',
    },
    titleContainer:{
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
        flexDirection: "column",
    },logSign:{

    },buttons:{
        height: 40,
        width: 150,
        margin: 7,
        borderWidth: 1,
        borderRadius: 15,
        padding: 10,
        alignItems:'center',
        justifyContent:'center',
    }
})

function Splash({route, navigation}: Props) {
    return (
        <View style={styles.container}>
            <View style={styles.titleContainer}>
                <Text style={styles.title}>Life 256</Text>
                <TempLogo style={styles.logo} />
            </View>
            <View style={styles.logSign}>
                <TouchableOpacity style={styles.buttons}>
                    <Text onPress={() => navigation.navigate('Signup')}>Sign up</Text>
                </TouchableOpacity>
                <TouchableOpacity style={styles.buttons}>
                    <Text onPress={() => navigation.navigate('Login')}>Log in</Text>
                </TouchableOpacity>
            </View>
        </View>
    );
}
export default Splash;