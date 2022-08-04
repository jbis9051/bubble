import React from 'react';
import { View, Text } from 'react-native';

const Marker = () => (
    <View
        style={{
            flexDirection: 'column',
            alignItems: 'center',
            paddingBottom: 95,
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
);

export default Marker;
