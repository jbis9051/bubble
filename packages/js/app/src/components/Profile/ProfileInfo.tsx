import React, { useState, useEffect } from 'react';
import { View, Text } from 'react-native';

import ProfileImageTemplate from '../ProfileImageTemplate';

import Styles from './Styles';

const ProfileInfo = () => {
    // image, username, email will come from API

    const [name, setName] = useState('John Appleseed');
    const [email, setEmail] = useState('johnappleseed@bubble.com');
    const [source, setSource] = useState('');

    return (
        <View style={Styles.headerContent}>
            <ProfileImageTemplate 
                source={source}
                size={140}
            />
            <Text style={Styles.headerText}>{name}</Text>
            <Text
                style={{
                    fontSize: 16,
                    marginTop: 3,
                    color: '#e3e3e3',
                }}
            >
                {email}
            </Text>
        </View>
    );
};

export default ProfileInfo;