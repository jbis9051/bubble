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
    const [marker, setMarker] = useState(initialRegion);
    const setMapLocation = (newLocation: Location) => {
        setLocation(newLocation);
    };

    const setMarkerLocation = (newLocation: Location) => {
        setMarker(newLocation);
    };

    return (
        <View style={{ flex: 1 }}>
            <Map
                location={location}
                markerLocation={marker}
                setLocation={setLocation}
            />
            <SlideCard
                startingHeight={220}
                minHeight={70}
                marginTopHeight={200}
                setLocation={[setMapLocation, setMarkerLocation]}
                marker={marker}
            />
        </View>
    );
};

export default FriendsScreen;
