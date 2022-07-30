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

const nycCoordinates = {
    longitude: -74.0134,
    latitude: 40.7217,
};

const SlideCard: React.FunctionComponent<{
    startingHeight: number;
    minHeight: number;
    marginTopHeight: number;
    setLocation: (newLocation: Location) => void;
}> = ({ startingHeight, minHeight, marginTopHeight, setLocation }) => {
    const [bottomHeight, setBottomHeight] = useState(startingHeight);
    const deviceHeight = Dimensions.get('window').height;

    const panResponder = useRef(
        PanResponder.create({
            onMoveShouldSetPanResponder: () => true,
            onPanResponderMove: (_e, gestureState) => {
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
                        <UserIcon
                            name="John"
                            location={nycCoordinates}
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
