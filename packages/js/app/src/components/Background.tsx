import React from 'react';
import { ImageBackground } from 'react-native';
import { Image } from 'react-native-svg';

import source from '../constants/BackgroundImage';

const Background: React.FunctionComponent<{ children: Element }> = ({
    children
}) => (
    <ImageBackground
        source={source}
    >
        {children}
    </ImageBackground>
);

export default Background;