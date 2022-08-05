import React, { useEffect, useRef, useState } from 'react';
import { View, PanResponder, Dimensions, Text } from 'react-native';
import SlideCardTemplate from '../SlideCardTemplate';
import UserIcon from './UserIcon';
import DividerLine from '../Misc/DividerLine';

import styles from './styles';

interface Location {
    longitude: number;
    latitude: number;
}

const coordinates = [
    {
        longitude: -74.6551,
        latitude: 40.3431,
    },
    {
        longitude: -83.7382,
        latitude: 42.287,
    },
    {
        longitude: -74.0131,
        latitude: 40.7118,
    },
    {
        longitude: -122.009,
        latitude: 37.3346,
    },
    {
        longitude: -73.620071,
        latitude: 41.027054,
    },
];

const heightProps = {
    startingHeight: 180,
    minHeight: 70,
    marginTopHeight: 200,
};

const SlideCard: React.FunctionComponent<{
    marker: Location;
    setLocation: [
        (newLocation: Location) => void,
        (newLocation: Location) => void
    ];
}> = ({ marker, setLocation }) => {
    const { startingHeight, minHeight, marginTopHeight } = heightProps;

    const [bottomHeight, setBottomHeight] = useState(startingHeight);
    const prevHeight = useRef(startingHeight);
    const deviceHeight = Dimensions.get('window').height;

    useEffect(() => {
        setBottomHeight(startingHeight);
        prevHeight.current = startingHeight;
    }, [marker]);

    const panResponder = useRef(
        PanResponder.create({
            onMoveShouldSetPanResponder: () => true,
            onMoveShouldSetPanResponderCapture: () => true,
            onPanResponderMove: (_e, gestureState) => {
                const newDeviceHeight = Math.min(
                    deviceHeight - marginTopHeight,
                    Math.max(prevHeight.current - gestureState.dy, minHeight)
                );
                setBottomHeight(newDeviceHeight);
            },
            onPanResponderRelease(_e, gestureState) {
                prevHeight.current -= gestureState.dy;
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
                            location={coordinates[0]}
                            setLocation={setLocation}
                        />
                        <UserIcon
                            name="Santhosh"
                            location={coordinates[1]}
                            setLocation={setLocation}
                        />
                        <UserIcon
                            name="Kevin"
                            location={coordinates[2]}
                            setLocation={setLocation}
                        />
                        <UserIcon
                            name="Kyle"
                            location={coordinates[3]}
                            setLocation={setLocation}
                        />
                        <UserIcon
                            name="Sidney"
                            location={coordinates[4]}
                            setLocation={setLocation}
                        />
                        <UserIcon name="Lia" setLocation={setLocation} />
                    </View>
                </View>
            </SlideCardTemplate>
        </View>
    );
};

export default SlideCard;
