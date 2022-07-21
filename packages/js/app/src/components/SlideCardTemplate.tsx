import React from 'react';
import { View } from 'react-native';

import Styles from './Styles';

const SlideCardTemplate: React.FunctionComponent<{ children: JSX.Element, style: {} }> = ({
    children,
    style
}) => (
    <View style={{
        ...Styles.slideCardTemplate, 
        ...style
    }}>
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
