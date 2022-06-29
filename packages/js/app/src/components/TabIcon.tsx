import React from 'react';
import { 
    View, 
    Text
} from 'react-native';

import { FontAwesomeIcon } from '@fortawesome/react-native-fontawesome';
import { IconProp } from '@fortawesome/fontawesome-svg-core';


interface TabIconProps {
    name: string;
    icon: IconProp;
    focused: boolean
}

const TabIcon: React.FC<TabIconProps> = ({ name, icon, focused }) => {
    const color = focused ? '#e32f45' : '#748c94';
    return (
        <View style={{alignItems: 'center', justifyContent: 'center', top: 10} }>
            <FontAwesomeIcon 
                icon={icon} 
                color={color} 
                size={35}
            />
            <Text
                style={{
                    color: color, 
                    fontSize: 12 
                }}
            >
                {name}
            </Text>
        </View>
    );
};

export default TabIcon;
