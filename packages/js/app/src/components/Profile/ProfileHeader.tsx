import React from 'react';
import { View, Image, Text } from 'react-native';
import ProfileImageTemplate from '../ProfileImageTemplate';

import Styles from './Styles';

const ProfileHeader: React.FunctionComponent<{ profileImage: string }> = ({ profileImage }) => (
    <View style={Styles.header}>
        <ProfileImageTemplate 
            source={profileImage}
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
