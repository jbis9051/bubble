import React, { useState } from 'react';
import { View } from 'react-native';
import Map from '../components/Groups/Map';
import SlideCard from '../components/Groups/SlideCard';

interface Location {
    longitude: number;
    latitude: number;
}

const initialRegion = {
    longitude: -122.4324,
    latitude: 37.78825,
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
