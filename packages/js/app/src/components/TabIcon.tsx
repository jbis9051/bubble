import React from 'react';
import { View, Text } from 'react-native';

import { FontAwesomeIcon } from '@fortawesome/react-native-fontawesome';
import { IconProp } from '@fortawesome/fontawesome-svg-core';

import Colors from '../constants/Colors';

interface TabIconProps {
    name: string;
    icon: IconProp;
    focused: boolean;
}

const TabIcon: React.FunctionComponent<TabIconProps> = ({
    name,
    icon,
    focused,
}) => {
    const color = focused ? Colors.primary : Colors.unselected;
    return (
        <View
            style={{ alignItems: 'center', justifyContent: 'center', top: 5 }}
        >
            <FontAwesomeIcon icon={icon} color={color} size={30} />
            <Text
                style={{
                    color,
                    fontSize: 12,
                }}
            >
                {name}
            </Text>
        </View>
    );
};

export default TabIcon;
