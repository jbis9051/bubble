import React from 'react';

import { Image, ImageProps } from 'react-native';

export function ImageDisplay(props: ImageProps) {
    return (
        <Image
            {...props}
            source={{
                cache: 'force-cache',
                // @ts-ignore
                ...(props.source || {}),
            }}
        />
    );
}