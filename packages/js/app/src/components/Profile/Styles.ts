import { StyleSheet } from 'react-native';

const Styles = StyleSheet.create({
    shadow: {
        shadowColor: '#171717',
        shadowOffset: { width: -3, height: 10 },
        shadowOpacity: 0.1,
        shadowRadius: 10,
    },
    navigation: {
        height: 40,
        width: '100%',
        justifyContent: 'flex-end',
        flexDirection: 'row',
        paddingRight: 20,
    },
    editButton: {
        color: '#ffffff',
        fontSize: 24,
        fontWeight: '600',
    },
    header: {
        alignItems: 'center',
        justifyContent: 'center',
        flexDirection: 'column',
        padding: 10,
        height: 206,
    },
    headerText: {
        fontSize: 36,
        color: '#ffffff',
    },
    heading: {
        fontSize: 30,
        fontWeight: '800',
        marginBottom: 10,
    },
});

export default Styles;
