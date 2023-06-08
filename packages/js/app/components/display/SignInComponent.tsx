import React, { useEffect, useState } from 'react';
import { StyleSheet, SafeAreaView, Alert, ScrollView } from 'react-native';
import { UserLocal, UserService } from '../../lib/bubbleApi/user';
import { LoggingService } from '../../lib/bubbleApi/logging';
import StyledButton, { TextButton } from '../bubbleUI/Button';
import StyledText from '../StyledText';
import { StyledInput } from '../Input';

interface SignInScreenProps {
    setUser: (user: UserLocal | null) => void;
}
export default function SignInScreen({ setUser }: SignInScreenProps) {
    const [email, setEmail] = useState('');
    const [username, setUsername] = useState('');
    const [displayName, setDisplayName] = useState('');
    const [password, setPassword] = useState('');
    const [passwordConfirmation, setPasswordConfirmation] = useState('');

    const [forgotPassword, setForgotPassword] = useState(false);
    const [signingUp, setSigningUp] = useState(true);

    useEffect(() => {
        // reset all fields when signingUp is changed
        setEmail('');
        setUsername('');
        setDisplayName('');
        setPassword('');
        setPasswordConfirmation('');
    }, [signingUp]);

    const refreshUser = () => {
        UserService.retrieveSession()
            .then((s) => {
                setUser(s);
            })
            .catch(LoggingService.error);
    };

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
        UserService.register(username, password, displayName)
            .then(refreshUser)
            .catch(LoggingService.error);
    };

    const submitSignIn = () => {
        if (!email || !password) {
            Alert.alert('Please fill out all fields');
            return;
        }
        UserService.login(username, password)
            .then(refreshUser)
            .catch(LoggingService.error);
    };

    const submitForgotPassword = () => {
        if (!email) {
            Alert.alert('Please fill out all fields');
            return;
        }
        UserService.forgot(email)
            .then(() =>
                Alert.alert('Check your email for a password reset link')
            )
            .catch(LoggingService.error);
    };

    const toggleSignUp = () => setSigningUp(!signingUp);

    if (forgotPassword) {
        return (
            <ScrollView contentInsetAdjustmentBehavior="automatic">
                <SafeAreaView style={styles.container}>
                    <StyledText variant="h1">Reset password</StyledText>
                    <StyledText variant="body">
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
                        onPress={submitForgotPassword}
                        style={{
                            marginHorizontal: 15,
                        }}
                    >
                        Submit
                    </StyledButton>
                    <TextButton
                        color="primary"
                        onPress={() => setForgotPassword(false)}
                    >
                        Back
                    </TextButton>
                </SafeAreaView>
            </ScrollView>
        );
    }

    if (signingUp) {
        return (
            <ScrollView contentInsetAdjustmentBehavior="automatic">
                <SafeAreaView style={styles.container}>
                    <StyledText variant="h1">Bubble</StyledText>
                    <StyledText variant="body">
                        Your location sharing service.
                    </StyledText>
                    <StyledText variant="body">
                        Completely open-source and end-to-end encrypted.
                    </StyledText>
                    <StyledInput
                        viewStyle={styles.textInput}
                        value={email}
                        onChange={setEmail}
                        label="Email"
                    />
                    <StyledInput
                        viewStyle={styles.textInput}
                        value={username}
                        onChange={setUsername}
                        label="Username"
                    />
                    <StyledInput
                        viewStyle={styles.textInput}
                        value={email}
                        onChange={setEmail}
                        label="Display Name"
                    />
                    <StyledInput
                        viewStyle={styles.textInput}
                        value={password}
                        onChange={setPassword}
                        label="Password"
                        secureTextEntry={true}
                    />
                    <StyledInput
                        viewStyle={styles.textInput}
                        value={passwordConfirmation}
                        onChange={setPasswordConfirmation}
                        label="Confirm Password"
                        secureTextEntry={true}
                    />
                    <StyledButton
                        color="primary"
                        onPress={submitSignUp}
                        style={{
                            marginHorizontal: 15,
                        }}
                    >
                        Sign Up
                    </StyledButton>
                    <StyledText>Already have an account?</StyledText>
                    <TextButton color="primary" onPress={toggleSignUp}>
                        Sign in instead
                    </TextButton>
                </SafeAreaView>
            </ScrollView>
        );
    }

    return (
        <ScrollView contentInsetAdjustmentBehavior="automatic">
            <SafeAreaView style={styles.container}>
                <StyledText variant="h1">Welcome back.</StyledText>
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
                    onPress={submitSignIn}
                    style={{
                        marginHorizontal: 15,
                    }}
                >
                    Sign In
                </StyledButton>
                <TextButton
                    color="primary"
                    onPress={() => setForgotPassword(true)}
                >
                    Forgot password
                </TextButton>
                <StyledText>Don't have an account yet?</StyledText>
                <TextButton color="primary" onPress={toggleSignUp}>
                    Create an account
                </TextButton>
            </SafeAreaView>
        </ScrollView>
    );
}

const styles = StyleSheet.create({
    container: {},
    textInput: {
        marginVertical: 15,
        marginHorizontal: 15,
    },
});
