import React, { useState } from 'react';
import { View, TouchableWithoutFeedback, Dimensions } from 'react-native';
import { Region } from 'react-native-maps';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { BlurView } from '@react-native-community/blur';
import Map from '../components/Groups/Map';
import SearchBar from '../components/Groups/SearchBar';
import SlideCard from '../components/Groups/SlideCard';
import SearchGroups from '../components/Groups/SearchGroups';

const initialRegions: UserLocation[] = [
    {
        name: 'Anonymous',
        location: {
            longitude: -122.4324,
            latitude: 37.78825,
            latitudeDelta: 0.015,
            longitudeDelta: 0.015,
        },
    },
];

type UserLocation = {
    name?: string;
    location: Region;
};

const GroupsScreen = () => {
    const [locations, setLocations] = useState([initialRegions[0].location]);
    const [isFocused, setFocus] = useState(false);
    const [isBlurred, setBlur] = useState(false);
    const [search, setSearch] = useState('');
    const [activeGroup, setActiveGroup] =
        useState<UserLocation[]>(initialRegions);
    const insets = useSafeAreaInsets();

    return (
        <View style={{ flex: 1 }}>
            <Map locations={locations} />
            <SlideCard group={activeGroup} setLocations={setLocations} />
            {isFocused && (
                <TouchableWithoutFeedback
                    onPress={() => {
                        setFocus(false);
                        setBlur(false);
                    }}
                >
                    <BlurView
                        blurType="light"
                        style={{
                            position: 'absolute',
                            top: 0,
                            right: 0,
                            left: 0,
                            bottom: 0,
                        }}
                    />
                </TouchableWithoutFeedback>
            )}
            <SearchBar
                insets={insets}
                search={search}
                isBlurred={isBlurred}
                isFocused={isFocused}
                setFocus={setFocus}
                setSearch={setSearch}
                setBlur={setBlur}
            />
            {isFocused && (
                <SearchGroups
                    insets={insets}
                    search={search}
                    setLocations={setLocations}
                    setFocus={setFocus}
                    setSearch={setSearch}
                    setBlur={setBlur}
                    setGroup={setActiveGroup}
                />
            )}
        </View>
    );
};

export default GroupsScreen;
