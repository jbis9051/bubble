import React from 'react';
import { Image } from 'react-native';

import Styles from './Styles';

interface ProfileImageProps {
    source: string;
    size: number;
}

const ProfileImageTemplate: React.FunctionComponent<ProfileImageProps> = ({
    size,
}) => (
    <Image
        /* eslint-disable global-require */
        style={{
            ...Styles.profileImage,
            height: size,
            width: size,
        }}
    />
);

export default ProfileImageTemplate;
