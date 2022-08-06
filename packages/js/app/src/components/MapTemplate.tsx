import React, { useEffect, useState } from 'react';
import { Platform, ViewStyle, StyleProp } from 'react-native';
import MapView, { Region } from 'react-native-maps';
import MapboxGL from '@rnmapbox/maps';
import Marker from './Marker';

MapboxGL.setAccessToken(process.env.REACT_APP_MAPBOX_ACCESS_TOKEN as string);

const initialRegion: Region[] = [
    {
        longitude: -122.4324,
        latitude: 37.78825,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
];

const regionContainingPoints = (locations: Region[]) => {
    let minLatitude = locations[0].latitude;
    let maxLatitude = locations[0].latitude;
    let minLongitude = locations[0].longitude;
    let maxLongitude = locations[0].longitude;

    locations.forEach((location) => {
        minLatitude = Math.min(minLatitude, location.latitude);
        maxLatitude = Math.max(maxLatitude, location.latitude);
        minLongitude = Math.min(minLongitude, location.longitude);
        maxLongitude = Math.max(maxLongitude, location.longitude);
    });

    const midLat = (minLatitude + maxLatitude) / 2;
    const midLng = (minLongitude + maxLongitude) / 2;

    const deltaLat =
        maxLatitude - minLatitude !== 0
            ? (maxLatitude - minLatitude) * 1.5
            : 0.015;
    const deltaLng =
        maxLongitude - minLongitude !== 0
            ? (maxLongitude - minLongitude) * 1.5
            : 0.015;

    return {
        latitude: midLat,
        longitude: midLng,
        latitudeDelta: deltaLat,
        longitudeDelta: deltaLng,
    };
};

const MapTemplate: React.FunctionComponent<{
    locations?: Region[];
    style?: StyleProp<ViewStyle>;
}> = ({ locations = initialRegion, style }) => {
    const [viewLocations, setViewLocations] = useState(locations);
    const { latitude, longitude } = regionContainingPoints(locations);

    useEffect(() => {
        setViewLocations(locations);
    }, [locations]);

    return Platform.OS === 'ios' ? (
        <MapView
            region={{ ...regionContainingPoints(viewLocations) }}
            style={style}
        >
            {locations.map((markerLocation, key) => (
                <Marker
                    coordinate={{
                        latitude: markerLocation.latitude,
                        longitude: markerLocation.longitude,
                    }}
                    key={key}
                />
            ))}
        </MapView>
    ) : (
        <MapboxGL.MapView style={style} styleURL={MapboxGL.StyleURL.Street}>
            <MapboxGL.Camera
                zoomLevel={15}
                centerCoordinate={[longitude, latitude]}
            />
            <MapboxGL.UserLocation />
            {locations.map((markerLocation, key) => (
                <Marker
                    coordinate={{
                        latitude: markerLocation.latitude,
                        longitude: markerLocation.longitude,
                    }}
                    key={key}
                />
            ))}
        </MapboxGL.MapView>
    );
};

export default MapTemplate;
