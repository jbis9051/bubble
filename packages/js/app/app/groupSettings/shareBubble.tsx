import { StatusBar } from 'expo-status-bar';
import { Platform, StyleSheet, View } from 'react-native';
import StyledText from '../../components/StyledText';
import InviteUserComponent from '../../components/display/InviteUserComponent';
import MainStore from '../../stores/MainStore';

export default function ShareBubble() {
    const current = MainStore.current_group;

    if (!current) {
        return null;
    }

    return (
        <View style={styles.container}>
            <StyledText style={{ marginVertical: 15, marginLeft: 0 }}>
                Invite Members to {current.name}
            </StyledText>
            <InviteUserComponent groupUuid={current.uuid} />
            <StatusBar style={Platform.OS === 'ios' ? 'light' : 'auto'} />
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        padding: 15,
    },
});
