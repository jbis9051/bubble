import React, { Component } from 'react';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';

import HomeScreen from './views/HomeScreen';
import MapScreen from './views/MapScreen';
import ProfileScreen from './views/ProfileScreen';
import FriendsScreen from './views/FriendsScreen';

import TabIcon from './components/TabIcon';

const Tab = createBottomTabNavigator();


const TabBar = () => (
    <Tab.Navigator
        screenOptions={{
            tabBarShowLabel: false,
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
                        image={require('../assets/icons/home.png')}
                        size={35}
                        color='#e32f45'
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
                        image={require('../assets/icons/friends.png')}
                        size={35}
                        color='#e32f45'
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
                        image={require('../assets/icons/globe.png')}
                        size={30}
                        color='#e32f45'
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
                        image={require('../assets/icons/user.png')}
                        size={30}
                        color='#e32f45'
                        focused={focused}
                    />
                )
            }} 
        />
    </Tab.Navigator>
);

export default TabBar;