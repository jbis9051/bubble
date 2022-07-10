import React, {Component, useState,} from 'react';
import {View, StyleSheet,TextInput, Text} from 'react-native';
import {FontAwesomeIcon} from "@fortawesome/react-native-fontawesome";
import {faAsterisk} from "@fortawesome/free-solid-svg-icons/faAsterisk";
import colors from '../constants/Colors';

const styles = StyleSheet.create({
    container:{
        flexDirection: 'column',
        padding: 10,
    },descriptors:{
        flexDirection: 'row',
    },textInput:{
        borderLeftWidth: 0,
        borderRightWidth: 0,
        borderTopWidth: 0,
        height: 45,
        width: 300,
        borderWidth: 1,
        padding: 0,
    },
    textInputDescriptors:{
        flex: 1,
        justifyContent: 'center',
        alignItems: "flex-start",
        fontSize: 15,
        fontWeight: '300',
    },asterisk:{
        flex:1,
        justifyContent: 'flex-end',
    },
})


const TextInputBox: React.FC<{descriptor: string, secure: boolean, input: string}> =
    ({descriptor, secure, input}) => {
    const [isFocused, setFocus] = useState(false);
    const isPhone = input=="telephoneNumber"
    return (
        <View style={styles.container}>

            <View style={styles.descriptors}>
                <Text style={[styles.textInputDescriptors,
                    {color: isFocused ? colors.primary : colors.black}]}
                    //{color: colors.black}]}
                >{descriptor}</Text>
            </View>
            <TextInput
                style={[styles.textInput,
                    {borderBottomColor: isFocused ? colors.primary : colors.black}]} //I dont know how to do it without inline stylesheet
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