import React, { useContext, useEffect } from 'react';
import {
    TouchableOpacity,
    Text,
    StyleProp,
    Platform,
    ViewStyle,
    View,
} from 'react-native';
import { ActivityIndicator } from 'react-native';
import * as Haptics from 'expo-haptics';
import { ThemeContext } from '../../lib/Context';

const isAndroid = Platform.OS === 'android';

type ColorTypes = 'primary' | 'secondary' | 'danger';

function FilledButton(props: StyledButtonProps) {
    const { color, children, style, onPress, loading, disabled, fontSize } =
        props;
    const theme = useContext(ThemeContext);

    // const OuterComponent = (disabled ? View : TouchableOpacity) as React.ComponentType<any>;
    const bgcolor = disabled ? 'rgba(0,0,0,.06)' : theme.colors[color];

    return (
        <TouchableOpacity
            style={[
                {
                    height: 40,
                    backgroundColor: bgcolor,
                    borderRadius: 15,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    flexDirection: 'row',
                },
                style,
            ]}
            onPress={onPress}
        >
            {(() => {
                if (loading)
                    return (
                        <ActivityIndicator style={{ marginHorizontal: 15 }} />
                    );
                return (
                    <Text
                        style={{
                            color: disabled
                                ? theme.colors.secondaryPaper
                                : theme.complementColors[color],
                            fontSize: fontSize,
                        }}
                    >
                        {children}
                    </Text>
                );
            })()}
        </TouchableOpacity>
    );
}

function OutlinedButton(props: StyledButtonProps) {
    const { color, children, style, onPress, loading, disabled, fontSize } =
        props;
    const theme = useContext(ThemeContext);

    const forecolor = disabled ? 'rgba(0,0,0,.06)' : theme.colors[color];
    return (
        <TouchableOpacity
            style={[
                {
                    height: 40,
                    backgroundColor: theme.background,
                    borderStyle: 'solid',
                    borderWidth: 1,
                    borderRadius: 15,
                    borderColor: forecolor,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    flexDirection: 'row',
                },
                style,
            ]}
            onPress={onPress}
        >
            {(() => {
                if (loading)
                    return (
                        <ActivityIndicator style={{ marginHorizontal: 15 }} />
                    );
                return (
                    <Text
                        style={{
                            color: forecolor,
                            fontSize: fontSize,
                        }}
                    >
                        {children}
                    </Text>
                );
            })()}
        </TouchableOpacity>
    );
}

interface StyledButtonProps {
    color: ColorTypes;
    variant?: 'filled' | 'outlined';
    children: React.ReactText;
    style?: StyleProp<ViewStyle>;
    onPress?: () => void;
    loading?: boolean;
    disabled?: boolean;
    fontSize?: number;
}
export default function StyledButton(props: StyledButtonProps) {
    const { variant, loading } = props;

    const onPressHaptic = () => {
        Haptics.selectionAsync();
        props.onPress && props.onPress();
    };

    switch (variant) {
        case 'outlined':
            return <OutlinedButton {...props} onPress={onPressHaptic} />;
        case 'filled':
        default:
            return <FilledButton {...props} onPress={onPressHaptic} />;
    }
}

interface TextButtonProps {
    children: React.ReactText;
    color?: ColorTypes;
    onPress?: () => void;
    style?: StyleProp<ViewStyle>;
    disabled?: boolean;
    nomargin?: boolean;
    fontSize?: number;
    inHeader?: boolean;
}
export function TextButton(props: TextButtonProps) {
    const {
        children,
        onPress,
        color,
        style,
        disabled,
        nomargin,
        fontSize,
        inHeader,
    } = props;

    let noMarginStyle: typeof style = {};
    if (nomargin || inHeader) {
        noMarginStyle = {
            margin: 0,
            marginTop: 0,
            marginBottom: 0,
            marginLeft: 0,
            marginRight: 0,
        };
    }
    let _fontSize = 20;
    if (inHeader) {
        _fontSize = 16;
    } else if (fontSize) {
        _fontSize = fontSize;
    }

    return (
        <>
            <TouchableOpacity
                disabled={disabled}
                onPress={onPress ? onPress : () => {}}
                style={[
                    { margin: 15, marginTop: isAndroid ? 15 : 20 },
                    noMarginStyle,
                    style,
                ]}
            >
                <Text
                    style={{
                        fontSize: _fontSize,
                        color: color === 'secondary' ? 'black' : '#007AFF',
                    }}
                >
                    {children}
                </Text>
            </TouchableOpacity>
        </>
    );
}
