import React, { useEffect, useRef, useState } from 'react';
import { View, PanResponder, Dimensions } from 'react-native';
import { Region } from 'react-native-maps';
import SlideCardTemplate from '../SlideCardTemplate';
import UserInfo from './UserInfo';

const heightProps = {
    startingHeight: 100,
    minHeight: 100,
    marginTopHeight: 200,
};

type UserLocation = {
    name?: string;
    location: Region;
};

const SlideCard: React.FunctionComponent<{
    userGroup: UserLocation[];
    group: UserLocation[];
    locations: Region[];
    setLocations: React.Dispatch<React.SetStateAction<Region[]>>;
    setGroups: React.Dispatch<React.SetStateAction<UserLocation[]>>;
    setUserGroup: React.Dispatch<React.SetStateAction<UserLocation[]>>;
}> = ({
    userGroup,
    group,
    locations,
    setLocations,
    setGroups,
    setUserGroup,
}) => {
    const { startingHeight, minHeight, marginTopHeight } = heightProps;
    const [bottomHeight, setBottomHeight] = useState(startingHeight);
    const prevHeight = useRef(startingHeight);
    const deviceHeight = Dimensions.get('window').height;

    useEffect(() => {
        setBottomHeight(startingHeight);
        prevHeight.current = startingHeight;
    }, [group, locations, userGroup]);

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
                    {userGroup.map((user, key) => (
                        <UserInfo
                            user={user}
                            key={key}
                            setGroups={setGroups}
                            setLocations={setLocations}
                        />
                    ))}
                </View>
            </SlideCardTemplate>
        </View>
    );
};

export default SlideCard;
