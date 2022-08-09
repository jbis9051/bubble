import React, { useState } from 'react';
import { View } from 'react-native';
import { Region } from 'react-native-maps';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import Map from '../components/Groups/Map';
import SearchBar from '../components/Groups/SearchBar';
import SlideCard from '../components/Groups/SlideCard';

const initialRegions: Region[] = [
    {
        longitude: -122.4324,
        latitude: 37.78825,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
];

const GroupsScreen = () => {
    const [locations, setLocations] = useState(initialRegions);
    const insets = useSafeAreaInsets();

    return (
        <View style={{ flex: 1 }}>
            <Map locations={locations} />
            <SearchBar insets={insets} />
            <SlideCard locations={locations} setLocations={setLocations} />
        </View>
    );
};

export default GroupsScreen;
