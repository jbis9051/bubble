import React from 'react';
import StyledText, { CustomTextProps } from './StyledText';
import {View} from "react-native";
import Colors from "../constants/Colors";

interface AvatarProps {
    name: string;
    width: string | number;
    textVariant?: CustomTextProps['variant'];
}
export default function Avatar({ name, width, textVariant }: AvatarProps) {
    const initials = name
        .split(' ')
        .map((n) => n[0])
        .join('');

    return (
        <View
            style={{
                width,
                aspectRatio: 1,
                borderRadius: 9999,
                backgroundColor: Colors.colors.secondaryPaper,
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
            }}
        >
            <StyledText
                nomargin
                style={{ color: 'white' }}
                variant={textVariant}
            >
                {initials}
            </StyledText>
        </View>
    );
}
