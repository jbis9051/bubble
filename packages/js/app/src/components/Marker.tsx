import React from 'react';
import { View, Text } from 'react-native';
import { Marker as MarkerView } from 'react-native-maps';

const Marker: React.FunctionComponent<{
    coordinate: { longitude: number; latitude: number };
}> = ({ coordinate }) => (
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
    </MarkerView>
);

export default Marker;
