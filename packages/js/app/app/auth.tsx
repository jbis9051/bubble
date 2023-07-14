import React, {useEffect, useState} from 'react';
import {
    StyleSheet,
    SafeAreaView,
    Alert,
    ScrollView,
    KeyboardAvoidingView,
    TouchableOpacity, View,
} from 'react-native';
import Animated, {
    WithTimingConfig,
    runOnJS,
    set,
    useAnimatedStyle,
    useSharedValue,
    withTiming,
} from 'react-native-reanimated';
import {Entypo} from '@expo/vector-icons';
import StyledButton, {TextButton} from '../components/bubbleUI/Button';
import StyledText from '../components/StyledText';
import {StyledInput} from '../components/Input';
import SignUp1Svg from '../assets/svgs/SignUp1Background.svg';
import SignUp2Svg from '../assets/svgs/SignUp2Background.svg';
import FrontendInstanceStore from "../stores/FrontendInstanceStore";
import MainStore from "../stores/MainStore";

function SignUp({
                    switchToSignIn,
                }: {
    switchToSignIn: () => void;
}) {
    const [email, setEmail] = useState('');
    const [username, setUsername] = useState('');
    const [name, setName] = useState('');
    const [password, setPassword] = useState('');
    const [passwordConfirmation, setPasswordConfirmation] = useState('');

    const [curSlide, setCurSlide] = useState(0);
    const [displayedSlide, setDisplayedSlide] = useState(0);

    const [loading, setLoading] = useState(false);

    const submitSignUp = async () => {
        if (
            !email ||
            !username ||
            !name ||
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
        FrontendInstanceStore.instance.register(username, password, name, email)
            .catch((e) => {
                Alert.alert('Error', e);
            })
            .finally(() => {
                setLoading(false);
            })
    };

    const backArrowDeltaX = 100;
    const timingConfig: WithTimingConfig = {duration: 500};

    const bodyTranslateX = useSharedValue(0);
    const bodyOpacity = useSharedValue(1);
    const backArrowTranslateX = useSharedValue(-backArrowDeltaX);
    const backArrowOpacity = useSharedValue(0);

    const animatedBodyStyle = useAnimatedStyle(() => ({
        transform: [{translateX: bodyTranslateX.value}],
        opacity: bodyOpacity.value,
    }));

    const animatedBackArrowStyle = useAnimatedStyle(() => ({
        transform: [{translateX: backArrowTranslateX.value}],
        opacity: backArrowOpacity.value,
    }));

    useEffect(() => {
        if (curSlide === 0) {
            backArrowTranslateX.value = withTiming(
                -backArrowDeltaX,
                timingConfig
            );
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
                (bodyOpacity.value = withTiming(1)), timingConfig;
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

    const slideForward = () =>
        setCurSlide(Math.min(curSlide + 1, slides.length - 1));
    const slideBackward = () => setCurSlide(Math.max(curSlide - 1, 0));

    const slides: React.ReactNode[] = [
        <>
            <StyledInput
                viewStyle={styles.textInput}
                value={email}
                onChange={setEmail}
                label="Email"
                textContentType="username"
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
            <TextButton color="secondary" onPress={switchToSignIn} underlined>
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
                value={name}
                onChange={setName}
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
        </>,
    ];

    return (
        <>
            <SignUp1Svg
                height={'100%'}
                width={'100%'}
                style={{position: 'absolute'}}
            />
            <KeyboardAvoidingView
                style={{flex: 1, flexDirection: 'column'}}
                behavior="padding"
            >
                <ScrollView contentInsetAdjustmentBehavior="automatic">
                    <SafeAreaView style={styles.container}>
                        <View
                            style={{
                                display: 'flex',
                                flexDirection: 'row',
                                alignItems: 'center',
                                marginBottom: '10%',
                                width: '100%',
                                backgroundColor: 'transparent',
                            }}
                        >
                            <Animated.View
                                style={[
                                    {
                                        margin: 15,
                                        position: 'absolute',
                                        zIndex: 1,
                                    },
                                    animatedBackArrowStyle,
                                ]}
                            >
                                <TouchableOpacity onPress={slideBackward}>
                                    <Entypo
                                        name="chevron-left"
                                        size={24}
                                        color="black"
                                    />
                                </TouchableOpacity>
                            </Animated.View>
                            <StyledText
                                variant="h1"
                                nomargin
                                style={{textAlign: 'center', width: '100%'}}
                            >
                                Sign up
                            </StyledText>
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

function SignIn(
    {
        switchToSignUp,
        switchToForgotPassword,
    }: {
        switchToSignUp: () => void;
        switchToForgotPassword: () => void;
    }
) {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [loading, setLoading] = useState(false);

    const submit = async () => {
        setLoading(true);
        FrontendInstanceStore.instance.login(email, password)
            .then(async _uuid => {
                MainStore.groups = await FrontendInstanceStore.instance.get_groups();
                MainStore.status = await FrontendInstanceStore.instance.status();
            })
            .catch(err => {
                Alert.alert('Error', err.message);
            })
            .finally(() => {
                setLoading(false);
            })
    };

    return (
        <>
            <SignUp2Svg
                height={'100%'}
                width={'100%'}
                style={{position: 'absolute'}}
            />
            <KeyboardAvoidingView
                style={{
                    flex: 1,
                    flexDirection: 'column',
                    justifyContent: 'center',
                }}
                behavior="padding"
            >
                <ScrollView contentInsetAdjustmentBehavior="automatic">
                    <SafeAreaView style={styles.container}>
                        <View
                            style={{
                                marginBottom: '10%',
                                backgroundColor: 'transparent',
                            }}
                        >
                            <StyledText
                                variant="h1"
                                nomargin
                                style={{textAlign: 'center', width: '100%'}}
                            >
                                Welcome back
                            </StyledText>
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
                            onPress={switchToForgotPassword}
                            underlined
                        >
                            Forgot password
                        </TextButton>
                        <StyledText>Don't have an account yet?</StyledText>
                        <TextButton
                            color="secondary"
                            onPress={switchToSignUp}
                            underlined
                        >
                            Create an account
                        </TextButton>
                    </SafeAreaView>
                </ScrollView>
            </KeyboardAvoidingView>
        </>
    );
}

function ForgotPassword({
                            switchToSignIn,
                        }: {
    switchToSignIn: () => void;
}) {
    const [email, setEmail] = useState('');
    const [loading, setLoading] = useState(false);

    const submit = async () => {
        setLoading(true);
        FrontendInstanceStore.instance.forgot(email)
            .catch(err => {
                Alert.alert('Error', err);
            })
            .finally(() => {
                setLoading(false);
            });
    };

    return (
        <>
            <SignUp1Svg
                height={'100%'}
                width={'100%'}
                style={{position: 'absolute'}}
            />
            <KeyboardAvoidingView
                style={{
                    flex: 1,
                    flexDirection: 'column',
                    justifyContent: 'center',
                }}
                behavior="padding"
            >
                <ScrollView contentInsetAdjustmentBehavior="automatic">
                    <SafeAreaView style={styles.container}>
                        <StyledText
                            nomargin
                            variant="h1"
                            style={{textAlign: 'center', marginBottom: 15}}
                        >
                            Reset password
                        </StyledText>
                        <StyledText
                            nomargin
                            variant="body"
                            style={{textAlign: 'center'}}
                        >
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
                            onPress={switchToSignIn}
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

enum AuthPage {
    SIGN_IN,
    SIGN_UP,
    FORGOT_PASSWORD,
}


export default function Auth() {
    const [page, setPage] = useState(AuthPage.SIGN_IN);

    switch (page) {
        case AuthPage.SIGN_IN:
            return <SignIn switchToSignUp={() => setPage(AuthPage.SIGN_UP)}
                           switchToForgotPassword={() => setPage(AuthPage.FORGOT_PASSWORD)}/>;
        case AuthPage.SIGN_UP:
            return <SignUp switchToSignIn={() => setPage(AuthPage.SIGN_IN)}/>;
        case AuthPage.FORGOT_PASSWORD:
            return <ForgotPassword switchToSignIn={() => setPage(AuthPage.SIGN_IN)}/>;
    }
}

const styles = StyleSheet.create({
    container: {
        marginTop: '40%',
    },
    textInput: {
        marginVertical: 15,
        marginHorizontal: 15,
    },
});
