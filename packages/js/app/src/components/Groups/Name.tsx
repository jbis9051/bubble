import React from 'react';
import { Text } from 'react-native';

const Name: React.FunctionComponent<{ name: string }> = ({ name }) => (
    <Text
        style={{ marginTop: 8, textAlign: 'center' }}
        numberOfLines={1}
        adjustsFontSizeToFit={true}
    >
        {name}
    </Text>
);

export default Name;
