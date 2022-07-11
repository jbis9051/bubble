import React from 'react';
import { ScrollView, ImageBackground } from 'react-native';

import { SafeAreaView } from 'react-native-safe-area-context';

import Background from '../components/Background';
import Navigation from '../components/Profile/Navigation';
import ProfileHeader from '../components/Profile/ProfileHeader';
import SlideCardComponent from '../components/Profile/SlideCard';

const ProfileScreen = () => (
    <Background>
        <SafeAreaView style={{ flex: 1 }} edges={['top', 'left', 'right']}>
            <Navigation />
            <ProfileHeader />
        </SafeAreaView>
        <ScrollView style={{ flex: 0 }} showsVerticalScrollIndicator={false}>
            <SlideCardComponent />
        </ScrollView>
    </Background>
);

export default ProfileScreen;
