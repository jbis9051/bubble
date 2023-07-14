import React from 'react';
import {StyleSheet, View} from 'react-native';
import StyledText from '../StyledText';
import Avatar from '../Avatar';
import { UserOut } from '@bubble/react-native-bubble-rust';

interface GroupMemberDisplayProps {
    member: UserOut;
}
export function GroupMemberDisplay({ member }: GroupMemberDisplayProps) {
    return (
        <View style={styles.container}>
            <View
                style={{
                    width: '100%',
                    display: 'flex',
                    flexDirection: 'row',
                    justifyContent: 'center',
                    alignItems: 'center',
                    padding: 20,
                }}
            >
                <Avatar name={member.name} width="25%" textVariant="h2" />
                <StyledText
                    nomargin
                    variant="h2"
                    style={{ flex: 1, textAlign: 'center' }}
                >
                    {member.name}
                </StyledText>
            </View>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
    },
});
