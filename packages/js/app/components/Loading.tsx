import React, { useState, useEffect } from 'react';
import { ActivityIndicator, StyleSheet, View } from 'react-native';

interface LoadingProps {
    shouldCenter?: boolean;
    delay?: number;
    defaultDelay?: boolean;
}
const DEFAULT_DELAY = 1000;
export function Loading(props: LoadingProps) {
    const { shouldCenter, delay, defaultDelay } = props;
    const [show, setShow] = useState<boolean>(!(defaultDelay || delay));
    const cdelay = defaultDelay ? DEFAULT_DELAY : delay;

    useEffect(() => {
        if (!show) {
            setTimeout(() => {
                setShow(true);
            }, cdelay);
        }
    }, []);

    const viewStyle = shouldCenter ? styles.centerView : null;

    if (!show) return <View style={viewStyle} />;

    return (
        <View style={viewStyle}>
            <ActivityIndicator />
        </View>
    );
}

const styles = StyleSheet.create({
    centerView: {
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100%',
        width: '100%',
    },
});

export function FlatListLoadingFooter() {
    return (
        <View style={{ paddingVertical: 20 }}>
            <ActivityIndicator />
        </View>
    );
}
