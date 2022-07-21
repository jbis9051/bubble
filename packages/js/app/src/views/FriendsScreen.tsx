import React from 'react';
import { View, ScrollView } from 'react-native';
import Map from '../components/Groups/Map';
import SlideCard from '../components/Groups/SlideCard';

const FriendsScreen = () => (
    <View>
        <Map />
        <ScrollView style={{ 
            width:'100%', 
            position: 'absolute',
            top: 0,
            bottom: 0
        }} showsVerticalScrollIndicator={false}>
            <SlideCard />
        </ScrollView>
    </View>
);

export default FriendsScreen;
