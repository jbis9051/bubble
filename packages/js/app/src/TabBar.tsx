import React from 'react';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';

import HomeScreen from './views/HomeScreen';
import SettingsScreen from './views/SettingsScreen';
import MapScreen from './views/MapScreen';
import ProfileScreen from './views/ProfileScreen';
import FriendsScreen from './views/FriendsScreen';

const Tab = createBottomTabNavigator();

const TabBar = () => (
    <Tab.Navigator
        initialRouteName='Home'
    >
        <Tab.Screen name="Home" component={HomeScreen} />
        <Tab.Screen name="Friends" component={FriendsScreen} />
        <Tab.Screen name="Map" component={MapScreen} />
        <Tab.Screen name="Profile" component={ProfileScreen} />
    </Tab.Navigator>
);

export default TabBar;