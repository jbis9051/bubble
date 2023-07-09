import { Pressable, SafeAreaView, StyleSheet, Text, TouchableOpacity, View } from 'react-native';
import MapView, { LatLng, Marker } from 'react-native-maps';
import { Ionicons, MaterialCommunityIcons } from '@expo/vector-icons';
import { Link } from 'expo-router';
import { useSelector } from 'react-redux';
import { selectCurrentGroup } from '../../redux/slices/groupSlice';
import { MapService } from '../../lib/bubbleApi/location';
import { useContext, useEffect, useRef, useState } from 'react';
import { UserLocal } from '../../lib/bubbleApi/user';
import { getInitials } from '../../lib/formatText';
import { ThemeContext } from '../../lib/Context';
import StyledText from '../../components/StyledText';

interface CustomMarkerProps {
    coordinate: LatLng;
    user: UserLocal;
    selected?: boolean;
    onPress?: () => void;
}

function CustomMarker(props: CustomMarkerProps) {
    const { coordinate, user } = props;
    const theme = useContext(ThemeContext);

    return (
        <Marker coordinate={coordinate}>
            <View
                style={{
                    height: 60,
                    width: 60,
                    borderRadius: 50,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    backgroundColor: theme.colors.primaryPaper,
                }}
            >
                <View
                    style={{
                        height: 50,
                        width: 50,
                        borderRadius: 50,
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        backgroundColor: theme.colors.secondaryPaper,
                    }}
                >
                    <StyledText nomargin style={{ color: theme.colors.primaryPaper }}>{getInitials(user.name)}</StyledText>
                </View>
            </View>
        </Marker>
    );
}

interface UserWithLocation extends UserLocal {
    location: LatLng;
}

export default function MapScreen() {
    const activeGroup = useSelector(selectCurrentGroup);
    const [memberLocations, setMemberLocations] = useState<UserWithLocation[]>([]);

    useEffect(() => {
        const interval = setInterval(async () => {
            if (activeGroup) {
                const memberLocations = await Promise.all(
                    activeGroup.members.map(async (member) => {
                        const locations = await MapService
                            .get_location(
                                activeGroup.uuid,
                                member.primary_client_uuid,
                                Date.now() + 1,
                                1);
                        ;
                        return { ...member, location: locations[0].coordinate };
                    })
                )
                setMemberLocations(memberLocations);
            }
        }, 2000);

        return () => clearInterval(interval);
    }, [activeGroup])

    return (
        <View>
            <MapView style={styles.map}>
                {memberLocations.map(mLoc => (
                    <CustomMarker
                        coordinate={mLoc.location}
                        user={mLoc}
                    />
                ))}
            </MapView>
            <SafeAreaView
                style={{
                    position: 'absolute',
                    display: 'flex',
                    flexDirection: 'row',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                }}
            >
                <View
                    style={{
                        width: '80%',
                        height: 50,
                        paddingHorizontal: 10,
                    }}
                >
                    <Link href="/allGroupsModal" asChild>
                        <TouchableOpacity
                            style={{
                                backgroundColor: 'white',
                                borderRadius: 30,
                                height: '100%',
                                width: '100%',
                                display: 'flex',
                                flexDirection: 'row',
                                alignItems: 'center',
                                paddingHorizontal: 15,
                                gap: 15,
                            }}
                        >
                            <MaterialCommunityIcons
                                name="chart-bubble"
                                size={24}
                                color="black"
                            />
                            <Text numberOfLines={1} style={{ width: "85%" }}>{activeGroup?.name}</Text>
                        </TouchableOpacity>
                    </Link>
                </View>
                <View
                    style={{
                        width: '20%',
                        height: '100%',
                        paddingHorizontal: 12,
                    }}
                >
                    <Link href="/groupSettingsModal" asChild>
                        <TouchableOpacity
                            style={{
                                backgroundColor: 'white',
                                borderRadius: 9999,
                                width: '100%',
                                aspectRatio: 1,
                                display: 'flex',
                                alignItems: 'center',
                                justifyContent: 'center',
                            }}
                        >
                            <Ionicons name="settings" size={24} color="black" />
                        </TouchableOpacity>
                    </Link>
                </View>
            </SafeAreaView>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        alignItems: 'center',
        justifyContent: 'center',
    },
    title: {
        fontSize: 20,
        fontWeight: 'bold',
    },
    separator: {
        marginVertical: 30,
        height: 1,
        width: '80%',
    },
    map: {
        width: '100%',
        height: '100%',
    },
});
