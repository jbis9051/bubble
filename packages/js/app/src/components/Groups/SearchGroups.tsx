import React from 'react';
import { View, ScrollView, Dimensions } from 'react-native';
import { EdgeInsets } from 'react-native-safe-area-context';
import { Region } from 'react-native-maps';
import { BlurView } from '@react-native-community/blur';
import GroupIcon from './GroupIcon';
import styles from './styles';

type UserLocation = {
    name?: string;
    location: Region;
};

const coordinates: UserLocation[] = [
    {
        name: 'Johnny',
        location: {
            longitude: -74.6551,
            latitude: 40.3431,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
    {
        name: 'Santhosh',
        location: {
            longitude: -83.7382,
            latitude: 42.287,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
    {
        name: 'Kyle',
        location: {
            longitude: -74.0131,
            latitude: 40.7118,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
    {
        name: 'Sidney',
        location: {
            longitude: -122.009,
            latitude: 37.3346,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
    {
        name: 'Lia',
        location: {
            longitude: -73.620071,
            latitude: 41.027054,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
];

const SearchGroups: React.FunctionComponent<{
    insets: EdgeInsets;
    search: string;
    setLocations: React.Dispatch<React.SetStateAction<Region[]>>;
    setFocus: React.Dispatch<React.SetStateAction<boolean>>;
    setBlur: React.Dispatch<React.SetStateAction<boolean>>;
    setSearch: React.Dispatch<React.SetStateAction<string>>;
    setGroup: React.Dispatch<React.SetStateAction<UserLocation[]>>;
    setUserGroup: React.Dispatch<React.SetStateAction<UserLocation[]>>;
}> = ({
    insets,
    search,
    setLocations,
    setFocus,
    setBlur,
    setSearch,
    setGroup,
    setUserGroup,
}) => {
    const deviceWidth = Dimensions.get('window').width;
    const groups = [
        {
            groupName: 'Group 1',
        },
        {
            groupName: 'Group 2',
            locations: [coordinates[0], coordinates[2], coordinates[4]],
        },
        {
            groupName: 'Group 3',
        },
        {
            groupName: 'Group 4',
        },
        {
            groupName: 'Group 1',
        },
        {
            groupName: 'Group 2',
            locations: [coordinates[0], coordinates[2], coordinates[4]],
        },
        {
            groupName: 'Group 3',
        },
        {
            groupName: 'Group 4',
        },
        {
            groupName: 'Group 3',
        },
        {
            groupName: 'Group 4',
        },
        {
            groupName: 'Group 3',
        },
        {
            groupName: 'Group 4',
        },
    ];

    return (
        <BlurView
            blurType="xlight"
            style={{
                width: deviceWidth - 30,
                backgroundColor: 'transparent',
                position: 'absolute',
                top: insets.top + 50,
                bottom: insets.bottom,
                left: 15,
                borderRadius: 15,
            }}
        >
            <ScrollView
                showsVerticalScrollIndicator={false}
                keyboardShouldPersistTaps={'handled'}
                onScrollBeginDrag={() => setBlur(true)}
            >
                <View style={styles.groupView}>
                    {groups
                        .filter((group) =>
                            search.length > 0
                                ? group.groupName.includes(search)
                                : true
                        )
                        .map((group, key) => (
                            <GroupIcon
                                groupName={group.groupName}
                                locations={group.locations}
                                setLocations={setLocations}
                                setFocus={setFocus}
                                setSearch={setSearch}
                                setBlur={setBlur}
                                setGroup={setGroup}
                                setUserGroup={setUserGroup}
                                lightText={false}
                                key={key}
                            />
                        ))}
                </View>
            </ScrollView>

        </BlurView>
    );
};

export default SearchGroups;
