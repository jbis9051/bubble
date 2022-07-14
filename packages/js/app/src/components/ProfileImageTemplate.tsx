import React from 'react';
import { Image } from 'react-native';
import ProfileImageProps from '../Interfaces/ProfileImageProps';

import Styles from './Styles';

const ProfileImageTemplate: React.FunctionComponent<ProfileImageProps> = ({ source, size }) => {
    return (
        <Image
                /* eslint-disable global-require */
                style={{
                    ...Styles.profileImage,
                    height: size,
                    width: size
                }}
        />
    );
};

export default ProfileImageTemplate;