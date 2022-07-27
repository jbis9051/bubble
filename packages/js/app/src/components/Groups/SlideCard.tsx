import React from 'react';
import { View, Modal } from 'react-native';
import SlideCardTemplate from '../SlideCardTemplate';
import UserIcon from './UserIcon';

import styles from './styles';

const SlideCard = () => (
    <Modal
        transparent
    >
        <View style={styles.userView}>
            <UserIcon name='John' />
            <UserIcon name='Santhosh' />
            <UserIcon name='Kevin' />
            <UserIcon name='Kyle' />
            <UserIcon name='Sidney' />
            <UserIcon name='Lia' />
        </View>
    </Modal>
    // <SlideCardTemplate style={{marginTop: 600 }}>
        
    // </SlideCardTemplate>
);

export default SlideCard;