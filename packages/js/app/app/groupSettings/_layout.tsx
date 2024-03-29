import React from 'react';
import { Stack } from 'expo-router';

export default function GroupSettingsLayout() {
    return (
        <Stack screenOptions={{ presentation: 'modal' }}>
            <Stack.Screen
                name="index"
                options={{ presentation: 'modal', title: 'Bubble Settings' }}
            />
            <Stack.Screen
                name="shareBubble"
                options={{ presentation: 'card', title: 'Invite Members' }}
            />
            <Stack.Screen
                name="memberDisplay"
                options={{ presentation: 'card', title: 'Group Member' }}
            />
        </Stack>
    );
}
