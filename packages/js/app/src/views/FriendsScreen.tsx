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

const locationProps = {};

const FriendsScreen = () => {
    const [viewLocation, setViewLocation] = useState(initialRegion);
    const [marker, setMarker] = useState(initialRegion);

    const setMapLocation = (newLocation: Location) => {
        setViewLocation(newLocation);
    };
    const setMarkerLocation = (newLocation: Location) => {
        setMarker(newLocation);
    };

    return (
        <View style={{ flex: 1 }}>
            <Map
                location={viewLocation}
                markerLocation={marker}
                setLocation={setViewLocation}
            />
            <SlideCard
                setLocation={[setMapLocation, setMarkerLocation]}
                marker={marker}
            />
        </View>
    );
};

export default FriendsScreen;
