import React, { Component } from 'react';
import {View, StyleSheet,TextInput, Text} from 'react-native';
import {FontAwesomeIcon} from "@fortawesome/react-native-fontawesome";
import {faAsterisk} from "@fortawesome/free-solid-svg-icons/faAsterisk";
import colors from '../constants/Colors';

const styles = StyleSheet.create({
    container:{
        flexDirection: 'column',
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
    },asterisk:{
        flex:1,
        justifyContent: 'flex-end',
    },
})



const TextInputBox: React.FC<{descriptor: string, required: boolean}> = ({descriptor, required}) => {
    return (
        <View style={styles.container}>
            <View style={styles.descriptors}>
                <Text style={styles.textInputDescriptors}>{descriptor}</Text>
                {required &&
                <FontAwesomeIcon
                    style={styles.asterisk}
                    icon={faAsterisk}
                    color={colors.primary}
                    size={15}
                />}
            </View>

            <TextInput
                style={styles.textInput}
                underlineColorAndroid='transparent'
                keyboardType="default"/>
        </View>
    )
}
export default TextInputBox;