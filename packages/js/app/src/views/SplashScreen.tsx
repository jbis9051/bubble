import React from 'react';
import { Text, View, StyleSheet } from 'react-native';
import { NativeStackScreenProps } from '@react-navigation/native-stack';
import colors from '../constants/colors';
import SplashBackground from '../assets/SplashBackground.svg';
import Logo from '../assets/LogoNoBackground.svg';

type RootStackParamList = {
    Login: undefined;
    Signup1: undefined;
    Signup2: undefined;
    Splash: undefined;
};

type Props = NativeStackScreenProps<RootStackParamList, 'Splash'>;

const styles = StyleSheet.create({
    container: {
        alignItems: 'center',
        justifyContent: 'center',
        flexDirection: 'row',
        backgroundColor: colors.white,
        width: '100%',
        height: '100%',
    },
    backgroundImage: {
        height: '100%',
        width: '100%',
    },
    logoContainer: {
        alignItems: 'center',
        flex: 1,
    },
    titleContainer: {
        flex: 2,
        justifyContent: 'center',
    },
    title: {
        fontSize: 80,
        fontWeight: '300',
        color: colors.white,
    },
});

function Splash({ route, navigation }: Props) {
    setTimeout(() => {
        navigation.navigate('Login');
    }, 2000);
    return (
        <View style={styles.container}>
            <SplashBackground
                height={'100%'}
                width={'100%'}
                style={{ position: 'absolute' }}
            />
            <View style={styles.logoContainer}>
                <Logo height={100} width={100} />
            </View>
            <View style={styles.titleContainer}>
                <Text style={styles.title}>Bubble</Text>
            </View>
        </View>
    );
}
export default Splash;
