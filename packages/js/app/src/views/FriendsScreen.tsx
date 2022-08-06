import React, { useState } from 'react';
import { View } from 'react-native';
import { Region } from 'react-native-maps';
import Map from '../components/Groups/Map';
import SlideCard from '../components/Groups/SlideCard';

const initialRegions: Region[] = [
    {
        longitude: -122.4324,
        latitude: 37.78825,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
];

const FriendsScreen = () => {
    const [locations, setLocations] = useState(initialRegions);

    return (
        <View style={{ flex: 1 }}>
            <Map locations={locations} />
            <SlideCard locations={locations} setLocations={setLocations} />
        </View>
    );
};

export default FriendsScreen;
