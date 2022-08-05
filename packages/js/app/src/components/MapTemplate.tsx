import React, { useEffect, useState, CSSProperties } from 'react';
import { Platform, ViewStyle, StyleProp } from 'react-native';
import MapView, { Region } from 'react-native-maps';
import MapboxGL from '@rnmapbox/maps';
import Marker from './Marker';

MapboxGL.setAccessToken(process.env.REACT_APP_MAPBOX_ACCESS_TOKEN as string);

const initialRegion = {
    longitude: -122.4324,
    latitude: 37.78825,
};

const MapTemplate: React.FunctionComponent<{
    region?: Region;
    style?: StyleProp<ViewStyle>;
}> = ({ region = initialRegion, style }) => {
    const [location, setLocation] = useState(region);

    useEffect(() => {
        setLocation(region);
    }, [region]);

    return Platform.OS === 'ios' ? (
        <MapView
            region={{
                latitudeDelta: 0.015,
                longitudeDelta: 0.015,
                ...location,
            }}
            style={style}
        >
            <Marker
                coordinate={{
                    latitude: location.latitude,
                    longitude: location.longitude,
                }}
            />
        </MapView>
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
