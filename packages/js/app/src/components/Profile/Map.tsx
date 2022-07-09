import React from 'react';
import { View } from 'react-native';
import MapView from 'react-native-maps';

const Map = () => (
    <View>
        <MapView
            initialRegion={{
                latitude: 37.78825,
                longitude: -122.4324,
                latitudeDelta: 0.0922,
                longitudeDelta: 0.0421,
            }}
            style={{
                height: 300,
                borderRadius: 15,
                marginBottom: 10,
            }}
        />
    </View>
);

export default Map;
