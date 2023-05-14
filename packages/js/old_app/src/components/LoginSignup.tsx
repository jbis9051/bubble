import { createNativeStackNavigator } from '@react-navigation/native-stack';
import React from 'react';
import login from '../views/LoginScreen';
import signup1 from '../views/SignupScreens/Signup1';
import signup2 from '../views/SignupScreens/Signup2';
import splash from '../views/SplashScreen';
import tabBar from './TabBar';

type RootStackParamList = {
    TabBar: undefined;
    Login: undefined;
    Signup1: undefined;
    Signup2: undefined;
    Splash: undefined;
};

const RootStack = createNativeStackNavigator<RootStackParamList>();

const LoginSignup: React.FC<{ isLoggedIn: boolean }> = ({ isLoggedIn }) => (
    <RootStack.Navigator initialRouteName={isLoggedIn ? 'TabBar' : 'Login'}>
        <RootStack.Screen
            name="Splash"
            component={splash}
            options={{ headerShown: false, title: 'Bubble' }}
        />
        <RootStack.Screen
            name="TabBar"
            component={tabBar}
            options={{ headerShown: false, title: 'TabBar' }}
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
);
export default LoginSignup;
