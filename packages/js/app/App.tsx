import React, { Component } from 'react';
import {TextInput, Text, View, StyleSheet, Button, StatusBar, TouchableOpacity} from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {StackNavigationProp} from "@react-navigation/stack";
import login from './screens/LoginScreen';
import signup from './screens/SignupScreen';

type RootStackParamList = {
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
                options={{title: "Life 256"}}
            />
            <RootStack.Screen
                name="Signup"
                component={signup}
                options={{title: "Life 256"}}/>
        </RootStack.Navigator>
    </NavigationContainer>
);
}

export default App;