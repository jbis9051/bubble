import React, { useState } from 'react';
import { View } from 'react-native';
import { Region } from 'react-native-maps';
import Map from '../components/Groups/Map';
import SlideCard from '../components/Groups/SlideCard';

const initialRegion: Region = {
    longitude: -122.4324,
    latitude: 37.78825,
    latitudeDelta: 0.015,
    longitudeDelta: 0.015,
};

const FriendsScreen = () => {
    const [location, setLocation] = useState(initialRegion);

    return (
        <View style={{ flex: 1 }}>
            <Map location={location} />
            <SlideCard location={location} setLocation={setLocation} />
        </View>
    );
};

export default FriendsScreen;
