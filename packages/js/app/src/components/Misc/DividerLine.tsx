import React from 'react';
import { View } from 'react-native';

import Colors from '../../constants/Colors';

const DividerLine = () => (
    <View
        style={{
            width: '100%',
            backgroundColor: Colors.grey,
            height: 1,
            marginTop: 15,
            borderRadius: 100,
        }}
    ></View>
);

export default DividerLine;
