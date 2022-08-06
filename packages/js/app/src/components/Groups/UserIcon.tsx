import React from 'react';
import { View, TouchableWithoutFeedback } from 'react-native';
import { Region } from 'react-native-maps';
import ProfileImageTemplate from '../ProfileImageTemplate';
import Name from './Name';

import styles from './styles';

const initialRegion: Region = {
    longitude: -122.4324,
    latitude: 37.78825,
    latitudeDelta: 0.015,
    longitudeDelta: 0.015,
};

const UserIcon: React.FunctionComponent<{
    name: string;
    locations?: Region[];
    setLocations: (newLocations: Region[]) => void;
}> = ({ name, locations = [initialRegion], setLocations }) => (
    <TouchableWithoutFeedback onPress={() => setLocations(locations)}>
        <View style={styles.userIcon}>
            <ProfileImageTemplate source="" size={60} />
            <Name name={name} />
        </View>
    </TouchableWithoutFeedback>
);

export default UserIcon;
