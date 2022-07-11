import React from 'react';
import { ScrollView, ImageBackground } from 'react-native';

import { SafeAreaView } from 'react-native-safe-area-context';

import Navigation from '../components/Profile/Navigation';
import ProfileHeader from '../components/Profile/ProfileHeader';
import SlideCardComponent from '../components/Profile/SlideCard';

const ProfileScreen = () => (
    <ImageBackground
        /* eslint-disable global-require */
        source={require('../../assets/background.png')}
    >
        <SafeAreaView style={{ flex: 1 }} edges={['top', 'left', 'right']}>
            <Navigation />
            <ProfileHeader profileImage='../../assets/user.jpeg'/>
        </SafeAreaView>
        <ScrollView style={{ flex: 0 }} showsVerticalScrollIndicator={false}>
            <SlideCardComponent />
        </ScrollView>
    </ImageBackground>
);

export default ProfileScreen;
