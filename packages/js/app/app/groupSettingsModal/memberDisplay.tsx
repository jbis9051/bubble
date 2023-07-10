import React, {useEffect, useState} from 'react';
import {useSelector} from 'react-redux';
import {useLocalSearchParams, useNavigation} from 'expo-router';
import {Alert, StyleSheet, View} from 'react-native';
import {GroupMemberDisplay} from '../../components/display/GroupMemberDisplay';
import {selectCurrentGroup} from '../../redux/slices/groupSlice';
import StyledButton from '../../components/bubbleUI/Button';
import {GroupService} from '../../lib/bubbleApi/group';
import {LoggingService} from '../../lib/bubbleApi/logging';

export default function MemberDisplay() {
    const curGroup = useSelector(selectCurrentGroup);
    const {user_uuid} = useLocalSearchParams();
    const navigation = useNavigation();

    const [kicking, setKicking] = useState(false);

    // useEffect(() => {
    //     navigation.setOptions({
    //         title: curMember?.name,
    //     });
    // }, []);

    const curMember = curGroup?.members.find((m) => m.user_uuid === user_uuid);

    const handleKick = () => {
        Alert.alert(
            `Kick '${curMember?.name}'?`,
            'They will need another invite to join back.', [
                {
                    text: 'OK',
                    style: 'destructive',
                    onPress: () => {
                        setKicking(true);
                        GroupService.remove_member(curGroup?.uuid!, user_uuid as string)
                            .then(() => {
                                navigation.goBack();
                            })
                            .catch((e) => {
                                LoggingService.error(e);
                                Alert.alert('Something went wrong.');
                                setKicking(false);
                            });
                    }
                },
                {
                    text: 'Cancel',
                    style: 'cancel'
                }
            ]);
    };

    if (!curMember) return null;

    return (
        <View style={styles.container}>
            <GroupMemberDisplay member={curMember}/>
            <View
                style={{
                    marginBottom: 30,
                    paddingHorizontal: 15,
                }}
            >
                <StyledButton
                    loading={kicking}
                    onPress={handleKick}
                    color="danger"
                >
                    Kick member
                </StyledButton>
            </View>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
    },
});
