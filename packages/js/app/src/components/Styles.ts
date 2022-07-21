import { StyleSheet } from 'react-native';

const Styles = StyleSheet.create({
    shadow: {
        shadowColor: '#171717',
        shadowOffset: { width: -3, height: 10 },
        shadowOpacity: 0.1,
        shadowRadius: 10,
    },
    slideCardTemplate: {
        padding: 20,
        paddingTop: 10,
        paddingBottom: 30,
        backgroundColor: '#ffffff',
        borderTopRightRadius: 30,
        borderTopLeftRadius: 30,
        marginTop: 256,
    },
    infoCardTemplate: {
        marginBottom: 10,
        padding: 20,
        backgroundColor: '#ffffff',
        borderRadius: 10,
    },
    navigation: {
        height: 40,
        width: '100%',
        justifyContent: 'flex-end',
        flexDirection: 'row',
        paddingRight: 20,
    },
    profileImage: {
        borderRadius: 100,
        borderWidth: 2.5,
        borderColor: '#d3d3d3',
    },
    background: {
        position: 'absolute',
        zIndex: 0,
        elevation: 0,
    }
});

export default Styles;
