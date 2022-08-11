import React, { useEffect, useRef, useState } from 'react';
import { View, PanResponder, Dimensions, Text } from 'react-native';
import { Region } from 'react-native-maps';
import SlideCardTemplate from '../SlideCardTemplate';
import ProfileImageTemplate from '../ProfileImageTemplate';

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
    startingHeight: 150,
    minHeight: 45,
    marginTopHeight: 200,
};

type UserLocation = {
    name?: string;
    location: Region;
    longitude?: number;
    latitude?: number;
};

const SlideCard: React.FunctionComponent<{
    group: UserLocation[];
    setLocations: React.Dispatch<React.SetStateAction<Region[]>>;
}> = ({ group, setLocations }) => {
    const { startingHeight, minHeight, marginTopHeight } = heightProps;
    const [bottomHeight, setBottomHeight] = useState(startingHeight);
    const prevHeight = useRef(startingHeight);
    const deviceHeight = Dimensions.get('window').height;

    useEffect(() => {
        setBottomHeight(startingHeight);
        prevHeight.current = startingHeight;
    }, [group]);

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
                    left: 0,
                    right: 0,
                }}
                panResponder={panResponder}
            >
                <View>
                    {group.map((user, key) => (
                        <View
                            key={key}
                            style={{
                                flexDirection: 'row',
                                margin: 10,
                            }}
                        >
                            <ProfileImageTemplate source="" size={50} />
                            <View
                                style={{
                                    flexDirection: 'column',
                                    marginLeft: 10,
                                }}
                            >
                                <Text style={{ fontSize: 24 }}>
                                    {user.name ?? 'Anonymous'}
                                </Text>
                                {user.location && (
                                    <Text>
                                        {user.location.longitude}{' '}
                                        {user.location.latitude}
                                    </Text>
                                )}
                                {user.longitude && user.latitude && (
                                    <Text>
                                        {user.longitude} {user.latitude}
                                    </Text>
                                )}
                            </View>
                        </View>
                    ))}
                </View>
            </SlideCardTemplate>
        </View>
    );
};

export default SlideCard;
