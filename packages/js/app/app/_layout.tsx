import FontAwesome from '@expo/vector-icons/FontAwesome';
import {useFonts} from 'expo-font';
import {SplashScreen, Stack} from 'expo-router';
import {useEffect, useRef, useState} from 'react';
import {observer} from 'mobx-react-lite';
import {Alert} from 'react-native';
import {autorun} from 'mobx';
import Auth from './auth';
import MainStore from '../stores/MainStore';
import FrontendInstanceStore from '../stores/FrontendInstanceStore';
import FrontendInstance from "../lib/FrontendInstance";

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
    const receiveTimer = useRef<NodeJS.Timer | null>(null);

    const loggedIn = !!MainStore.status?.account_data;

    function receive() {
        if (receiveTimer.current) {
            clearTimeout(receiveTimer.current);
        }
        FrontendInstanceStore.instance
            .receive_messages()
            .then(async received => {
                if(received > 0){
                    MainStore.groups =
                        await FrontendInstanceStore.instance.get_groups();
                    MainStore.current_group = MainStore.groups.find(g => g.uuid === MainStore.current_group?.uuid) || MainStore.groups[0] || null;
                }
                receiveTimer.current = setTimeout(receive, 2000);
            })
            .catch((err) => {
                Alert.alert('Error Receiving Messages', err.message);
            });
    }

    useEffect(
        () =>
            autorun(() => {
                if (MainStore.status?.account_data) {
                    receive();
                }
                return () => {
                    if (receiveTimer.current) {
                        clearTimeout(receiveTimer.current);
                    }
                };
            }),
        []
    );

    useEffect(() => {
        if (!FrontendInstanceStore.isInitialized()) {
            FrontendInstance.getAppDir().then((appDir) => {
                    console.log("appDir: ", appDir);
                    return FrontendInstanceStore.init({
                        data_directory: appDir,
                        force_new: false,
                    })
                }
            )
                .then(() => FrontendInstanceStore.instance.status())
                .then((status) => {
                    MainStore.status = status;
                    setInited(true);
                })
                .then(async () => {
                    if (MainStore.status?.account_data) {
                        MainStore.groups =
                            await FrontendInstanceStore.instance.get_groups();
                        if (MainStore.groups.length > 0) {
                            MainStore.current_group = MainStore.groups[0];
                        }
                    }
                })
                .catch((err) => {
                    throw err;
                });
        }
    }, []);

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
        <Stack initialRouteName={'map'}>
            <Stack.Screen name="map" options={{headerShown: false}}/>
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
