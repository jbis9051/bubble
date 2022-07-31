import React, { useState } from 'react';
import { View, StyleSheet, TextInput, Text } from 'react-native';
import colors from '../constants/Colors';

const styles = StyleSheet.create({
    container: {
        padding: '2.5%',
        alignItems: 'center',
    },
    descriptors: {
        flexDirection: 'row',
    },
    textInput: {
        height: 45,
        borderLeftWidth: 0,
        borderRightWidth: 0,
        borderTopWidth: 0,
        borderBottomWidth: 1,
        width: '100%',
        padding: 0,
    },
    textInputDescriptors: {
        flex: 1,
        justifyContent: 'center',
        alignItems: 'flex-start',
        fontSize: 15,
        fontWeight: '300',
    },
    asterisk: {
        flex: 1,
        justifyContent: 'flex-end',
    },
});

const phoneText = (inputNum: string) => {
    const phoneNumber = inputNum.replace(/[^\d]/g, '');
    const len = phoneNumber.length;
    if (len > 6) {
        return `(${phoneNumber.slice(0, 3)}) ${phoneNumber.slice(
            3,
            6
        )}-${phoneNumber.slice(6, 10)}`;
    }
    if (len <= 3) {
        return phoneNumber;
    }
    if (len <= 6) {
        return `(${phoneNumber.slice(0, 3)}) ${phoneNumber.slice(3, 6)}`;
    }
    return null;
};

const TextInputBox: React.FC<{ descriptor: string; params: string }> = ({
    descriptor,
    params,
}) => {
    const [isFocused, setFocus] = useState(false);
    const [input, setInput] = useState('');
    const isPhone = params.includes('phoneNumber');
    const isSecure = params.includes('password');
    return (
        <View style={styles.container}>
            <View style={styles.descriptors}>
                <Text
                    style={[
                        styles.textInputDescriptors,
                        { color: isFocused ? colors.primary : colors.black },
                    ]}
                >
                    {descriptor}
                </Text>
            </View>
            {isPhone && (
                <TextInput
                    style={[
                        styles.textInput,
                        {
                            borderBottomColor: isFocused
                                ? colors.primary
                                : colors.black,
                        },
                    ]}
                    onFocus={() => setFocus(true)}
                    onBlur={() => setFocus(false)}
                    secureTextEntry={isSecure}
                    textContentType={isPhone ? 'telephoneNumber' : undefined}
                    keyboardType="default"
                    value={input}
                    onChangeText={(e) => {
                        setInput(phoneText(e));
                    }}
                />
            )}
            {!isPhone && (
                <TextInput
                    style={[
                        styles.textInput,
                        {
                            borderBottomColor: isFocused
                                ? colors.primary
                                : colors.black,
                        },
                    ]}
                    onFocus={() => setFocus(true)}
                    onBlur={() => setFocus(false)}
                    secureTextEntry={isSecure}
                    textContentType={isPhone ? 'telephoneNumber' : undefined}
                    keyboardType="default"
                />
            )}
        </View>
    );
};
export default TextInputBox;
