import React, { useRef } from 'react';
import { View, TextInput, Dimensions, TextBase } from 'react-native';
import { EdgeInsets } from 'react-native-safe-area-context';
import { FontAwesomeIcon } from '@fortawesome/react-native-fontawesome';
import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';
import styles from './styles';

const SearchBar: React.FunctionComponent<{
    insets: EdgeInsets;
    isFocused: boolean;
    setFocus: React.Dispatch<React.SetStateAction<boolean>>;
    setSearch: React.Dispatch<React.SetStateAction<string>>;
}> = ({ insets, isFocused, setFocus, setSearch }) => {
    const deviceWidth = Dimensions.get('window').width;
    const searchBar = useRef<TextInput>(null);

    if (isFocused) {
        searchBar?.current?.focus();
    } else {
        searchBar?.current?.blur();
    }

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
            <TextInput
                style={{
                    flex: 1,
                }}
                placeholder="Search groups"
                ref={searchBar}
                onChangeText={setSearch}
                onFocus={() => setFocus(true)}
                onBlur={() => setFocus(false)}
            />
        </View>
    );
};
export default SearchBar;
