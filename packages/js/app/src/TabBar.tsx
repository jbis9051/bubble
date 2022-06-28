import React from 'react';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';

import HomeScreen from './views/HomeScreen';
import SettingsScreen from './views/SettingsScreen';

const Tab = createBottomTabNavigator();

const TabBar = () => (
    <Tab.Navigator
        initialRouteName='Home'
    >
        <Tab.Screen name="Home" component={HomeScreen} />
        <Tab.Screen name="Settings" component={SettingsScreen} />
    </Tab.Navigator>
);

export default TabBar;