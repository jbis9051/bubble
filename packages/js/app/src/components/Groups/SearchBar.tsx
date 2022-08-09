import React from 'react';
import { View, TextInput, Dimensions } from 'react-native';
import { EdgeInsets } from 'react-native-safe-area-context';
import { FontAwesomeIcon } from '@fortawesome/react-native-fontawesome';
import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';
import styles from './styles';

const SearchBar: React.FunctionComponent<{ insets: EdgeInsets }> = ({
    insets,
}) => {
    const deviceWidth = Dimensions.get('window').width;
    return (
        <View
            style={{
                ...styles.searchBar,
                top: insets.top + 5,
                width: deviceWidth - 30,
            }}
        >
            <FontAwesomeIcon
                icon={faMagnifyingGlass}
                style={styles.searchIcon}
            />
            <TextInput placeholder="Search groups" />
        </View>
    );
};
export default SearchBar;
