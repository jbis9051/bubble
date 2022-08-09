import React, { useEffect, useRef, useState } from 'react';
import { View, PanResponder, Dimensions, Text } from 'react-native';
import { Region } from 'react-native-maps';
import SlideCardTemplate from '../SlideCardTemplate';
import UserIcon from './UserIcon';
import GroupIcon from './GroupIcon';
import DividerLine from '../Misc/DividerLine';

import styles from './styles';

const coordinates: Region[] = [
    {
        longitude: -74.6551,
        latitude: 40.3431,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
    {
        longitude: -83.7382,
        latitude: 42.287,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
    {
        longitude: -74.0131,
        latitude: 40.7118,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
    {
        longitude: -122.009,
        latitude: 37.3346,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
    {
        longitude: -73.620071,
        latitude: 41.027054,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
];

const heightProps = {
    startingHeight: 180,
    minHeight: 70,
    marginTopHeight: 200,
};

const SlideCard: React.FunctionComponent<{
    locations: Region[];
    setLocations: (newLocations: Region[]) => void;
}> = ({ locations, setLocations }) => {
    const { startingHeight, minHeight, marginTopHeight } = heightProps;
    const [bottomHeight, setBottomHeight] = useState(startingHeight);
    const prevHeight = useRef(startingHeight);
    const deviceHeight = Dimensions.get('window').height;

    useEffect(() => {
        setBottomHeight(startingHeight);
        prevHeight.current = startingHeight;
    }, [locations]);

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
                    <Text style={styles.peopleHeading}>Groups</Text>
                    <DividerLine />
                    <View style={styles.groupView}>
                        <GroupIcon
                            groupName="Group 1"
                            setLocations={setLocations}
                        />
                        <GroupIcon
                            groupName="Group 2"
                            locations={[
                                coordinates[0],
                                coordinates[2],
                                coordinates[4],
                            ]}
                            setLocations={setLocations}
                        />
                    </View>
                    <View style={styles.userView}>
                        <UserIcon
                            name="John"
                            locations={[coordinates[0]]}
                            setLocations={setLocations}
                        />
                        <UserIcon
                            name="Santhosh"
                            locations={[coordinates[1]]}
                            setLocations={setLocations}
                        />
                        <UserIcon
                            name="Kevin"
                            locations={[coordinates[2]]}
                            setLocations={setLocations}
                        />
                        <UserIcon
                            name="Kyle"
                            locations={[coordinates[3]]}
                            setLocations={setLocations}
                        />
                        <UserIcon
                            name="Sidney"
                            locations={[coordinates[4]]}
                            setLocations={setLocations}
                        />
                        <UserIcon name="Lia" setLocations={setLocations} />
                    </View>
                </View>
            </SlideCardTemplate>
        </View>
    );
};

export default SlideCard;
