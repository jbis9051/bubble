import React from 'react';
import { View, Text, TouchableWithoutFeedback } from 'react-native';
import ProfileImageTemplate from '../ProfileImageTemplate';

import styles from './styles';

interface Location {
    longitude: number;
    latitude: number;
}

const initialRegion = {
    longitude: -122.4324,
    latitude: 37.78825,
};

const UserIcon: React.FunctionComponent<{
    name: string;
    location?: Location;
    setLocation: (newLocation: Location) => void;
}> = ({ name, location = initialRegion, setLocation }) => (
    <TouchableWithoutFeedback onPress={() => setLocation(location)}>
        <View style={styles.userIcon}>
            <ProfileImageTemplate source="" size={80} />
            <Text style={{ marginTop: 8, fontSize: 18 }}>{name}</Text>
        </View>
    </TouchableWithoutFeedback>
);

export default UserIcon;
