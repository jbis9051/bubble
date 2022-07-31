import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/stack';
import login from './src/views/LoginScreen';
import signup1 from './src/views/SignupScreens/Signup1';
import signup2 from './src/views/SignupScreens/Signup2';
import splash from './src/views/SplashScreen';
import TabBar from './src/components/TabBar';

type RootStackParamList = {
    Login: undefined;
    Signup1: undefined;
    Signup2: undefined;
    Splash: undefined;
};

const App = () => {
    const RootStack = createNativeStackNavigator<RootStackParamList>();

    return (
        <NavigationContainer>
            <RootStack.Navigator initialRouteName={'Splash'}>
                <RootStack.Screen
                    name="Splash"
                    component={splash}
                    options={{ headerShown: false, title: 'Life 256' }}
                />
                <RootStack.Screen
                    name="Login"
                    component={login}
                    options={{ headerShown: false, title: 'Log In' }}
                />
                <RootStack.Screen
                    name="Signup1"
                    component={signup1}
                    options={{ headerShown: false, title: 'Sign Up' }}
                />
                <RootStack.Screen
                    name="Signup2"
                    component={signup2}
                    options={{ headerShown: false, title: 'Sign Up' }}
                />
            </RootStack.Navigator>
        </NavigationContainer>
    );
};

export default () => (
    <NavigationContainer>
        <TabBar />
    </NavigationContainer>
);
