import React from 'react';
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

const ChildrenComponent = (
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
                detail="johnappleseed@bubble.com"
                icon={mailIcon}
            />
            <InfoCard title="Phone" detail="123-456-7890" icon={phoneIcon} />
            <InfoCard title="Username" detail="johnappleseed" icon={userIcon} />
            <InfoCard
                title="Last seen"
                detail="San Franscico, California"
                icon={locationIcon}
            />
        </View>
    </View>
);

const SlideCard = () => <SlideCardTemplate children={ChildrenComponent} />;

export default SlideCard;
