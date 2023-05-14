import React, { useEffect } from 'react';
import { NavigationContainer } from '@react-navigation/native';
import SplashScreen from 'react-native-splash-screen';
import LoginSignup from './src/components/LoginSignup';

const TEMP_IS_LOGGED_IN = false;

const fetchIsLoggedIn = () => {
    try {
        return TEMP_IS_LOGGED_IN;
    } catch (error) {
        console.error(error);
        return false;
    }
};

const isLoggedIn = fetchIsLoggedIn();

const App = () => {
    useEffect(() => {
        SplashScreen.hide();
    }, []);

    return (
        <NavigationContainer>
            <LoginSignup isLoggedIn={isLoggedIn} />
        </NavigationContainer>
    );
};

export default App;
