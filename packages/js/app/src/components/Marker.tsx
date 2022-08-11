import React from 'react';
import { View, Text, Platform } from 'react-native';
import { Marker as MarkerView } from 'react-native-maps';
import MapboxGL from '@rnmapbox/maps';

const Marker: React.FunctionComponent<{
    name: string;
    coordinate: { longitude: number; latitude: number };
}> = ({ name, coordinate }) =>
    Platform.OS === 'ios' ? (
        <MarkerView
            coordinate={{
                longitude: coordinate.longitude,
                latitude: coordinate.latitude,
            }}
        >
            <View
                style={{
                    flexDirection: 'column',
                    alignItems: 'center',
                    paddingBottom: 87.5,
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
                        {name[0]}
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
        </MarkerView>
    ) : (
        <MapboxGL.MarkerView
            id="markerId"
            coordinate={[coordinate.longitude, coordinate.latitude]}
        >
            <View
                style={{
                    flexDirection: 'column',
                    alignItems: 'center',
                    paddingBottom: 87.5,
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
                        {name[0]}
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
        </MapboxGL.MarkerView>
    );

export default Marker;
