import React from 'react';
import { View, Image } from 'react-native';
import { Region } from 'react-native-maps';
import styles from './styles';
import Name from './Name';

const GroupIcon: React.FunctionComponent<{
    groupName: string;
    locations?: Region[];
}> = ({ groupName, locations }) => (
    <View style={styles.groupContainer}>
        <Image style={styles.groupIcon} />
        <Name name={groupName} />
    </View>
);

export default GroupIcon;
