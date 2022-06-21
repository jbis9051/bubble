//import React, { Component } from 'react';
import * as React from 'react';
import {TextInput, Text, View, StyleSheet, Button, StatusBar, TouchableOpacity} from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {StackNavigationProp} from "@react-navigation/stack";
import Login from './screens/LoginScreen/index';
import Signup from './screens/SignupScreen/index';



const App = () => {
    type RootStackParamList = {
        loginScreen: undefined,
        signupScreen: undefined,
    };

    const RootStack = createNativeStackNavigator<RootStackParamList>();

    return(
        <NavigationContainer>
            <RootStack.Navigator initialRouteName={"loginScreen"}>
                <RootStack.Screen
                name="loginScreen"
                component={Login}
                options={{title: 'Welcome'}}
                />
                <RootStack.Screen
                name="signupScreen" component={Signup}/>
            </RootStack.Navigator>
        </NavigationContainer>
    );
}
export default App;