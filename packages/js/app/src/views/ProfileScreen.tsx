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
    },
    header: {
        alignItems: 'center',
        justifyContent: 'center',
        flexDirection: 'column',
        padding: 10,
        height: 256,
    },
    profileImage: {
        height: 150,
        width: 150,
        borderRadius: 100,
        borderWidth: 2.5,
        borderColor: '#d3d3d3'
    },
    info: {
        padding: 20,
        backgroundColor: '#ffffff',
        borderTopRightRadius: 30,
        borderTopLeftRadius: 30,
        marginTop: 256
    },
    headerText: {
        fontSize: 36,
        color: '#ffffff',
    },
    heading: {
        fontSize: 24,
        marginBottom: 10
    }
});

const HeaderComponent = () => (
    <View style={styles.header}>
        <Image 
            source={require('../../assets/user.jpeg')}
            style={styles.profileImage}
        />
        <Text style={styles.headerText}>
            John Appleseed
        </Text>
        <Text style={{
            ...styles.headerText,
            fontSize: 14,
            marginTop: 3,
            color: '#e3e3e3'
        }}>
            johnappleseed@bubble.com
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
                height: 300,
                borderRadius: 15,
                marginBottom: 10
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
            marginBottom: 20
        }}
    >
        <Text 
            style={{
                fontSize: 20
            }}
        >{title}: {detail}</Text>
    </View>
);

const SlideUpComponent = () => (
    <View style={styles.info}>
        <Text style={styles.heading}>Map</Text>
        <MapComponent />
        <Text style={styles.heading}>Information</Text>
        <View style={{
            borderRadius: 15,
            backgroundColor: '#ffffff',
            paddingBottom: 24
        }}>
            <InfoComponent title='Email' detail='johnappleseed@bubble.com' />
            <InfoComponent title='Phone' detail='123-456-7890' />
            <InfoComponent title='Username' detail='johnappleseed' />
            <InfoComponent title='Last seen' detail='San Franscico, California' />
            <InfoComponent title='Last seen' detail='San Franscico, California' />

            <InfoComponent title='Last seen' detail='San Franscico, California' />

            <InfoComponent title='Last seen' detail='San Franscico, California' />


        </View>
    </View>
);

const ProfileScreen = () => (
    <ImageBackground
        source={require('../../assets/background.png')}
    >
        <SafeAreaView style={{flex: 1}} edges={['top', 'left', 'right']}>
            <HeaderComponent />
        </SafeAreaView>
        <ScrollView showsVerticalScrollIndicator={false}>
            <SlideUpComponent />
        </ScrollView>
    </ImageBackground>
   
);

export default ProfileScreen;
