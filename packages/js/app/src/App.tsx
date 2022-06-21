import React, { Component } from 'react';
import {TextInput, Text, View, StyleSheet, Button, StatusBar, TouchableOpacity} from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {StackNavigationProp} from "@react-navigation/stack";
import login from './screens/LoginScreen/index';
import signup from './screens/SignupScreen/index';
import splash from './screens/SplashScreen/index';

type RootStackParamList = {
    Splash: undefined
    Login: undefined,
    Signup: undefined,
};


const App = () => {
    const RootStack = createNativeStackNavigator<RootStackParamList>();
    return(
    <NavigationContainer>
        <RootStack.Navigator initialRouteName={"Login"}>
            <RootStack.Screen
                name="Login"
                component={login}
                options={{headerShown:false}}
            />
            <RootStack.Screen
                name="Signup"
                component={signup}
                options={{headerShown:false}}/>
            <RootStack.Screen
                name="Splash"
                component={splash}
                options={{headerShown:false}}
            />
        </RootStack.Navigator>
    </NavigationContainer>
);
}

export default App;