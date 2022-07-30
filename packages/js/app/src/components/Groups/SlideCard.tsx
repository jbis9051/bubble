import React, { useRef, useState } from 'react';
import { View, PanResponder, Dimensions, Text } from 'react-native';
import SlideCardTemplate from '../SlideCardTemplate';
import UserIcon from './UserIcon';
import DividerLine from '../Misc/DividerLine';

import styles from './styles';

const SlideCard: React.FunctionComponent<{
    minHeight: number;
    marginTopHeight: number;
}> = ({ minHeight, marginTopHeight }) => {
    const [bottomHeight, setBottomHeight] = useState(150);
    const deviceHeight = Dimensions.get('window').height;

    const panResponder = useRef(
        PanResponder.create({
            onMoveShouldSetPanResponder: () => true,
            onPanResponderMove: (e, gestureState) => {
                let newDeviceHeight;
                if (gestureState.moveY > deviceHeight - minHeight) {
                    newDeviceHeight = minHeight;
                } else if (gestureState.moveY < marginTopHeight) {
                    newDeviceHeight = deviceHeight - marginTopHeight;
                } else {
                    newDeviceHeight = deviceHeight - gestureState.moveY;
                }
                setBottomHeight(newDeviceHeight);
            },
        })
    ).current;

    return (
        <View>
            <SlideCardTemplate
                style={{
                    height: bottomHeight,
                    position: 'absolute',
                    bottom: 0,
                    marginTop: 0,
                }}
                panResponder={panResponder}
            >
                <View>
                    <Text style={styles.peopleHeading}>People</Text>
                    <DividerLine />
                    <View style={styles.userView}>
                        <UserIcon name="John" />
                        <UserIcon name="Santhosh" />
                        <UserIcon name="Kevin" />
                        <UserIcon name="Kyle" />
                        <UserIcon name="Sidney" />
                        <UserIcon name="Lia" />
                    </View>
                </View>
            </SlideCardTemplate>
        </View>
    );
};

export default SlideCard;
