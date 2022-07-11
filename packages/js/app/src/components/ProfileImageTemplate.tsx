import React from 'react';
import { Image } from 'react-native';
import ProfileImageProps from '../Interfaces/ProfileImageProps';

import Styles from './Styles';

const ProfileImageTemplate: React.FunctionComponent<ProfileImageProps> = ({ source, size }) => {
    return (
        <Image
                 // React native documentation says that all images needs to be compiled before bundled
                /* eslint-disable global-require */
                // source={require(source)}
                style={{
                    ...Styles.profileImage,
                    height: size,
                    width: size
                }}
        />
    );
};

export default ProfileImageTemplate;