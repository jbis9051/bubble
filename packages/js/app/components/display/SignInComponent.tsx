import React, { useEffect, useState } from 'react';
import { StyleSheet, SafeAreaView, Alert, ScrollView, KeyboardAvoidingView, TouchableOpacity } from 'react-native';
import { UserLocal, UserService } from '../../lib/bubbleApi/user';
import { LoggingService } from '../../lib/bubbleApi/logging';
import StyledButton, { TextButton } from '../bubbleUI/Button';
import StyledText from '../StyledText';
import { StyledInput } from '../Input';
import SignUp1Svg from '../../assets/svgs/SignUp1Background.svg';
import SignUp2Svg from '../../assets/svgs/SignUp2Background.svg';
import Animated, { WithTimingConfig, runOnJS, set, useAnimatedStyle, useSharedValue, withTiming } from 'react-native-reanimated';
import { View } from '../Themed';
import { Entypo } from '@expo/vector-icons';
import { useDispatch } from 'react-redux';
import { setAuth } from '../../redux/slices/authSlice';

function SignUpFlow({
    refreshUser,
    toggleSignUp,
}: {
    refreshUser: () => void;
    toggleSignUp: () => void;
}) {
    const [email, setEmail] = useState('');
    const [username, setUsername] = useState('');
    const [displayName, setDisplayName] = useState('');
    const [password, setPassword] = useState('');
    const [passwordConfirmation, setPasswordConfirmation] = useState('');

    const [curSlide, setCurSlide] = useState(0);
    const [displayedSlide, setDisplayedSlide] = useState(0);

    const [loading, setLoading] = useState(false);

    const submitSignUp = async () => {
        if (
            !email ||
            !username ||
            !displayName ||
            !password ||
            !passwordConfirmation
        ) {
            Alert.alert('Please fill out all fields');
            return;
        }
        if (password !== passwordConfirmation) {
            Alert.alert('Passwords do not match');
            return;
        }
        setLoading(true);
        await UserService.register(username, password, displayName)
            .then(refreshUser)
            .catch(LoggingService.error);
        setLoading(false);
    };

    const backArrowDeltaX = 100;
    const timingConfig: WithTimingConfig = { duration: 500 };

    const bodyTranslateX = useSharedValue(0);
    const bodyOpacity = useSharedValue(1);
    const backArrowTranslateX = useSharedValue(-backArrowDeltaX);
    const backArrowOpacity = useSharedValue(0);

    const animatedBodyStyle = useAnimatedStyle(() => ({
        transform: [{ translateX: bodyTranslateX.value }],
        opacity: bodyOpacity.value,
    }));

    const animatedBackArrowStyle = useAnimatedStyle(() => ({
        transform: [{ translateX: backArrowTranslateX.value }],
        opacity: backArrowOpacity.value,
    }));

    useEffect(() => {
        if (curSlide === 0) {
            backArrowTranslateX.value = withTiming(-backArrowDeltaX, timingConfig);
            backArrowOpacity.value = withTiming(0, timingConfig);
        } else {
            backArrowTranslateX.value = withTiming(0, timingConfig);
            backArrowOpacity.value = withTiming(1, timingConfig);
        }

        const bodyDeltaX = 50;

        if (curSlide > displayedSlide) {
            bodyTranslateX.value = withTiming(-bodyDeltaX, timingConfig);
            bodyOpacity.value = withTiming(0, {}, () => {
                runOnJS(setDisplayedSlide)(curSlide);
                bodyTranslateX.value = bodyDeltaX;
                bodyTranslateX.value = withTiming(0, timingConfig);
                bodyOpacity.value = withTiming(1), timingConfig;
            });
        } else if (curSlide < displayedSlide) {
            bodyTranslateX.value = withTiming(bodyDeltaX, timingConfig);
            bodyOpacity.value = withTiming(0, {}, () => {
                runOnJS(setDisplayedSlide)(curSlide);
                bodyTranslateX.value = -bodyDeltaX;
                bodyTranslateX.value = withTiming(0, timingConfig);
                bodyOpacity.value = withTiming(1, timingConfig);
            });
        }
    }, [curSlide]);

    const slideForward = () => setCurSlide(Math.min(curSlide + 1, slides.length - 1));
    const slideBackward = () => setCurSlide(Math.max(curSlide - 1, 0));

    const slides: React.ReactNode[] = [
        <>
            <StyledInput
                viewStyle={styles.textInput}
                value={email}
                onChange={setEmail}
                label="Email"
                textContentType='username'
            />
            <StyledButton
                color="primary"
                onPress={slideForward}
                style={{
                    marginHorizontal: 15,
                }}
                disabled={email === ''}
            >
                Continue
            </StyledButton>
            <StyledText>Already have an account?</StyledText>
            <TextButton color="secondary" onPress={toggleSignUp} underlined>
                Sign in instead
            </TextButton>
        </>,
        <>
            <StyledInput
                viewStyle={styles.textInput}
                value={username}
                onChange={setUsername}
                label="Username"
            />
            <StyledInput
                viewStyle={styles.textInput}
                value={displayName}
                onChange={setDisplayName}
                label="Display Name"
            />
            <StyledButton
                color="primary"
                onPress={slideForward}
                style={{
                    marginHorizontal: 15,
                }}
                disabled={email === ''}
            >
                Continue
            </StyledButton>
        </>,
        <>
            <StyledInput
                viewStyle={styles.textInput}
                value={password}
                onChange={setPassword}
                label="Password"
                secureTextEntry
            />
            <StyledInput
                viewStyle={styles.textInput}
                value={passwordConfirmation}
                onChange={setPasswordConfirmation}
                label="Confirm Password"
                secureTextEntry
            />
            <StyledButton
                color="primary"
                onPress={submitSignUp}
                style={{
                    marginHorizontal: 15,
                }}
                disabled={password === '' || passwordConfirmation === ''}
                loading={loading}
            >
                Finish Sign Up
            </StyledButton>
        </>
    ];

    return (
        <>
            <SignUp1Svg
                height={'100%'}
                width={'100%'}
                style={{ position: 'absolute' }}
            />
            <KeyboardAvoidingView style={{ flex: 1, flexDirection: 'column', }} behavior="padding">
                <ScrollView contentInsetAdjustmentBehavior="automatic">
                    <SafeAreaView style={styles.container}>
                        <View style={{
                            display: "flex",
                            flexDirection: "row",
                            alignItems: "center",
                            marginBottom: "10%",
                            width: "100%",
                            backgroundColor: 'transparent',
                        }}>
                            <Animated.View
                                style={[{
                                    margin: 15,
                                    position: 'absolute',
                                    zIndex: 1,
                                }, animatedBackArrowStyle]}
                            >
                                <TouchableOpacity onPress={slideBackward}>
                                    <Entypo name="chevron-left" size={24} color="black" />
                                </TouchableOpacity>
                            </Animated.View>
                            <StyledText variant="h1" nomargin style={{ textAlign: "center", width: "100%", }}>Sign up</StyledText>
                        </View>
                        <Animated.View style={animatedBodyStyle}>
                            {slides[displayedSlide]}
                        </Animated.View>
                    </SafeAreaView>
                </ScrollView>
            </KeyboardAvoidingView>
        </>
    );
}

