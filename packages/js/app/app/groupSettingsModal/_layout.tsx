import React from 'react';
import { Stack } from 'expo-router';

export const unstable_settings = {
    initialRouteName: 'index',
};

export default function GroupListModalLayout() {
    return (
        <Stack
            screenOptions={{ presentation: "modal", }}
        >
            <Stack.Screen
                name="index"
                options={{ presentation: "modal", title: "Bubble Settings" }}
            />
            <Stack.Screen
                name="shareBubble"
                options={{ presentation: "card", title: "Invite Members" }}
            />
            <Stack.Screen
                name="memberDisplay"
                options={{ presentation: "card", title: "View Member" }}
            />
        </Stack>
    );
}