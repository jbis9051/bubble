import FontAwesome from '@expo/vector-icons/FontAwesome';
import { Link, Tabs } from 'expo-router';
import { Pressable, useColorScheme } from 'react-native';

import Colors from '../../constants/Colors';

/**
 * You can explore the built-in icon families and icons on the web at https://icons.expo.fyi/
 */
function TabBarIcon(props: {
    name: React.ComponentProps<typeof FontAwesome>['name'];
    color: string;
    size?: number;
}) {
    return <FontAwesome size={24} style={{ marginBottom: -3 }} {...props} />;
}

export default function TabLayout() {
    const colorScheme = useColorScheme();

    return (
        <Tabs
            screenOptions={{
                tabBarActiveTintColor:
                    Colors[colorScheme ?? 'light'].background,
                tabBarShowLabel: false,
                tabBarStyle: {
                    height: 70,
                    backgroundColor: 'black',
                },
            }}
        >
            <Tabs.Screen
                name="index"
                options={{
                    title: 'Map',
                    headerShown: false,
                    tabBarIcon: ({ color }) => (
                        <TabBarIcon name="map-marker" color={color} />
                    ),
                }}
            />
        </Tabs>
    );
}
