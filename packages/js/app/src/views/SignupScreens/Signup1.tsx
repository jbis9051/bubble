import React from 'react';
import {
    Text,
    View,
    StyleSheet,
    TouchableOpacity,
    ImageBackground,
    Dimensions,
} from 'react-native';
import { NativeStackScreenProps } from '@react-navigation/native-stack';
import TextInputBox from '../../components/TextInputBox';
import scaleComponent from '../../components/scaleComponent';
import colors from '../../constants/Colors';
import Signup1Background from '../../assets/SignUp1Background.svg';

type RootStackParamList = {
    Login: undefined;
    Signup1: undefined;
    Signup2: undefined;
    Splash: undefined;
};

type Props = NativeStackScreenProps<RootStackParamList, 'Signup1'>;

const scaledSignupButtonWidth = scaleComponent(300, false);
const scaledTitleFont = scaleComponent(45, true);

const styles = StyleSheet.create({
    container: {
        alignItems: 'center',
        backgroundColor: colors.white,
        width: '100%',
        height: '100%',
    },
    backgroundImage: {
        alignItems: 'center',
    },
    titleContainer: {
        top: '12%',
        flex: 3,
        justifyContent: 'center',
    },
    signupContainer: {
        flex: 4.75,
        justifyContent: 'center',
    },
    signupButtonContainer: {
        flex: 2,
    },
    accountExistContainer: {
        flex: 1,
        alignItems: 'center',
        bottom: '2.6%',
    },
    title: {
        fontSize: scaledTitleFont,
        fontWeight: '400',
        color: colors.primary,
    },
    signupButton: {
        height: 50,
        width: scaledSignupButtonWidth,
        margin: 7,
        borderRadius: 25,
        padding: 10,
        alignItems: 'center',
        justifyContent: 'center',
        backgroundColor: colors.primary,
    },
    buttonText: {
        color: colors.white,
        fontWeight: '600',
    },
    noAccountText: {
        color: colors.primary,
        fontSize: 16,
        fontWeight: '300',
    },
    accountExistTextLink: {
        color: colors.primary,
        fontSize: 16,
        fontWeight: '300',
    },
});

const info = [
    {
        username: '',
        password: '',
        email: '',
        phone: '',
        name: '',
    },
];

function Signup({ route, navigation }: Props) {
    return (
        <View style={styles.container}>
            <Signup1Background
                height={'100%'}
                width={'100%'}
                style={{ position: 'absolute' }}
            />
            <View style={styles.titleContainer}>
                <Text style={styles.title}>Sign Up</Text>
            </View>
            <View style={styles.signupContainer}>
                <TextInputBox
                    descriptor="Phone Number"
                    params={'telephoneNumber'}
                    value={info}
                />
                <TextInputBox descriptor="Email" params={''} />
                <TextInputBox descriptor="Name" params={''} />
            </View>
            <View style={styles.signupButtonContainer}>
                <TouchableOpacity
                    style={styles.signupButton}
                    onPress={() => navigation.navigate('Signup2')}
                >
                    <Text style={styles.buttonText}>Next</Text>
                </TouchableOpacity>
            </View>
            <View style={styles.accountExistContainer}>
                <Text style={styles.noAccountText}>
                    Already have an account?
                </Text>
                <TouchableOpacity onPress={() => navigation.navigate('Login')}>
                    <Text style={styles.accountExistTextLink}>Sign In</Text>
                </TouchableOpacity>
            </View>
        </View>
    );
}
export default Signup;
