import React, {useState} from 'react';
import {Alert, StyleSheet, View} from 'react-native';
import StyledText from '../../components/StyledText';
import {StyledInput} from '../../components/Input';
import StyledButton from '../../components/bubbleUI/Button';
import InviteUserComponent from '../../components/display/InviteUserComponent';
import FrontendInstanceStore from "../../stores/FrontendInstanceStore";
import MainStore from "../../stores/MainStore";

export default function NewGroup() {
    const [name, setName] = useState('');
    const [loading, setLoading] = useState(false);
    const [createdUuid, setCreatedUuid] = useState('');


    const handleCreate = () => {
        if (name.length === 0) {
            return Alert.alert('Please enter a name for your Bubble');
        }
        setLoading(true);
        FrontendInstanceStore.instance.create_group()
            .then(async (new_uuid) => {
                await FrontendInstanceStore.instance.update_group(new_uuid, name);
                const groups = await FrontendInstanceStore.instance.get_groups();
                MainStore.groups = groups;
                MainStore.current_group = groups.find((group) => group.uuid === new_uuid) || null;
                setCreatedUuid(new_uuid);
            })
            .catch((err) => {
                Alert.alert('Error', err.message);
            })
            .finally(() => setLoading(false));
    };

    if (createdUuid) {
        return (
            <View style={styles.container}>
                <StyledText variant="h2" nomargin style={{marginBottom: 15}}>
                    Bubble Created!
                </StyledText>
                <InviteUserComponent groupUuid={createdUuid}/>
            </View>
        );
    }

    return (
        <View style={styles.container}>
            <StyledText variant="h2" nomargin style={{marginBottom: 15}}>
                Name Your Bubble
            </StyledText>
            <StyledInput label="Bubble Name" value={name} onChange={setName}/>
            <StyledButton
                color="primary"
                style={{marginBottom: 15, marginTop: 'auto'}}
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
