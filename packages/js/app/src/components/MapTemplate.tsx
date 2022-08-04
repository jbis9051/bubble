import React, { useEffect, useState } from 'react';
import { Platform } from 'react-native';
import MapView, { Marker as MapViewMarker } from 'react-native-maps';
import MapboxGL from '@rnmapbox/maps';
import Marker from './Marker';

MapboxGL.setAccessToken(process.env.REACT_APP_MAPBOX_ACCESS_TOKEN as string);

interface Location {
    longitude: number;
    latitude: number;
}

const initialRegion = {
    longitude: -122.4324,
    latitude: 37.78825,
};

const MapTemplate = ({
    region = initialRegion,
    style = {},
    markerRegion = initialRegion,
    updateLocation = (_newRegion: Location) => null,
}) => {
    const [location, setLocation] = useState(region);
    const [markerLocation, setMarkerLocation] = useState(markerRegion);

    useEffect(() => {
        setLocation(region);
    }, [region]);

    useEffect(() => {
        setMarkerLocation(markerRegion);
    }, [markerRegion]);

    return Platform.OS === 'ios' ? (
        <MapView
            region={{
                latitudeDelta: 0.015,
                longitudeDelta: 0.015,
                ...location,
            }}
            onRegionChangeComplete={(newRegion) => {
                updateLocation(newRegion);
            }}
            style={style}
        >
            <MapViewMarker
                coordinate={{
                    latitude: markerLocation.latitude,
                    longitude: markerLocation.longitude,
                }}
            >
                <Marker />
            </MapViewMarker>
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
