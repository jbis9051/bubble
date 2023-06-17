import FontAwesome from '@expo/vector-icons/FontAwesome';
import {
    DarkTheme,
    DefaultTheme,
    ThemeProvider as NavThemeProvider,
} from '@react-navigation/native';
import { useFonts } from 'expo-font';
import { SplashScreen, Stack, } from 'expo-router';
import { useEffect } from 'react';
import { useColorScheme } from 'react-native';
import { Provider as ReduxProvider, useSelector, } from 'react-redux';

import { UserContext, UserLocal, useSession } from '../lib/bubbleApi/user';
import SignInScreen from '../components/display/SignInComponent';
import { ThemeContext } from '../lib/Context';
import Colors from '../constants/Colors';
import store from '../redux/store';
import { selectUser } from '../redux/slices/authSlice';
import { useGroups } from '../lib/bubbleApi/group';
import { AndroidPromptProvider } from '../components/PromptProvider';

export {
    // Catch any errors thrown by the Layout component.
    ErrorBoundary,
} from 'expo-router';

export const unstable_settings = {
    // Ensure that reloading on `/modal` keeps a back button present.
    initialRouteName: '(tabs)',
};

function RootLayout() {
    const [fontsLoaded, error] = useFonts({
        SpaceMono: require('../assets/fonts/SpaceMono-Regular.ttf'),
        ...FontAwesome.font,
    });
    const { loaded: userLoading, } = useSession();
    const { loaded: groupsLoaded } = useGroups();
    const user = useSelector(selectUser);

    useEffect(() => {
        if (error) throw error;
    }, [error]);

    const loaded = userLoading && fontsLoaded && groupsLoaded;

    return (
        <>
            {/* Keep the splash screen open until the assets have loaded. In the future, we should just support async font loading with a native version of font-display. */}
            {!loaded && <SplashScreen />}
            {loaded && !user && <SignInScreen />}
            {loaded && user && <RootLayoutNav user={user} />}
        </>
    );
}

function RootLayoutNav({ user }: { user: UserLocal }) {
    const colorScheme = useColorScheme();
    const darkMode = colorScheme === 'dark';

    return (
        <>
            <NavThemeProvider value={colorScheme === 'dark' ? DarkTheme : DefaultTheme}>
                <ThemeContext.Provider value={darkMode ? Colors.dark : Colors.light}>
                    <AndroidPromptProvider />
                    <Stack>
                        <Stack.Screen
                            name="(tabs)"
                            options={{ headerShown: false }}
                        />
                        <Stack.Screen
                            name="allGroupsModal"
                            options={{ presentation: 'modal', headerShown: false, }}
                        />
                        <Stack.Screen
                            name="groupSettingsModal"
                            options={{ presentation: 'modal', headerShown: false, }}
                        />
                    </Stack>
                </ThemeContext.Provider>
            </NavThemeProvider>
        </>
    );
}

export default function WithReduxLayout() {
    return (
        <ReduxProvider store={store}>
            <RootLayout />
        </ReduxProvider>
    )
}