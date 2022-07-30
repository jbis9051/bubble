import React, { useRef, useState } from 'react';
import { View, PanResponder, Dimensions, Text } from 'react-native';
import SlideCardTemplate from '../SlideCardTemplate';
import UserIcon from './UserIcon';
import DividerLine from '../Misc/DividerLine';

import styles from './styles';

interface Location {
    longitude: number;
    latitude: number;
}

const coordinate = {
    longitude: -74.6551,
    latitude: 40.3431,
};

const SlideCard: React.FunctionComponent<{
    startingHeight: number;
    minHeight: number;
    marginTopHeight: number;
    setLocation: (newLocation: Location) => void;
}> = ({ startingHeight, minHeight, marginTopHeight, setLocation }) => {
    const [bottomHeight, setBottomHeight] = useState(startingHeight);
    const deviceHeight = Dimensions.get('window').height;
    let prevDeviceHeight = startingHeight;

    const panResponder = useRef(
        PanResponder.create({
            onMoveShouldSetPanResponder: () => true,
            onMoveShouldSetPanResponderCapture: () => true,
            onPanResponderMove: (_e, gestureState) => {
                const newDeviceHeight = Math.min(
                    deviceHeight - marginTopHeight,
                    Math.max(prevDeviceHeight - gestureState.dy, minHeight)
                );
                setBottomHeight(newDeviceHeight);
            },
            onPanResponderRelease(_e, gestureState) {
                prevDeviceHeight -= gestureState.dy;
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
                        <UserIcon
                            name="John"
                            location={coordinate}
                            setLocation={setLocation}
                        />
                        <UserIcon name="Santhosh" setLocation={setLocation} />
                        <UserIcon name="Kevin" setLocation={setLocation} />
                        <UserIcon name="Kyle" setLocation={setLocation} />
                        <UserIcon name="Sidney" setLocation={setLocation} />
                        <UserIcon name="Lia" setLocation={setLocation} />
                    </View>
                </View>
            </SlideCardTemplate>
        </View>
    );
};

export default SlideCard;
