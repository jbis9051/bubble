import React from 'react';
import { View } from 'react-native';
import SlideCardTemplate from '../SlideCardTemplate';
import UserIcon from './UserIcon';

import styles from './styles';

const SlideCard = () => (
    <SlideCardTemplate style={{marginTop: 600}}>
        <View style={styles.userView}>
            <UserIcon name='John' />
            <UserIcon name='Santhosh' />
            <UserIcon name='Kevin' />
            <UserIcon name='Kyle' />
            <UserIcon name='Sidney' />
            <UserIcon name='Lia' />
        </View>
    </SlideCardTemplate>
);

export default SlideCard;