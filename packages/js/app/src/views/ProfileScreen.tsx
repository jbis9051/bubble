import React from 'react';
import { ScrollView, View } from 'react-native';

import { SafeAreaView } from 'react-native-safe-area-context';

import Background from '../components/Background';
import Navigation from '../components/Profile/Navigation';
import ProfileHeader from '../components/Profile/ProfileHeader';
import SlideCard from '../components/Profile/SlideCard';

const ProfileScreen = () => (
    <View>
        <Background />
        <SafeAreaView style={{ flex: 1 }} edges={['top', 'left', 'right']}>
            <Navigation />
            <ProfileHeader />
        </SafeAreaView>
        <ScrollView style={{ flex: 0 }} showsVerticalScrollIndicator={false}>
            <SlideCard />
        </ScrollView>
    </View>
);

export default ProfileScreen;
