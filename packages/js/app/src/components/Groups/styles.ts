import { StyleSheet } from 'react-native';

import Colors from '../../constants/Colors';

const styles = StyleSheet.create({
    userView: {
        flexDirection: 'row',
        flexWrap: 'wrap',
        alignItems: 'center',
        justifyContent: 'center',
    },
    groupView: {
        flexDirection: 'row',
        flexWrap: 'wrap',
        alignItems: 'center',
        justifyContent: 'center',
    },
    userIcon: {
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        margin: 13,
    },
    groupContainer: {
        alignItems: 'center',
        justifyContent: 'center',
        margin: 13,
    },
    groupIcon: {
        borderRadius: 15,
        height: 100,
        width: 100,
        borderWidth: 2.5,
        borderColor: Colors.grey,
    },
    peopleHeading: {
        fontSize: 30,
        fontWeight: '800',
    },
});

export default styles;