function SignInFlow({
    toggleSignUp,
    toggleForgotPassword,
    submitSignIn,
}: {
    toggleSignUp: () => void;
    toggleForgotPassword: () => void;
    submitSignIn: (email: string, password: string) => Promise<void> | undefined;
}) {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [loading, setLoading] = useState(false);

    const submit = async () => {
        setLoading(true);
        await submitSignIn(email, password);
        setLoading(false);
    }

    return (
        <>
            <SignUp2Svg
                height={'100%'}
                width={'100%'}
                style={{ position: 'absolute' }}
            />
            <KeyboardAvoidingView style={{ flex: 1, flexDirection: 'column', justifyContent: 'center', }} behavior="padding">
                <ScrollView contentInsetAdjustmentBehavior="automatic">
                    <SafeAreaView style={styles.container}>
                        <View style={{
                            marginBottom: "10%",
                            backgroundColor: 'transparent',
                        }}>
                            <StyledText variant="h1" nomargin style={{ textAlign: "center", width: "100%", }}>Welcome back</StyledText>
                        </View>
                        <StyledInput
                            viewStyle={styles.textInput}
                            value={email}
                            onChange={setEmail}
                            label="Email"
                        />
                        <StyledInput
                            viewStyle={styles.textInput}
                            value={password}
                            onChange={setPassword}
                            label="Password"
                            secureTextEntry={true}
                        />
                        <StyledButton
                            color="primary"
                            onPress={submit}
                            style={{
                                marginHorizontal: 15,
                            }}
                            loading={loading}
                        >
                            Sign In
                        </StyledButton>
                        <TextButton
                            color="secondary"
                            onPress={toggleForgotPassword}
                            underlined
                        >
                            Forgot password
                        </TextButton>
                        <StyledText>Don't have an account yet?</StyledText>
                        <TextButton color="secondary" onPress={toggleSignUp} underlined>
                            Create an account
                        </TextButton>
                    </SafeAreaView>
                </ScrollView>
            </KeyboardAvoidingView>
        </>
    )
}

