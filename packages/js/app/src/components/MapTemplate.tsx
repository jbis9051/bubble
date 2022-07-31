import React, { useEffect, useState } from 'react';
import { View, Text, Platform } from 'react-native';
import MapView, { Marker } from 'react-native-maps';
import MapboxGL from '@rnmapbox/maps';

MapboxGL.setAccessToken(process.env.REACT_APP_MAPBOX_ACCESS_TOKEN as string);

const initialRegion = {
    longitude: -122.4324,
    latitude: 37.78825,
};

const MapTemplate = ({
    region = initialRegion,
    style = {},
    markerRegion = initialRegion,
    updateLocation = () => null,
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
            <Marker
                coordinate={{
                    latitude: markerLocation.latitude,
                    longitude: markerLocation.longitude,
                }}
            >
                <View
                    style={{
                        flexDirection: 'column',
                        alignItems: 'center',
                    }}
                >
                    <View
                        style={{
                            height: 75,
                            width: 75,
                            backgroundColor: '#ffffff',
                            borderRadius: 100,
                            alignItems: 'center',
                            justifyContent: 'center',
                            marginBottom: 5,
                        }}
                    >
                        <Text
                            style={{
                                fontSize: 48,
                            }}
                        >
                            J
                        </Text>
                    </View>
                    <View
                        style={{
                            margin: 0,
                            alignItems: 'center',
                            justifyContent: 'center',
                            height: 15,
                            width: 15,
                            backgroundColor: '#ffffff',
                            borderRadius: 100,
                        }}
                    ></View>
                </View>
            </Marker>
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
