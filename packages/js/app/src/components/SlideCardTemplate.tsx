import React from 'react';
import { View } from 'react-native';

import Styles from './Styles';

const SlideCardTemplate: React.FunctionComponent<{ children: JSX.Element }> = ({
    children,
}) => (
    <View style={Styles.slideCardTemplate}>
        <View
            style={{
                width: '100%',
                alignItems: 'center',
                justifyContent: 'center',
            }}
        >
            <View
                style={{
                    width: 64,
                    height: 8,
                    backgroundColor: '#d3d3d3',
                    borderRadius: 100,
                }}
            ></View>
        </View>
        {children}
    </View>
);

export default SlideCardTemplate;
