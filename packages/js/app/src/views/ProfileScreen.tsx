import React from 'react';
import { 
    View,
    ScrollView,
    Text, 
    Image, 
    StyleSheet
} from 'react-native';
import MapView from 'react-native-maps';
import { SafeAreaView } from 'react-native-safe-area-context';
const styles = StyleSheet.create({
    shadow: {
        shadowColor: '#171717',
        shadowOffset: {width: -3, height: 10},
        shadowOpacity: 0.8,
        shadowRadius: 10,
    },
    map: {
        ...StyleSheet.absoluteFillObject,
    },
});

const HeaderComponent = () => (
    <View
        style={{
            backgroundColor: '#ffffff',
            alignItems: 'center',
            flexDirection: 'column',
            padding: 10,
            ...styles.shadow
        }}
    >
        <Image 
            source={require('../../assets/user.jpeg')}
            style={{
                height: 150,
                width: 150,
                borderRadius: 100,
                borderWidth: 3,
                borderColor: '#d3d3d3',
                ...styles.shadow
            }}
        />
        <Text
            style={{
                fontSize: 36
            }}
        >
            Johnny Ramirez
        </Text>
    </View>
);



const MapComponent = () => (
    <View>
        <MapView
            initialRegion={{
                latitude: 37.78825,
                longitude: -122.4324,
                latitudeDelta: 0.0922,
                longitudeDelta: 0.0421,
            }}
            style={{
                height: 250,
            }}
        />
    </View>
);

const InfoComponent = () => (
    <View
        style={{
            justifyContent: 'center',
            padding: 20,
            backgroundColor: '#ffffff',
            borderBottomColor: '#d3d3d3',
            borderBottomWidth: 2
        }}
    >
        <Text 
            style={{
                fontSize: 20
            }}
        >Last Seen: San Francisco, California</Text>
    </View>
)

const ProfileScreen = () => (
    <View>
        <HeaderComponent />
        <ScrollView>
            <MapComponent />
            <InfoComponent />
            <InfoComponent />
            <InfoComponent />
            <InfoComponent />
        </ScrollView>
    </View>
);

export default ProfileScreen;
