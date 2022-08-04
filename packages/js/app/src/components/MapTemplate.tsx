import React, { useEffect, useState, useRef } from 'react';
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
    // const mapView = useRef<<MapView>>(null).current;

    useEffect(() => {
        setLocation(region);
    }, [region]);

    useEffect(() => {
        // mapView?.animateToRegion({
        //     latitudeDelta: 0.015,
        //     longitudeDelta: 0.015,
        //     ...markerRegion,
        // }, 5000);
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
            // ref={mapView}
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
