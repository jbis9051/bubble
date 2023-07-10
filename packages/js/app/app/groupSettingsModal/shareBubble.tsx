import { StatusBar } from 'expo-status-bar';
import {Platform, StyleSheet, View} from 'react-native';
import { useRoute } from '@react-navigation/native';
import { useSelector } from 'react-redux';
import StyledText from '../../components/StyledText';
import InviteUserComponent from '../../components/display/InviteUserComponent';
import { selectCurrentGroup } from '../../redux/slices/groupSlice';

export default function ShareBubble() {
    const curBubble = useSelector(selectCurrentGroup);

    if (!curBubble) return null;

    return (
        <View style={styles.container}>
            <StyledText style={{ marginVertical: 15, marginLeft: 0 }}>
                Invite Members to {curBubble.name}
            </StyledText>
            <InviteUserComponent groupUuid={curBubble.uuid} />
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
