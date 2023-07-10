import React, { useState } from 'react';
import { Alert, View } from 'react-native';
import { Feather } from '@expo/vector-icons';
import StyledText from '../StyledText';
import { GroupService } from '../../lib/bubbleApi/group';
import { LoggingService } from '../../lib/bubbleApi/logging';
import { StyledInput } from '../Input';
import StyledButton from '../bubbleUI/Button';

interface InviteUserComponentProps {
    groupUuid: string;
}
export default function InviteUserComponent({
    groupUuid,
}: InviteUserComponentProps) {
    const [username, setUsername] = useState('');

    const handleInvite = () => {
        if (!username.length) return Alert.alert('Please enter a username');
        GroupService.invite_user(groupUuid, username)
            .then(() => {
                setUsername('');
            })
            .catch(LoggingService.error);
    };

    return (
        <>
            <StyledInput
                label="Invite Username"
                value={username}
                onChange={setUsername}
            />
            <View
                style={{
                    display: 'flex',
                    flexDirection: 'row',
                    alignItems: 'center',
                    marginTop: 10,
                }}
            >
                <Feather name="info" size={20} color="black" />
                <StyledText nomargin style={{ marginLeft: 5 }} variant="mini">
                    You can find the username in the account tab.
                </StyledText>
            </View>
            <StyledButton
                color="primary"
                style={{ marginBottom: 15, marginTop: 'auto' }}
                onPress={handleInvite}
                disabled={!username.length}
            >
                Create
            </StyledButton>
        </>
    );
}
