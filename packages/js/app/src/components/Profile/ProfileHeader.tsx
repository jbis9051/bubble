import React from 'react';
import { View, Text } from 'react-native';
import ProfileImageTemplate from '../ProfileImageTemplate';
import ProfileInfo from './ProfileInfo';

import Styles from './Styles';

const ProfileHeader = () => (
    <View style={Styles.header}>
        <ProfileInfo />
    </View>
);

export default ProfileHeader;
