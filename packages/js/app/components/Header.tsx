import React, { Component } from 'react';
import {View, StyleSheet,TouchableOpacity, Text} from 'react-native';
import colors from '../constants/Colors';
import {FontAwesomeIcon} from "@fortawesome/react-native-fontawesome";
import {faArrowLeftLong} from "@fortawesome/free-solid-svg-icons/faArrowLeftLong";
import { useNavigation } from '@react-navigation/native';

const styles = StyleSheet.create({
    header: {
        flexDirection: "row",
        width: '100%',
        height: 95,
        backgroundColor: colors.primary
    }
})

const Header: React.FC<{page: string, message?: string}> = ({page, message}) =>{
    const navigation = useNavigation();
    return (
        <View style={styles.header}>
            <TouchableOpacity
                onPress={() => {navigation.navigate(page)}}
                style={{alignItems: 'center', flex:0.2}}>
                <FontAwesomeIcon
                    style={{top: 45}}
                    icon={faArrowLeftLong}
                    size={35}
                />
            </TouchableOpacity>
            <Text style={{flex:0.5}}>{message}</Text>
        </View>
    );
}
export default Header;