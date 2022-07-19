import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import TabBar from './src/components/TabBar';
import Login from './src/views/LoginScreen';
import Signup1 from './src/views/SignupScreens/Signup1';
import Signup2 from './src/views/SignupScreens/Signup2';
import Splash from './src/views/SplashScreen';

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
                    component={Splash}
                    options={{ headerShown: false, title: 'Life 256' }}
                />
                <RootStack.Screen
                    name="Login"
                    component={Login}
                    options={{ headerShown: false, title: 'Log In' }}
                />
                <RootStack.Screen
                    name="Signup1"
                    component={Signup1}
                    options={{ headerShown: false, title: 'Sign Up' }}
                />
                <RootStack.Screen
                    name="Signup2"
                    component={Signup2}
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
