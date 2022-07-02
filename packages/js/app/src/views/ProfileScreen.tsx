import React from 'react';
import { 
    View,
    ScrollView,
    Text, 
    Image, 
    StyleSheet,
    ImageBackground
} from 'react-native';

import MapView from 'react-native-maps';
import { SafeAreaView } from 'react-native-safe-area-context';

const styles = StyleSheet.create({
    shadow: {
        shadowColor: '#171717',
        shadowOffset: {width: -3, height: 10},
        shadowOpacity: 0.8,
        shadowRadius: 10,
    }
});

const HeaderComponent = () => (
    <ImageBackground
        source={require('../../assets/background.png')}
        style={{
            alignItems: 'center',
            justifyContent: 'center',
            flexDirection: 'column',
            padding: 10,
            height: 256,
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
            John Appleseed
        </Text>
    </ImageBackground>
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
                borderRadius: 10
            }}
        />
    </View>
);

interface InfoProps {
    title: string,
    detail: string
}

const InfoComponent: React.FunctionComponent<InfoProps> = ({ title='', detail='' }) => (
    <View
        style={{
            justifyContent: 'center',
            padding: 20,
            backgroundColor: '#ffffff',
            borderBottomColor: '#d3d3d3',
            borderBottomWidth: 1
        }}
    >
        <Text 
            style={{
                fontSize: 20
            }}
        >{title}: {detail}</Text>
    </View>
);

const ProfileScreen = () => (
    <SafeAreaView style={{flex: 1}} edges={['top', 'left', 'right']}>
        <HeaderComponent />
        <ScrollView>
            <MapComponent />
            <InfoComponent title='Email' detail='johnappleseed@bubble.com' />
            <InfoComponent title='Phone' detail='123-456-7890' />
            <InfoComponent title='Username' detail='johnappleseed' />
            <InfoComponent title='Last seen' detail='San Franscico, California' />
        </ScrollView>
    </SafeAreaView>
);

export default ProfileScreen;
