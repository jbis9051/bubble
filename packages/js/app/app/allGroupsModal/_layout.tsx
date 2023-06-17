import React from 'react';
import { Stack } from 'expo-router';

export const unstable_settings = {
    initialRouteName: 'index',
};

export default function BubbleListModalLayout() {
    return (
        <Stack
            screenOptions={{ presentation: "modal", }}
        >
            <Stack.Screen
                name="index"
                options={{ presentation: "modal", title: "Bubbles" }}
            />
            <Stack.Screen
                name="newGroup"
                options={{ presentation: "card", title: "Create Bubble" }}
            />
        </Stack>
    );
}