function ResetPasswordFlow({
    toggleForgotPassword,
    submitForgotPassword,
}: {
    toggleForgotPassword: () => void;
    submitForgotPassword: (email: string) => Promise<void> | undefined;
}) {
    const [email, setEmail] = useState('');
    const [loading, setLoading] = useState(false);

    const submit = async () => {
        setLoading(true);
        await submitForgotPassword(email);
        setLoading(false);
    }

    return (
        <>
            <SignUp1Svg
                height={'100%'}
                width={'100%'}
                style={{ position: 'absolute' }}
            />
            <KeyboardAvoidingView style={{ flex: 1, flexDirection: 'column', justifyContent: 'center', }} behavior="padding">
                <ScrollView contentInsetAdjustmentBehavior="automatic">
                    <SafeAreaView style={styles.container}>
                        <StyledText nomargin variant="h1" style={{ textAlign: "center", marginBottom: 15 }}>Reset password</StyledText>
                        <StyledText nomargin variant="body" style={{ textAlign: "center", }}>
                            Enter the email that you used to sign up.
                        </StyledText>
                        <StyledInput
                            viewStyle={styles.textInput}
                            value={email}
                            onChange={setEmail}
                            label="Email"
                        />
                        <StyledButton
                            color="primary"
                            onPress={submit}
                            style={{
                                marginHorizontal: 15,
                            }}
                            loading={loading}
                        >
                            Submit
                        </StyledButton>
                        <TextButton
                            color="secondary"
                            onPress={toggleForgotPassword}
                            underlined
                        >
                            Back
                        </TextButton>
                    </SafeAreaView>
                </ScrollView>
            </KeyboardAvoidingView>
        </>
    );
}

export default function SignInScreen() {
    const dispatch = useDispatch();

    const [forgotPassword, setForgotPassword] = useState(false);
    const [signingUp, setSigningUp] = useState(true);

    const refreshUser = () => {
        UserService.retrieveSession()
            .then((s) => {
                dispatch(setAuth(s))
            })
            .catch(LoggingService.error);
    };

    const submitSignIn = (email: string, password: string) => {
        if (!email || !password) {
            Alert.alert('Please fill out all fields');
            return;
        }
        return UserService.login(email, password)
            .then(refreshUser)
            .catch(LoggingService.error);
    };

    const submitForgotPassword = (email: string) => {
        if (!email) {
            Alert.alert('Please fill out all fields');
            return;
        }
        return UserService.forgot(email)
            .then(() =>
                Alert.alert('Check your email for a password reset link')
            )
            .catch(LoggingService.error);
    };

    const toggleSignUp = () => setSigningUp(!signingUp);
    const toggleForgotPassword = () => setForgotPassword(!forgotPassword);

    if (forgotPassword) {
        return <ResetPasswordFlow
            submitForgotPassword={submitForgotPassword}
            toggleForgotPassword={toggleForgotPassword}
        />
    }

    if (signingUp) {
        return (
            <SignUpFlow
                refreshUser={refreshUser}
                toggleSignUp={toggleSignUp}
            />
        );
    }

    return <SignInFlow
        toggleSignUp={toggleSignUp}
        toggleForgotPassword={toggleForgotPassword}
        submitSignIn={submitSignIn}
    />
}

const styles = StyleSheet.create({
    container: {
        marginTop: "40%",
    },
    textInput: {
        marginVertical: 15,
        marginHorizontal: 15,
    },
});
