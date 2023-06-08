import FontAwesome from '@expo/vector-icons/FontAwesome';
import {
    DarkTheme,
    DefaultTheme,
    ThemeProvider as NavThemeProvider,
} from '@react-navigation/native';
import { useFonts } from 'expo-font';
import { SplashScreen, Stack } from 'expo-router';
import { useEffect } from 'react';
import { useColorScheme } from 'react-native';
import { UserContext, UserLocal, useSession } from '../lib/bubbleApi/user';
import SignInScreen from '../components/display/SignInComponent';
import { ThemeContext } from '../lib/Context';
import Colors from '../constants/Colors';

export {
    // Catch any errors thrown by the Layout component.
    ErrorBoundary,
} from 'expo-router';

export const unstable_settings = {
    // Ensure that reloading on `/modal` keeps a back button present.
    initialRouteName: '(tabs)',
};

export default function RootLayout() {
    const [fontsLoaded, error] = useFonts({
        SpaceMono: require('../assets/fonts/SpaceMono-Regular.ttf'),
        ...FontAwesome.font,
    });
    const { user, loaded: userLoading, setUser } = useSession();

    // Expo Router uses Error Boundaries to catch errors in the navigation tree.
    useEffect(() => {
        if (error) throw error;
    }, [error]);

    const loaded = userLoading && fontsLoaded;

    return (
        <>
            {/* Keep the splash screen open until the assets have loaded. In the future, we should just support async font loading with a native version of font-display. */}
            {!loaded && <SplashScreen />}
            {loaded && !user && <SignInScreen setUser={setUser} />}
            {loaded && user && <RootLayoutNav user={user} />}
        </>
    );
}

function RootLayoutNav({ user }: { user: UserLocal }) {
    const colorScheme = useColorScheme();
    const darkMode = colorScheme === 'dark';

    return (
        <>
            <NavThemeProvider
                value={colorScheme === 'dark' ? DarkTheme : DefaultTheme}
            >
                <ThemeContext.Provider
                    value={darkMode ? Colors.dark : Colors.light}
                >
                    <UserContext.Provider value={user}>
                        <Stack>
                            <Stack.Screen
                                name="(tabs)"
                                options={{ headerShown: false }}
                            />
                            <Stack.Screen
                                name="modal"
                                options={{ presentation: 'modal' }}
                            />
                        </Stack>
                    </UserContext.Provider>
                </ThemeContext.Provider>
            </NavThemeProvider>
        </>
    );
}
