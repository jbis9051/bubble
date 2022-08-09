import React from 'react';
import { Text } from 'react-native';

const Name: React.FunctionComponent<{ name: string; lightText?: boolean }> = ({
    name,
    lightText = false,
}) => (
    <Text
        style={{
            marginTop: 8,
            textAlign: 'center',
            color: lightText ? '#ffffff' : '#000000',
        }}
        numberOfLines={1}
        adjustsFontSizeToFit={true}
    >
        {name}
    </Text>
);

export default Name;
