import React, { Component } from 'react';
import {TextInput, Text, View, StyleSheet, Button, StatusBar} from 'react-native';

function App() {
    const [text, onChangeText] = React.useState("Useless Text");
    const [number, onChangeNumber] = React.useState(null);

    return (
        <View style={styles.container}>
            <Text style={styles.title}>Life 360</Text>
            <TextInput
                style={styles.textInput}
                //onChangeText={onChangeNumber} //calls when text is changed
                value={number}
                placeholder="Enter login"
                keyboardType="default"/>
            <TextInput
                style={styles.textInput}
                //onChangeText={onChangeNumber} //calls when text is changed
                value={number}
                placeholder="Enter Password"
                keyboardType="default"/>
            <Button style={styles.logIn}/>
            <StatusBar style={styles.statusBar} />
        </View>
    );
}