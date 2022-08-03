import React, { useEffect } from 'react';
import { NavigationContainer } from '@react-navigation/native';
import SplashScreen from 'react-native-splash-screen';
import LoginSignup from './src/components/LoginSignup';

const App = () => {
    useEffect(() => {
        setTimeout(() => {
            SplashScreen.hide();
        }, 2000);
    });

    return (
        <NavigationContainer>
            <LoginSignup />
        </NavigationContainer>
    );
};

export default App;
