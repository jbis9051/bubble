import React, { useState } from 'react';

import { Alert, SafeAreaView, StyleSheet } from 'react-native';

import { useDispatch } from 'react-redux';
import { View } from '../../components/Themed';
import StyledText from '../../components/StyledText';
import { StyledInput } from '../../components/Input';
import StyledButton from '../../components/bubbleUI/Button';
import { GroupService } from '../../lib/bubbleApi/group';
import { LoggingService } from '../../lib/bubbleApi/logging';
import { setActiveGroup, setGroups } from '../../redux/slices/groupSlice';
import InviteUserComponent from '../../components/display/InviteUserComponent';

export default function NewGroup() {
    const [name, setName] = useState('');
    const [loading, setLoading] = useState(false);
    const [createdUuid, setCreatedUuid] = useState('');

    const dispatch = useDispatch();

    const handleCreate = () => {
        if (!name.length)
            return Alert.alert('Please enter a name for your Bubble');
        setLoading(true);
        GroupService.create_group(name)
            .then((newGroupUuid) => {
                GroupService.get_groups()
                    .then((groups) => {
                        dispatch(setGroups(groups));
                        dispatch(setActiveGroup(newGroupUuid));
                        setCreatedUuid(newGroupUuid);
                    })
                    .catch(LoggingService.error);
            })
            .catch(LoggingService.error)
            .finally(() => setLoading(false));
    };

    if (createdUuid) {
        return (
            <View style={styles.container}>
                <StyledText variant="h2" nomargin style={{ marginBottom: 15 }}>
                    Bubble Created!
                </StyledText>
                <InviteUserComponent groupUuid={createdUuid} />
            </View>
        );
    }

    return (
        <View style={styles.container}>
            <StyledText variant="h2" nomargin style={{ marginBottom: 15 }}>
                Name your Bubble
            </StyledText>
            <StyledInput label="Bubble Name" value={name} onChange={setName} />
            <StyledButton
                color="primary"
                style={{ marginBottom: 15, marginTop: 'auto' }}
                onPress={handleCreate}
                disabled={!name.length}
            >
                Create
            </StyledButton>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        padding: 15,
    },
});
