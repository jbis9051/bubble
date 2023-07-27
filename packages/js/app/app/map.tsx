import {
    Pressable,
    SafeAreaView,
    StyleSheet,
    Text,
    TouchableOpacity,
    View,
} from 'react-native';
import MapView, { LatLng, Marker } from 'react-native-maps';
import { Ionicons, MaterialCommunityIcons } from '@expo/vector-icons';
import { Link } from 'expo-router';
import { useEffect, useRef, useState } from 'react';
import { observer } from 'mobx-react-lite';
import { UserOut, Uuid } from '@bubble/react-native-bubble-rust';
import { getInitials } from '../lib/formatText';
import StyledText from '../components/StyledText';
import Colors from '../constants/Colors';
import MainStore from '../stores/MainStore';
import FrontendInstanceStore from '../stores/FrontendInstanceStore';

interface CustomMarkerProps {
    coordinate: LatLng;
    user: UserOut;
    selected?: boolean;
    onPress?: () => void;
}

function CustomMarker({ coordinate, user }: CustomMarkerProps) {
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
                    backgroundColor: Colors.colors.primaryPaper,
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
                        backgroundColor: Colors.colors.secondaryPaper,
                    }}
                >
                    <StyledText
                        nomargin
                        style={{ color: Colors.colors.primaryPaper }}
                    >
                        {getInitials(user.name)}
                    </StyledText>
                </View>
            </View>
        </Marker>
    );
}

interface UserLocation {
    user: UserOut;
    client_uuid: Uuid;
    longitude: number;
    latitude: number;
    timestamp: Date;
}

const Map = observer(() => {
    const [memberLocations, setMemberLocations] = useState<UserLocation[]>([]);

    const currentTimer = useRef<NodeJS.Timer | null>(null);

    async function updateLocations() {
        console.log('updateLocations called');
        const group = MainStore.current_group;
        if (!group) {
            currentTimer.current = setTimeout(updateLocations, 2000);
            return;
        }
        const locations = await Promise.all(
            Object.entries(group.members).map(
                async ([user_uuid, user_group_info]) => {
                    const location =
                        await FrontendInstanceStore.instance.get_location(
                            group.uuid,
                            user_group_info.clients[0],
                            Date.now(),
                            1
                        );
                    if (location.length === 0) {
                        return null;
                    }
                    return {
                        user: user_group_info.info,
                        client_uuid: user_group_info.clients[0],
                        longitude: location[0].longitude,
                        latitude: location[0].latitude,
                        timestamp: new Date(location[0].timestamp),
                    } as UserLocation;
                }
            )
        );
        setMemberLocations(
            locations.filter((location) => location !== null) as UserLocation[]
        );
        currentTimer.current = setTimeout(updateLocations, 2000);
    }

    useEffect(() => {
        if (!currentTimer.current) {
            updateLocations();
        }

        return () => {
            if (currentTimer.current) {
                clearTimeout(currentTimer.current);
                currentTimer.current = null;
            }
        };
    }, []);

    return (
        <View>
            <MapView style={styles.map}>
                {memberLocations.map((userLocation) => (
                    <CustomMarker
                        key={userLocation.client_uuid}
                        coordinate={{
                            latitude: userLocation.latitude,
                            longitude: userLocation.longitude,
                        }}
                        user={userLocation.user}
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
                    <Link href="/groups" asChild>
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
                            <Text numberOfLines={1} style={{ width: '85%' }}>
                                {MainStore.current_group?.name}
                            </Text>
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
                    <Link href="/groupSettings" asChild>
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
});

export default Map;

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
