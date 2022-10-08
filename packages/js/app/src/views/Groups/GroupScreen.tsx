import React, { useState } from 'react';
import {
    Image,
    Text,
    View,
    TouchableOpacity,
    ScrollView,
    TextInput,
} from 'react-native';
import { FontAwesomeIcon } from '@fortawesome/react-native-fontawesome';
import { faTrash } from '@fortawesome/free-solid-svg-icons/faTrash';
import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';
import { faPlus } from '@fortawesome/free-solid-svg-icons/faPlus';
import { getAddPerson, getJoinGroup, getCreateGroup } from './GroupAPICalls';
import * as data from './groupsList.json';
import colors from '../../constants/Colors';
import styles from './styles';

const groupList = data;

const GroupScreen = () => {
    const [selected, setSelected] = useState(0);
    return (
        <View style={styles.container}>
            {/* <View style={styles.changeGroupContainer}>
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
            </View> */}
            <View style={styles.groupListContainer}>
                <View style={styles.groupsDescriptorContainer}>
                    <Text style={styles.groupDescriptorText}>
                        Manage Groups
                    </Text>
                    <TouchableOpacity onPress={getJoinGroup}>
                        <FontAwesomeIcon
                            icon={faPlus}
                            style={styles.searchIcon}
                        />
                    </TouchableOpacity>
                </View>
                <View style={styles.searchBar}>
                    <FontAwesomeIcon
                        icon={faMagnifyingGlass}
                        style={styles.searchIcon}
                    />
                    <TextInput
                        style={{ flex: 1 }}
                        placeholder="Search groups"
                    />
                </View>
                <ScrollView
                    bounces={true}
                    style={{ maxHeight: '82%', maxWidth: '100%' }}
                >
                    <View style={styles.groupContainer}>
                        <TouchableOpacity
                            style={styles.groupContainerTouchableNamePFP}
                            onPress={() => {
                                getCreateGroup();
                            }}
                        >
                            <Image
                                style={styles.groupProfilePicture}
                                source={require('./tempGroupProfile.jpg')} // eslint-disable-line global-require
                            />
                            <Text style={styles.groupText}>Create Group</Text>
                        </TouchableOpacity>
                    </View>
                    {groupList &&
                        groupList.groups.map((group) => {
                            const index = groupList.groups.findIndex(
                                (obj) => obj.name === group.name
                            );
                            return (
                                <View
                                    key={JSON.stringify(group)}
                                    style={[
                                        styles.groupContainer,
                                        selected === index &&
                                            styles.groupSelected,
                                    ]}
                                >
                                    <TouchableOpacity
                                        style={
                                            styles.groupContainerTouchableNamePFP
                                        }
                                        onPress={() => {
                                            setSelected(index);
                                        }}
                                    >
                                        <Image
                                            style={styles.groupProfilePicture}
                                            source={require('./tempGroupProfile.jpg')} // eslint-disable-line global-require
                                        />
                                        <Text style={styles.groupText}>
                                            {group.name}
                                        </Text>
                                    </TouchableOpacity>
                                    <TouchableOpacity
                                        style={{
                                            height: '20%',
                                            width: '10%',
                                            justifyContent: 'center',
                                        }}
                                    >
                                        <FontAwesomeIcon
                                            icon={faTrash}
                                            style={{ color: colors.darkGrey }}
                                        />
                                    </TouchableOpacity>
                                </View>
                            );
                        })}
                </ScrollView>
            </View>
        </View>
    );
};

export default GroupScreen;
