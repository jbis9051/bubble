import React, { useState } from 'react';
import {View, StyleSheet,TextInput, Text} from 'react-native';
import colors from '../constants/colors';

const styles = StyleSheet.create({
    container:{
        padding: 10,
    },
    textInput: {
        borderLeftWidth: 0,
        borderRightWidth: 0,
        borderTopWidth: 0,
        height: 50,
        width: 300,
        borderWidth: 1,
        padding: 10,
    },descriptors:{
        flexDirection: 'row',
    },
    textInputDescriptors:{
        flex: 1,
        justifyContent: 'center',
        alignItems: "flex-start",
        color: colors.black,
        fontWeight: '200',
    },
})



const TextInputBox: React.FC<{descriptor: string, secure: boolean, input: string}> =
    ({descriptor, secure, input}) => {
    const [isFocused, setFocus] = useState(false);
    const isPhone = input==="telephoneNumber"
    return (
        <View style={styles.container}>
            <View style={styles.descriptors}>
                <Text style={styles.textInputDescriptors}>{descriptor}</Text>
                <Text style={[styles.textInputDescriptors,
                    {color: isFocused ? colors.primary : colors.black}]}
                    // {color: colors.black}]}
                >{descriptor}</Text>
            </View>

            <TextInput
                style={styles.textInput}
                underlineColorAndroid='transparent'
                keyboardType="default"/>
                style={[styles.textInput,
                    {borderBottomColor: isFocused ? colors.primary : colors.black}]}
                onFocus = {() => setFocus(true)}
                onBlur = {() => setFocus(false)}
                secureTextEntry={secure}
                textContentType={isPhone ? 'telephoneNumber' : undefined}
                keyboardType="default"
            />
        </View>
    )
}
export default TextInputBox;