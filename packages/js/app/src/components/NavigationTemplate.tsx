import React from 'react';
import { View } from 'react-native';

import Styles from './Styles';

const NavigationTemplate: React.FunctionComponent<{ children: JSX.Element }> = ({children}) => (
    <View style={Styles.navigation}>
        {children}
    </View>
);

export default NavigationTemplate;