import React, { useEffect } from 'react';
import { ViewStyle } from 'react-native';
import Animated, { AnimateStyle, useAnimatedStyle, useSharedValue, withTiming } from 'react-native-reanimated';

interface FadeInViewProps {
    show?: boolean;
    style?: AnimateStyle<ViewStyle>;
    children: React.ReactNode;
}
export function FadeInView(props: FadeInViewProps) {
    const { show, children, style } = props;
    const opacity = useSharedValue(0);
    const cshow = show === undefined ? true : show;

    const animatedViewStyle = useAnimatedStyle(() => {
        return {
            opacity: opacity.value,
        }
    });

    useEffect(() => {
        opacity.value = withTiming(cshow ? 1 : 0);
    }, [cshow])

    return (
        <Animated.View
            style={[animatedViewStyle, style]}
        >
            {children}
        </Animated.View>
    )
}