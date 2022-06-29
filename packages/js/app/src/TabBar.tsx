import React from 'react';
import { StyleSheet, Text, View, Image, TouchableOpacity } from 'react-native';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';

import HomeScreen from './views/HomeScreen';
import SettingsScreen from './views/SettingsScreen';
import MapScreen from './views/MapScreen';
import ProfileScreen from './views/ProfileScreen';
import FriendsScreen from './views/FriendsScreen';

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
        <Tab.Screen name="Home" 
            component={HomeScreen} 
            options={{
                tabBarIcon: ({focused}) => (
                    <View style={{
                        alignItems: 'center',
                        justifyContent: 'center',
                        top: 10
                    }}>
                        <Image
                            source={require('../assets/icons/home.png')}
                            resizeMode='contain'
                            style={{
                                width: 35,
                                height: 35,
                                tintColor: focused ? '#e32f45' : '#748c94'
                            }}
                        />
                        <Text style={{color: focused ? '#e32f45' : '#748c94', fontSize: 12}}>
                            Home
                        </Text>
                    </View>
                )
            }}
        />
        <Tab.Screen name="Friends" 
            component={FriendsScreen}
            options={{
                tabBarIcon: ({focused}) => (
                    <View style={{
                        alignItems: 'center',
                        justifyContent: 'center',
                        top: 10
                    }}>
                        <Image
                            source={require('../assets/icons/friends.png')}
                            resizeMode='contain'
                            style={{
                                width: 35,
                                height: 35,
                                tintColor: focused ? '#e32f45' : '#748c94'
                            }}
                        />
                        <Text style={{color: focused ? '#e32f45' : '#748c94', fontSize: 12}}>
                            Friends
                        </Text>
                    </View>
                )
            }} 
        />
        <Tab.Screen name="Map" 
            component={MapScreen} 
            options={{
                tabBarIcon: ({focused}) => (
                    <View style={{
                        alignItems: 'center',
                        justifyContent: 'center',
                        top: 10
                    }}>
                        <Image
                            source={require('../assets/icons/globe.png')}
                            resizeMode='contain'
                            style={{
                                width: 30,
                                height: 30,
                                tintColor: focused ? '#e32f45' : '#748c94'
                            }}
                        />
                        <Text style={{color: focused ? '#e32f45' : '#748c94', fontSize: 12}}>
                            Map
                        </Text>
                    </View>
                )
            }} 
        />
        <Tab.Screen name="Profile" 
            component={ProfileScreen} 
            options={{
                tabBarIcon: ({focused}) => (
                    <View style={{
                        alignItems: 'center',
                        justifyContent: 'center',
                        top: 10
                    }}>
                        <Image
                            source={require('../assets/icons/user.png')}
                            resizeMode='contain'
                            style={{
                                width: 30,
                                height: 30,
                                tintColor: focused ? '#e32f45' : '#748c94'
                            }}
                        />
                        <Text style={{color: focused ? '#e32f45' : '#748c94', fontSize: 12}}>
                            User
                        </Text>
                    </View>
                )
            }} 
        />
    </Tab.Navigator>
);

export default TabBar;