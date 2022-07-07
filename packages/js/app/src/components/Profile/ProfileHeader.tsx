import React from 'react';
import {
    View,
    Image,
    Text
} from 'react-native';

import Styles from './Styles';

const ProfileHeader = () => (
    <View style={Styles.header}>
        <Image 
            source={require('../../../assets/user.jpeg')}
            style={Styles.profileImage}
        />
        <Text style={Styles.headerText}>
            John Appleseed
        </Text>
        <Text style={{
            fontSize: 16,
            marginTop: 3,
            color: '#e3e3e3'
        }}>
            johnappleseed@bubble.com
        </Text>
    </View>
);

export default ProfileHeader;