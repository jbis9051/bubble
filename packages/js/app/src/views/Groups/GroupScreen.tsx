import React, {useState} from 'react';
import { Image, Text, View, StyleSheet, TouchableOpacity } from 'react-native';
import {getAddPerson, getJoinGroup, getCreateGroup} from './GroupAPICalls';
import * as data from './groupsList.json';
import colors from '../../constants/Colors';

const styles = StyleSheet.create({
    container:{
        flex: 1,
        alignItems: 'center',
    },changeGroupContainer:{
        top: '10%',
        width: '100%',
        flex: 1.2,
        flexDirection: "column",
    }, groupListContainer:{
        width: '80%',
        top: '10%',
        flex: 3,
        alignItems: 'center',
    }, groupContainer:{
        alignItems: 'center',
        width: '100%',
        flexDirection: 'row',
        margin: 5,
    }, groupsDescriptorContainer:{
        paddingBottom: 5,
        width: '100%',
        borderBottomWidth: 2,
        borderBottomColor: colors.darkGrey,
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
    }, groupSelected:{
        backgroundColor: colors.secondary,
    },
    groupText:{
        fontSize: 20,
        fontWeight: '200',
    }, groupDescriptorText:{
        fontSize: 16,
        fontWeight: '200',
    }, groupProfilePicture:{
        marginTop: 5,
        marginRight: 5,
        width: 50,
        height: 50,
        borderRadius: 25,
    }
})

const groupList = data;



const GroupScreen = () => {
    const [selected, setSelected] = useState(0);

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
        <View style={styles.groupListContainer}>
            <View style={styles.groupsDescriptorContainer}>
                <Text style={styles.groupDescriptorText}>Groups</Text>
            </View>
            {
                groupList && groupList.groups.map(group => {
                return (
                    <View
                        key={JSON.stringify(group)}
                        style={[styles.groupContainer, selected===groupList.groups.findIndex(obj => obj.name===group.name) && styles.groupSelected]}
                    >
                        <Image
                            style={styles.groupProfilePicture}
                            source={require('./tempGroupProfile.jpg')}
                        />
                        <Text style={styles.groupText}>{group.name}</Text>
                    </View>
                )
            })}
        </View>
    </View>
    )
};

export default GroupScreen;