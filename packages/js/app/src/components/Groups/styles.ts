import { StyleSheet } from 'react-native';

const styles = StyleSheet.create({
    userView: {
        flexDirection: 'row',
        flexWrap: 'wrap',
        alignItems: 'center',
        justifyContent: 'center',
        backgroundColor: '#ffffff',
    },
    userIcon: {
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        margin: 15,
    },
    peopleHeading: {
        fontSize: 30,
        fontWeight: '800',
    },
});

export default styles;
