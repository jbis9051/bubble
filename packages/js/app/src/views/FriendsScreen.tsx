import React, { useState } from 'react';
import { View } from 'react-native';
import { Region } from 'react-native-maps';
import { SafeAreaView } from 'react-native-safe-area-context';
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

const FriendsScreen = () => {
    const [locations, setLocations] = useState(initialRegions);

    return (
        <View style={{ flex: 1 }}>
            <Map locations={locations} />
            <View
                style={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    right: 0,
                    bottom: 0,
                    padding: 12,
                }}
            >
                <SafeAreaView>
                    <SearchBar />
                </SafeAreaView>
            </View>
            <SlideCard locations={locations} setLocations={setLocations} />
        </View>
    );
};

export default FriendsScreen;
