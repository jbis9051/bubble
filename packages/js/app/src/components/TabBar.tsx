import React from 'react';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';

import HomeScreen from '../views/HomeScreen';
import MapScreen from '../views/MapScreen';
import ProfileScreen from '../views/ProfileScreen';
import FriendsScreen from '../views/FriendsScreen';

import TabIcon from './TabIcon';
import { 
    faHouse as homeIcon,
    faUserGroup as friendsIcon,
    faMap as mapIcon,
    faCircleUser as userIcon
} from '@fortawesome/free-solid-svg-icons';

const Tab = createBottomTabNavigator();


const TabBar = () => (
    <Tab.Navigator
        screenOptions={{
            tabBarShowLabel: false,
            headerShown: false,
            tabBarStyle: {
                backgroundColor: '#ffffff',
                height: 90,
            }
        }}
        initialRouteName='Home'
    >
        <Tab.Screen
            name='Home'
            component={HomeScreen}
            options={{
                tabBarIcon: ({ focused }) => (
                    <TabIcon
                        name='Home'
                        icon={homeIcon}
                        focused={focused}
                    />
                )
            }}
        />
        <Tab.Screen name="Friends" 
            component={FriendsScreen}
            options={{
                tabBarIcon: ({focused}) => (
                    <TabIcon
                        name='Friends'
                        icon={friendsIcon}
                        focused={focused}
                    />
                )
            }} 
        />
        <Tab.Screen name="Map" 
            component={MapScreen} 
            options={{
                tabBarIcon: ({focused}) => (
                    <TabIcon
                        name='Map'
                        icon={mapIcon}
                        focused={focused}
                    />
                )
            }} 
        />
        <Tab.Screen name="Profile" 
            component={ProfileScreen} 
            options={{
                tabBarIcon: ({focused}) => (
                    <TabIcon
                        name='Profile'
                        icon={userIcon}
                        focused={focused}
                    />
                )
            }} 
        />
    </Tab.Navigator>
);

export default TabBar;