import React from 'react';
import { View, Text } from 'react-native';
import ProfileImageTemplate from '../ProfileImageTemplate';

import styles from './styles';

const UserIcon: React.FunctionComponent<{ name: string }> = ({ name }) => (
    <View style={styles.userIcon}>
        <ProfileImageTemplate source='' size={80} />
        <Text style={{marginTop: 8, fontSize: 18}}>{name}</Text>
    </View>
)

export default UserIcon;