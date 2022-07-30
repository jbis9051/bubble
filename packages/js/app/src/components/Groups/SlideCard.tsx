import React, { useRef, useState } from 'react';
import { Animated, View, PanResponder, Dimensions } from 'react-native';
import SlideCardTemplate from '../SlideCardTemplate';
import UserIcon from './UserIcon';

import styles from './styles';

const SlideCard = () => {
    const [bottomHeight, setBottomHeight] = useState(150);
    const deviceHeight = Dimensions.get('window').height;

    const panResponder = useRef(
        PanResponder.create({
            onMoveShouldSetPanResponder: () => true,
            onPanResponderMove: (e, gestureState) => {
                let newDeviceHeight;
                if (gestureState.moveY > deviceHeight - 150) {
                    newDeviceHeight = 150;
                } else if (gestureState.moveY < 300) {
                    newDeviceHeight = deviceHeight - 300;
                } else {
                    newDeviceHeight = deviceHeight - gestureState.moveY;
                }
                setBottomHeight(newDeviceHeight);
            },
        })
    ).current;
    
    return (
        <View>
            <Animated.View>
            </Animated.View>
            <SlideCardTemplate 
                style={{ height: bottomHeight, position: 'absolute', bottom: 0, marginTop: 0 }}
                panResponder={panResponder}
            >
                <View style={styles.userView}>
                    <UserIcon name='John' />
                    <UserIcon name='Santhosh' />
                    <UserIcon name='Kevin' />
                    <UserIcon name='Kyle' />
                    <UserIcon name='Sidney' />
                    <UserIcon name='Lia' />
                </View>
            </SlideCardTemplate>
        </View>
    ) 
};

export default SlideCard;