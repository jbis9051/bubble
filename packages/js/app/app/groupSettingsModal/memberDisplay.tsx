import React, { useEffect, useState } from 'react';
import { useSelector } from 'react-redux';
import { useLocalSearchParams, useNavigation } from 'expo-router';
import { StyleSheet } from 'react-native';
import { GroupMemberDisplay } from '../../components/display/GroupMemberDisplay';
import { selectCurrentGroup } from '../../redux/slices/groupSlice';
import { View } from '../../components/Themed';
import StyledButton from '../../components/bubbleUI/Button';
import { GroupService } from '../../lib/bubbleApi/group';
import {
    alertPrompt,
    confirmPromptDestructive,
} from '../../components/PromptProvider';
import { LoggingService } from '../../lib/bubbleApi/logging';

export default function MemberDisplay() {
    const curGroup = useSelector(selectCurrentGroup);
    const { user_uuid } = useLocalSearchParams();
    const navigation = useNavigation();

    const [kicking, setKicking] = useState(false);

    // useEffect(() => {
    //     navigation.setOptions({
    //         title: curMember?.name,
    //     });
    // }, []);

    const curMember = curGroup?.members.find((m) => m.user_uuid === user_uuid);

    const handleKick = () => {
        confirmPromptDestructive(
            `Kick '${curMember?.name}'?`,
            'They will need another invite to join back.',
            () => {
                setKicking(true);
                GroupService.remove_member(curGroup?.uuid!, user_uuid as string)
                    .then(() => {
                        navigation.goBack();
                    })
                    .catch((e) => {
                        LoggingService.error(e);
                        alertPrompt('Something went wrong.');
                        setKicking(false);
                    });
            },
            undefined,
            'Kick'
        );
    };

    if (!curMember) return null;

    return (
        <View style={styles.container}>
            <GroupMemberDisplay member={curMember} />
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
