import React, { useState, useEffect } from 'react';
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
    const setMapLocation = (newLocation: Location) => {
        setLocation(newLocation);
    };

    useEffect(() => {
        setLocation(location);
    }, [location]);

    return (
        <View style={{ flex: 1 }}>
            <Map location={location} />
            <SlideCard
                startingHeight={220}
                minHeight={70}
                marginTopHeight={200}
                setLocation={setMapLocation}
            />
        </View>
    );
};

export default FriendsScreen;
