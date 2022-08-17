import React, {useState} from 'react';
import { Share, Text, View, StyleSheet, TouchableOpacity } from 'react-native';
import {share, getAddPerson, getJoinGroup, getCreateGroup} from './GroupAPICalls';
import colors from '../../constants/Colors';

const styles = StyleSheet.create({
    container:{
        flex: 1,
        alignItems: 'center',
    },changeGroupContainer:{
        top: '10%',
        width: '100%',
        flex: 0.3,
        flexDirection: "column",
    }, changeGroup:{
        flex: 1,
        backgroundColor: colors.primary,
        borderRadius: 30,
        margin: 10,
        justifyContent: 'center',
        alignItems: 'center',
    }, changeGroupText:{
        color: colors.white,
        fontSize: 14,
    },
})

const GroupScreen = () => {
    const [open, setOpen] = useState(false)

    return (
    <View style={styles.container}>
        <View style={styles.changeGroupContainer}>
            <TouchableOpacity
                style={styles.changeGroup}
                onPress={getJoinGroup}
            >
                <Text style={styles.changeGroupText}>Join A Group</Text>
            </TouchableOpacity>
            <TouchableOpacity
                style={styles.changeGroup}
                onPress={getAddPerson}
            >
                <Text style={styles.changeGroupText}>Add A Person</Text>
            </TouchableOpacity>
            <TouchableOpacity
                style={styles.changeGroup}
                onPress={getCreateGroup}
            >
                <Text style={styles.changeGroupText}>Create A Group</Text>
            </TouchableOpacity>
        </View>
    </View>
    )
};

export default GroupScreen;