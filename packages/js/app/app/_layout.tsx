import FontAwesome from '@expo/vector-icons/FontAwesome';
import {useFonts} from 'expo-font';
import {SplashScreen, Stack} from 'expo-router';
import {useEffect, useState} from 'react';
import Auth from "./auth";
import {observer} from "mobx-react-lite";
import MainStore from "../stores/MainStore";
import FrontendInstanceStore from "../stores/FrontendInstanceStore";

export {
    // Catch any errors thrown by the Layout component.
    ErrorBoundary,
} from 'expo-router';

const RootLayout = observer(() => {
    const [fontsLoaded, error] = useFonts({
        SpaceMono: require('../assets/fonts/SpaceMono-Regular.ttf'),
        ...FontAwesome.font,
    });
    const [inited, setInited] = useState(false);

    const loggedIn = !!(MainStore.status?.account_data);

    useEffect(() => {
        if (!FrontendInstanceStore.isInitialized()) {
            FrontendInstanceStore.init({
                data_directory: "/Users/joshuabrown3/Desktop/data",
                force_new: false
            })
                .then(() => FrontendInstanceStore.instance.status())
                .then((status) => {
                    MainStore.status = status;
                    setInited(true);
                })
                .then(async () => {
                    if (MainStore.status?.account_data) {
                        MainStore.groups = await FrontendInstanceStore.instance.get_groups()
                    }
                })
                .catch((err) => {
                    throw err;
                });
        }
    }, [])


    useEffect(() => {
        if (error) throw error;
    }, [error]);

    const loaded = fontsLoaded && inited;

    return (
        <>
            {/* Keep the splash screen open until the assets have loaded. In the future, we should just support async font loading with a native version of font-display. */}
            {!loaded && <SplashScreen/>}
            {loaded && !loggedIn && <Auth/>}
            {loaded && loggedIn && <RootLayoutNav/>}
        </>
    );
});

function RootLayoutNav() {
    return (
        <Stack initialRouteName={"map"}>
            <Stack.Screen
                name="map"
                options={{headerShown: false}}
            />
            <Stack.Screen
                name="groups"
                options={{
                    presentation: 'modal',
                    headerShown: false,
                }}
            />
            <Stack.Screen
                name="groupSettings"
                options={{
                    presentation: 'modal',
                    headerShown: false,
                }}
            />
        </Stack>
    );
}

export default RootLayout;