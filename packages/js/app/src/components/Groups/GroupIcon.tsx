import React from 'react';
import { View, Image, TouchableWithoutFeedback } from 'react-native';
import { Region } from 'react-native-maps';
import styles from './styles';
import Name from './Name';

const coordinates: Region[] = [
    {
        longitude: -74.6551,
        latitude: 40.3431,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
    {
        longitude: -83.7382,
        latitude: 42.287,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
    {
        longitude: -74.0131,
        latitude: 40.7118,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
    {
        longitude: -122.009,
        latitude: 37.3346,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
    {
        longitude: -73.620071,
        latitude: 41.027054,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
];

const GroupIcon: React.FunctionComponent<{
    groupName: string;
    locations?: Region[];
    setLocations: (newLocations: Region[]) => void;
}> = ({ groupName, locations = coordinates, setLocations }) => (
    <TouchableWithoutFeedback onPress={() => setLocations(locations)}>
        <View style={styles.groupContainer}>
            <Image style={styles.groupIcon} />
            <Name name={groupName} />
        </View>
    </TouchableWithoutFeedback>
);

export default GroupIcon;
