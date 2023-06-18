import { hello, init, multiply } from '@bubble/react-native-bubble-rust';
import { StyleSheet } from 'react-native';

import { useEffect, useState } from 'react';
import EditScreenInfo from '../../components/EditScreenInfo';
import { Text, View } from '../../components/Themed';

export default function TabTwoScreen() {
    const [math, setMath] = useState('');

    useEffect(() => {
        init('.')
            .then(() => hello('Josh'))
            .then((result) => {
                if (result.success) {
                    setMath(result.value.message);
                }
            });
    }, []);

    return (
        <View style={styles.container}>
            <Text style={styles.title}>Tab Two</Text>
            <View
                style={styles.separator}
                lightColor="#eee"
                darkColor="rgba(255,255,255,0.1)"
            />
            <EditScreenInfo path="app/(tabs)/two.tsx" />
            <Text>Bubble: {math}</Text>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        alignItems: 'center',
        justifyContent: 'center',
    },
    title: {
        fontSize: 20,
        fontWeight: 'bold',
    },
    separator: {
        marginVertical: 30,
        height: 1,
        width: '80%',
    },
});
