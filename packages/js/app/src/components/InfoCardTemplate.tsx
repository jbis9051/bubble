import React from 'react';
import { View } from 'react-native';

import Styles from './Styles';

const InfoCardTemplate: React.FunctionComponent<{ children: Element }> = ({
    children,
}) => (
    <View
        style={{
            ...Styles.infoCardTemplate,
            ...Styles.shadow,
        }}
    >
        {children}
    </View>
);

export default InfoCardTemplate;
