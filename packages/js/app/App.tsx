import React, { Component } from 'react';
import {TextInput, Text, View, StyleSheet, Button, StatusBar, TouchableOpacity} from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {StackNavigationProp} from "@react-navigation/stack";
import login from './screens/LoginScreen/index';
import signup1 from './screens/SignupScreen/signup1';
import signup2 from './screens/SignupScreen/signup2';
import splash from './screens/SplashScreen/index';

type RootStackParamList = {
    Login: undefined,
    Signup1: undefined,
    Signup2: undefined,
    Splash: undefined,
};


const App = () => {
    const RootStack = createNativeStackNavigator<RootStackParamList>();
    return(
    <NavigationContainer>
        <RootStack.Navigator initialRouteName={"Splash"}>
            <RootStack.Screen
                name="Splash"
                component={splash}
                options={{headerShown: false, title: "Life 256"}}
            />
            <RootStack.Screen
                name="Login"
                component={login}
                options={{headerShown: false, title: "Log In"}}
            />
            <RootStack.Screen
                name="Signup1"
                component={signup1}
                options={{headerShown: false, title: "Sign Up"}}/>
            <RootStack.Screen
                name="Signup2"
                component={signup2}
                options={{headerShown: false, title: "Sign Up"}}/>
        </RootStack.Navigator>
    </NavigationContainer>
);
}

export default App;