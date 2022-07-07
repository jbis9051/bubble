import React from 'react';
import { 
    View,
    ScrollView,
    Text, 
    Image, 
    StyleSheet,
    Pressable,
    ImageBackground
} from 'react-native';

import { 
    faEnvelope as mailIcon,
    faPhone as phoneIcon,
    faLocationDot as locationIcon,
    faUser as userIcon,
    faGear as gearIcon
} from '@fortawesome/free-solid-svg-icons';
import { IconProp } from '@fortawesome/fontawesome-svg-core';

import colors from '../constants/colors';

import MapView from 'react-native-maps';
import { SafeAreaView } from 'react-native-safe-area-context';
import { FontAwesomeIcon } from '@fortawesome/react-native-fontawesome';

const styles = StyleSheet.create({
    shadow: {
        shadowColor: '#171717',
        shadowOffset: {width: -3, height: 10},
        shadowOpacity: 0.1,
        shadowRadius: 10,
    },
    navigation: {
        height: 40,
        width: '100%',
        justifyContent: 'flex-end',
        flexDirection: 'row',
        paddingRight: 20
    },
    editButton: {
        color: '#ffffff',
        fontSize: 24,
        fontWeight: '600'
    },
    header: {
        alignItems: 'center',
        justifyContent: 'center',
        flexDirection: 'column',
        padding: 10,
        height: 206
    },
    profileImage: {
        height: 140,
        width: 140,
        borderRadius: 100,
        borderWidth: 2.5,
        borderColor: '#d3d3d3'
    },
    info: {
        padding: 20,
        paddingTop: 10,
        paddingBottom: 30,
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
        fontSize: 30,
        fontWeight: '800',
        marginBottom: 10
    }
});

const NavigationComponent = () => (
    <View style={styles.navigation}>
        <Pressable>
            <Text style={styles.editButton}>Edit</Text>
        </Pressable>
    </View>
);

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
            fontSize: 16,
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
                longitudeDelta: 0.0421
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
    detail: string,
    icon: IconProp
}

const InfoComponent: React.FunctionComponent<InfoProps> = ({ title='', detail='', icon }) => (
    <View
        style={{
            marginBottom: 10,
            padding: 20,
            backgroundColor: '#ffffff',
            borderRadius: 10,
            ...styles.shadow
        }}
    >
        {
            icon ? (
                <View style={{flexDirection: 'row'}}>
                    <FontAwesomeIcon
                        icon={icon}
                        color={colors.primary}
                    />
                    <Text style={{marginLeft: 5}}>
                        {title}
                    </Text>
                </View>
            ) : (
                <View>
                    <Text>{title}</Text>
                </View>
            )
        }
        <Text 
            style={{
                fontSize: 20,
                fontWeight: '500',
                marginTop: 10
            }}
        >{detail}</Text>
    </View>
);

const SlideUpComponent = () => (
    <View style={styles.info}>
        <View style={{width: '100%', alignItems: 'center', justifyContent: 'center'}}>
            <View style={{width: 64, height: 8, backgroundColor: '#d3d3d3', borderRadius: 100}} ></View>
        </View>
        <Text style={styles.heading}>Map</Text>
        <MapComponent />
        <Text style={styles.heading}>Information</Text>
        <View style={{
            borderRadius: 15,
            backgroundColor: '#ffffff',
            paddingBottom: 24
        }}>
            <InfoComponent title='Email' detail='johnappleseed@bubble.com' icon={mailIcon} />
            <InfoComponent title='Phone' detail='123-456-7890' icon={phoneIcon} />
            <InfoComponent title='Username' detail='johnappleseed' icon={userIcon} />
            <InfoComponent title='Last seen' detail='San Franscico, California' icon={locationIcon} />
        </View>
    </View>
);

const ProfileScreen = () => (
    <ImageBackground
        source={require('../../assets/background.png')}
    >
        <SafeAreaView style={{flex: 1}} edges={['top', 'left', 'right']}>
            <NavigationComponent />
            <HeaderComponent />
        </SafeAreaView>
        <ScrollView style={{flex: 0}} showsVerticalScrollIndicator={false}>
            <SlideUpComponent />
        </ScrollView>
    </ImageBackground>
);

export default ProfileScreen;
