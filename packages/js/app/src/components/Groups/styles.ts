import { StyleSheet } from 'react-native';

import Colors from '../../constants/Colors';

const styles = StyleSheet.create({
    shadow: {
        shadowColor: '#171717',
        shadowOffset: { width: -3, height: 10 },
        shadowOpacity: 0.1,
        shadowRadius: 10,
    },
    searchIcon: {
        padding: 5,
        marginRight: 5,
        height: 14,
        color: '#d3d3d3',
    },
    searchBar: {
        position: 'absolute',
        flexDirection: 'row',
        height: 35,
        width: '100%',
        borderRadius: 10,
        fontSize: 14,
        backgroundColor: '#ffffff',
        padding: 10,
        marginLeft: 15,
    },
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
        margin: 8,
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
