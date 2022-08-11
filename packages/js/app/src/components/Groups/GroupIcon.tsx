import React from 'react';
import { View, Image, TouchableWithoutFeedback, Alert } from 'react-native';
import { Region } from 'react-native-maps';
import styles from './styles';
import Name from './Name';

const coordinates: UserLocation[] = [
    {
        name: 'Johnny',
        location: {
            longitude: -74.6551,
            latitude: 40.3431,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
    {
        name: 'Santhosh',
        location: {
            longitude: -83.7382,
            latitude: 42.287,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
    {
        name: 'Kyle',
        location: {
            longitude: -74.0131,
            latitude: 40.7118,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
    {
        name: 'Sidney',
        location: {
            longitude: -122.009,
            latitude: 37.3346,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
    {
        name: 'Lia',
        location: {
            longitude: -73.620071,
            latitude: 41.027054,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
];

type UserLocation = {
    name?: string;
    location: Region;
};

const GroupIcon: React.FunctionComponent<{
    groupName: string;
    locations?: UserLocation[];
    lightText?: boolean;
    setLocations: React.Dispatch<React.SetStateAction<Region[]>>;
    setFocus?: React.Dispatch<React.SetStateAction<boolean>>;
    setBlur?: React.Dispatch<React.SetStateAction<boolean>>;
    setSearch?: React.Dispatch<React.SetStateAction<string>>;
    setGroup?: React.Dispatch<React.SetStateAction<UserLocation[]>>;
}> = ({
    groupName,
    locations = coordinates,
    lightText,
    setLocations,
    setFocus,
    setBlur,
    setSearch,
    setGroup,
}) => (
    <TouchableWithoutFeedback
        onPress={() => {
            setLocations(
                Object.values(locations).map(
                    (userLocation) => userLocation.location
                )
            );
            if (setFocus && setSearch && setBlur && setGroup) {
                setFocus(false);
                setBlur(false);
                setSearch('');
                setGroup(locations);
            }
        }}
    >
        <View style={styles.groupContainer}>
            <Image style={styles.groupIcon} />
            <Name name={groupName} lightText={lightText} />
        </View>
    </TouchableWithoutFeedback>
);

export default GroupIcon;
