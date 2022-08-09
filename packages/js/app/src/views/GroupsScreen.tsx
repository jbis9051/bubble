import React, { useState } from 'react';
import { View, TouchableWithoutFeedback, Dimensions } from 'react-native';
import { Region } from 'react-native-maps';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { BlurView } from '@react-native-community/blur';
import Map from '../components/Groups/Map';
import SearchBar from '../components/Groups/SearchBar';
import SlideCard from '../components/Groups/SlideCard';
import SearchGroups from '../components/Groups/SearchGroups';

const initialRegions: Region[] = [
    {
        longitude: -122.4324,
        latitude: 37.78825,
        latitudeDelta: 0.015,
        longitudeDelta: 0.015,
    },
];

const GroupsScreen = () => {
    const [locations, setLocations] = useState(initialRegions);
    const [isFocused, setFocus] = useState(false);
    const [search, setSearch] = useState('');
    const insets = useSafeAreaInsets();
    const deviceWidth = Dimensions.get('window').width;

    return (
        <View style={{ flex: 1 }}>
            <Map locations={locations} />
            <SlideCard locations={locations} setLocations={setLocations} />
            {isFocused && (
                <TouchableWithoutFeedback onPress={() => setFocus(false)}>
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
                isFocused={isFocused}
                setFocus={setFocus}
                setSearch={setSearch}
            />
            {isFocused && (
                <SearchGroups
                    insets={insets}
                    search={search}
                    setLocations={setLocations}
                    setFocus={setFocus}
                    setSearch={setSearch}
                />
            )}
        </View>
    );
};

export default GroupsScreen;
