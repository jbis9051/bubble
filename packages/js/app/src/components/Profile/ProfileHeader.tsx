import React from 'react';
import { View, Image, Text } from 'react-native';
import ProfileImageTemplate from '../ProfileImageTemplate';

import Styles from './Styles';

const ProfileHeader = () => (
    <View style={Styles.header}>
        <ProfileImageTemplate 
            source=''
            size={140}
        />
        <Text style={Styles.headerText}>John Appleseed</Text>
        <Text
            style={{
                fontSize: 16,
                marginTop: 3,
                color: '#e3e3e3',
            }}
        >
            johnappleseed@bubble.com
        </Text>
    </View>
);

export default ProfileHeader;
