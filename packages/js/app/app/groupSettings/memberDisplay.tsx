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
                        FrontendInstanceStore.instance
                            .leave_group(curMember.uuid)
                            .then(() => {
                                navigation.goBack();
                            })
                            .catch((err) => {
                                Alert.alert('Error', err);
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
