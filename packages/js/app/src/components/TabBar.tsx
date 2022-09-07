import React from 'react';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';

import {
    faHouse as homeIcon,
    faUserGroup as groupIcon,
    faMap as mapIcon,
    faCircleUser as userIcon,
} from '@fortawesome/free-solid-svg-icons';
import HomeScreen from '../views/HomeScreen';
import MapScreen from '../views/MapScreen';
import ProfileScreen from '../views/ProfileScreen';
import GroupScreen from '../views/Groups/GroupScreen';

import TabIcon from './TabIcon';

const Tab = createBottomTabNavigator();

const TabBar = () => (
    <Tab.Navigator
        screenOptions={{
            tabBarShowLabel: false,
            headerShown: false,
            tabBarStyle: {
                backgroundColor: '#ffffff',
                height: 90,
            },
        }}
        initialRouteName="Home"
    >
        <Tab.Screen
            name="Home"
            component={HomeScreen}
            options={{
                tabBarIcon: ({ focused }) => (
                    <TabIcon name="Home" icon={homeIcon} focused={focused} />
                ),
            }}
        />
        <Tab.Screen
            name="Groups"
            component={GroupScreen}
            options={{
                tabBarIcon: ({ focused }) => (
                    <TabIcon name="Groups" icon={groupIcon} focused={focused} />
                ),
            }}
        />
        <Tab.Screen
            name="Map"
            component={MapScreen}
            options={{
                tabBarIcon: ({ focused }) => (
                    <TabIcon name="Map" icon={mapIcon} focused={focused} />
                ),
            }}
        />
        <Tab.Screen
            name="Profile"
            component={ProfileScreen}
            options={{
                tabBarIcon: ({ focused }) => (
                    <TabIcon name="Profile" icon={userIcon} focused={focused} />
                ),
            }}
        />
    </Tab.Navigator>
);

export default TabBar;
