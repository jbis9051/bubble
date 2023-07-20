import React, { useState } from 'react';
import { useLocalSearchParams, useNavigation } from 'expo-router';
import { Alert, StyleSheet, View } from 'react-native';
import { GroupMemberDisplay } from '../../components/display/GroupMemberDisplay';
import StyledButton from '../../components/bubbleUI/Button';
import MainStore from '../../stores/MainStore';
import FrontendInstanceStore from '../../stores/FrontendInstanceStore';

export default function MemberDisplay() {
    const { user_uuid } = useLocalSearchParams();
    const navigation = useNavigation();

    const [kicking, setKicking] = useState(false);

    const curMember =
        MainStore.current_group?.members[user_uuid as string].info;

    if (!curMember) {
        return null;
    }

    const handleKick = () => {
        Alert.alert(
            `Kick '${curMember.name}'?`,
            'They will need another invite to join back.',
            [
                {
                    text: 'OK',
                    style: 'destructive',
                    onPress: () => {
                        setKicking(true);
                        if (!MainStore.current_group) {
                            return;
                        }
                        FrontendInstanceStore.instance
                            .remove_member(MainStore.current_group.uuid, curMember.uuid)
                            .then(async () => {
                                MainStore.groups = await FrontendInstanceStore.instance.get_groups();
                                MainStore.current_group = MainStore.groups.find(g => g.uuid === MainStore.current_group?.uuid) || MainStore.groups[0] || null;
                                navigation.goBack();
                            })
                            .catch((err) => {
                                Alert.alert('Error', err.message);
                            })
                            .finally(() => {
                                setKicking(false);
                            });
                    },
                },
                {
                    text: 'Cancel',
                    style: 'cancel',
                },
            ]
        );
    };

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
