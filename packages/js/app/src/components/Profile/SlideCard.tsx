import React, { useEffect, useState } from 'react';
import { View, Text } from 'react-native';

import {
    faEnvelope as mailIcon,
    faPhone as phoneIcon,
    faLocationDot as locationIcon,
    faUser as userIcon,
} from '@fortawesome/free-solid-svg-icons';
import SlideCardTemplate from '../SlideCardTemplate';
import Map from './Map';
import InfoCard from './InfoCard';

import Styles from './Styles';


const ChildrenComponent = () => {
    const [email, setEmail] = useState('johnappleseed@bubble.com');
    const [phone, setPhone] = useState('123-456-7890');
    const [username, setUsername] = useState('johnappleseed');
    const [lastLocation, setLastLocation] = useState('San Francisco');
    const [mapType, setMapType] = useState('Street');

    return (
        <View>
            <Text style={Styles.heading}>Map</Text>
            <Map />
            <Text style={Styles.heading}>Information</Text>
            <View
                style={{
                    borderRadius: 15,
                    backgroundColor: '#ffffff',
                    paddingBottom: 24,
                }}
            >
                <InfoCard
                    title="Email"
                    detail={email}
                    icon={mailIcon}
                />
                <InfoCard title="Phone" detail={phone} icon={phoneIcon} />
                <InfoCard title="Username" detail={username} icon={userIcon} />
                <InfoCard
                    title="Last seen"
                    detail={lastLocation}
                    icon={locationIcon}
                />
            </View>
        </View>
    );
    
};

const SlideCard = () => {
    return (
        <SlideCardTemplate>
            <ChildrenComponent />
        </SlideCardTemplate>
    );
}

export default SlideCard;
