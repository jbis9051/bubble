import React, { useEffect, useState } from 'react';
import { Alert, Touchable, TouchableHighlight, View } from 'react-native';
import { Feather } from '@expo/vector-icons';
import { UserOut } from '@bubble/react-native-bubble-rust';
import { useNavigation } from 'expo-router';
import StyledText from '../StyledText';
import { StyledInput } from '../Input';
import StyledButton from '../bubbleUI/Button';
import groups from '../../app/groups';
import FrontendInstanceStore from '../../stores/FrontendInstanceStore';
import Colors from '../../constants/Colors';
import { BubbleMember } from '../../app/groupSettings';
import MainStore from '../../stores/MainStore';

interface InviteUserComponentProps {
    groupUuid: string;
}

export default function InviteUserComponent({
    groupUuid,
}: InviteUserComponentProps) {
    const [username, setUsername] = useState('');
    const [searchResults, setSearchResults] = useState<UserOut[]>([]);
    const [currentSearchTimeout, setCurrentSearchTimeout] =
        useState<null | NodeJS.Timer>(null);
    const navigation = useNavigation();

    const handleInvite = () => {
        if (username.length === 0) {
            return Alert.alert('Please enter a username');
        }
        const uuid = searchResults.find(
            (user) => user.username === username
        )?.uuid;
        if (!uuid) {
            return Alert.alert('User not found');
        }
        if (MainStore.current_group === null) {
            return Alert.alert('No group selected');
        }
        FrontendInstanceStore.instance
            .add_member(MainStore.current_group.uuid, uuid)
            .then(() =>
                FrontendInstanceStore.instance.send_group_status(
                    MainStore.current_group!.uuid
                )
            )
            .then(async () => {
                MainStore.groups =
                    await FrontendInstanceStore.instance.get_groups();
                MainStore.current_group =
                    MainStore.groups.find(
                        (group) => MainStore.current_group?.uuid === group.uuid
                    ) || null;
                navigation.goBack();
            })
            .catch((err) => {
                Alert.alert('Error Inviting User', err.message);
            });
    };

    useEffect(() => () => {
            if (currentSearchTimeout) {
                clearTimeout(currentSearchTimeout);
            }
        }, []);

    return (
        <>
            <StyledInput
                label="Invite Username"
                value={username}
                onChange={(e) => {
                    if (currentSearchTimeout) {
                        clearTimeout(currentSearchTimeout);
                    }
                    setUsername(e);
                    if (e.length === 0) {
                        return setSearchResults([]);
                    }
                    setCurrentSearchTimeout(
                        setTimeout(async () => {
                            const results =
                                await FrontendInstanceStore.instance.search(e);
                            setSearchResults(results);
                        }, 1000)
                    );
                }}
            />
            <View
                style={{
                    display: 'flex',
                    alignItems: 'center',
                    marginTop: 10,
                }}
            >
                {searchResults.map((result, i) => (
                    <View
                        style={{
                            borderTopColor: Colors.colors.secondaryPaper,
                            borderBottomColor: Colors.colors.secondaryPaper,
                            borderTopWidth: i === 0 ? 1 : 0,
                            borderBottomWidth: 1,
                            width: '100%',
                        }}
                        key={result.uuid}
                    >
                        <BubbleMember
                            OnPress={() => {
                                setUsername(result.username);
                            }}
                            member={result}
                        />
                    </View>
                ))}
            </View>
            <StyledButton
                color="primary"
                style={{ marginBottom: 15, marginTop: 'auto' }}
                onPress={handleInvite}
                disabled={!username.length}
            >
                Invite
            </StyledButton>
        </>
    );
}
