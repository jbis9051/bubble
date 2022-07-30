import React from 'react';
import { View } from 'react-native';
import Map from '../components/Groups/Map';
import SlideCard from '../components/Groups/SlideCard';

const FriendsScreen = () => (
    <View style={{ flex: 1 }}>
        <Map />
        <SlideCard />
    </View>
);

export default FriendsScreen;
