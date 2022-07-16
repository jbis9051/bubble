import React from 'react';
import { Text, View, StyleSheet, ImageBackground } from 'react-native';
import { NativeStackScreenProps } from '@react-navigation/native-stack';
import colors from '../constants/Colors';

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
        backgroundColor: colors.white,
        width: '100%',
        height: '100%',
    },
    backgroundImage: {
        height: '100%',
        width: '100%',
    },
    title: {
        fontSize: 80,
        fontWeight: '300',
        justifyContent: 'center',
    },
    titleContainer: {
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
    },
});

function Splash({ route, navigation }: Props) {
    setTimeout(() => {
        navigation.navigate('Login');
    }, 2000);
    return (
        <View style={styles.container}>
            <ImageBackground
                source={require('../assets/background.png')}
                style={styles.backgroundImage}
            >
                <View style={styles.titleContainer}>
                    <Text style={styles.title}>Bubble</Text>
                </View>
            </ImageBackground>
        </View>
    );
}
export default Splash;
