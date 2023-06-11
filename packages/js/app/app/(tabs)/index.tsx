import { Pressable, SafeAreaView, StyleSheet, Text, View } from 'react-native';
import MapView, { LatLng, Marker } from 'react-native-maps';
import { Ionicons, MaterialCommunityIcons } from '@expo/vector-icons';
import { Link } from 'expo-router';

interface CustomMarkerProps {
    coordinate: LatLng;
    selected?: boolean;
    onPress?: () => void;
}
function CustomMarker(props: CustomMarkerProps) {
    const { coordinate } = props;

    return (
        <Marker coordinate={coordinate}>
            <View
                style={{
                    height: 50,
                    width: 50,
                    borderRadius: 50,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    backgroundColor: 'white',
                }}
            >
                <Text>Hello</Text>
            </View>
        </Marker>
    );
}

export default function MapScreen() {
    return (
        <View>
            <MapView style={styles.map}>
                <CustomMarker
                    coordinate={{ latitude: 37.427475, longitude: -122.169716 }}
                />
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
                    <Link href="/bubbleListModal" asChild>
                        <Pressable
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
                            <Text>Go Blue's Bubble</Text>
                        </Pressable>
                    </Link>
                </View>
                <View
                    style={{
                        width: '20%',
                        height: '100%',
                        paddingHorizontal: 12,
                    }}
                >
                    <View
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
                    </View>
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
