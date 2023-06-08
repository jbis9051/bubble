import React, { useContext } from 'react';
import {
    TouchableOpacity,
    Text,
    StyleProp,
    ActivityIndicator
} from 'react-native';
import { ThemeContext } from '../lib/Context';

interface StyledButtonProps {
    color: ColorTypes;
    variant?: 'filled' | 'outlined';
    children: React.ReactText;
    style?: StyleProp<any>;
    onPress?: () => void;
    loading?: boolean;
    disabled?: boolean;
    fontSize?: number;
}

type ColorTypes = 'primary' | 'secondary' | 'danger';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function FilledButton(props: StyledButtonProps) {
    const { color, children, style, onPress, loading, disabled, fontSize } =
        props;
    const theme = useContext(ThemeContext);

    // const OuterComponent = (disabled ? View : TouchableOpacity) as React.ComponentType<any>;
    // const bgcolor = disabled ? "rgba(0,0,0,.06)" : theme.colors[color];
    // TODO: update this to use theme colors
    const bgcolor = 'green';

    return (
        <TouchableOpacity
            style={[
                {
                    width: '90%',
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
                            fontSize,
                        }}
                    >
                        {children}
                    </Text>
                );
            })()}
        </TouchableOpacity>
    );
}
