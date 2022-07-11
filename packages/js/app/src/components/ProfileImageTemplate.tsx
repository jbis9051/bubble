import React from 'react';
import { Image } from 'react-native';
import ProfileImageProps from '../Interfaces/ProfileImageProps';

import Styles from './Styles';

const ProfileImageTemplate: React.FunctionComponent<ProfileImageProps> = ({ source, size }) => (
    <Image
            /* eslint-disable global-require */
            source={require(source)}
            style={{
                ...Styles.profileImage,
                height: size,
                width: size
            }}
    />
);

export default ProfileImageTemplate;