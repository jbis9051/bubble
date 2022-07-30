import React, { useEffect, useState } from 'react';
import { Platform } from 'react-native';
import MapView from 'react-native-maps';
import MapboxGL from '@rnmapbox/maps';

MapboxGL.setAccessToken(process.env.REACT_APP_MAPBOX_ACCESS_TOKEN as string);

const initialRegion = {
    longitude: -122.4324,
    latitude: 37.78825,
};

const MapTemplate = ({ region = initialRegion, style = {} }) => {
    const [location, setLocation] = useState(region);

    useEffect(() => {
        setLocation(region);
    }, [region]);

    return Platform.OS === 'ios' ? (
        <MapView
            region={{
                ...location,
                latitudeDelta: 0.0922,
                longitudeDelta: 0.0421,
            }}
            style={style}
        />
    ) : (
        <MapboxGL.MapView style={style} styleURL={MapboxGL.StyleURL.Street}>
            <MapboxGL.Camera
                zoomLevel={10}
                centerCoordinate={Object.values(location)}
            />
            <MapboxGL.UserLocation />
        </MapboxGL.MapView>
    );
};

export default MapTemplate;
