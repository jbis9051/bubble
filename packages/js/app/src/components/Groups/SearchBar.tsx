import React, { useRef } from 'react';
import { View, TextInput, Dimensions } from 'react-native';
import { EdgeInsets } from 'react-native-safe-area-context';
import { FontAwesomeIcon } from '@fortawesome/react-native-fontawesome';
import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';
import styles from './styles';

const SearchBar: React.FunctionComponent<{
    insets: EdgeInsets;
    search: string;
    isBlurred: boolean;
    isFocused: boolean;
    setFocus: React.Dispatch<React.SetStateAction<boolean>>;
    setBlur: React.Dispatch<React.SetStateAction<boolean>>;
    setSearch: React.Dispatch<React.SetStateAction<string>>;
}> = ({
    insets,
    search,
    isBlurred,
    isFocused,
    setFocus,
    setBlur,
    setSearch,
}) => {
    const deviceWidth = Dimensions.get('window').width;
    const searchBar = useRef<TextInput>(null);

    if (isFocused && !isBlurred) {
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
                value={search}
                placeholder="Search groups"
                ref={searchBar}
                onChangeText={setSearch}
                onFocus={() => {
                    setFocus(true);
                    setBlur(false);
                }}
                onBlur={() => {
                    setBlur(true);
                }}
            />
        </View>
    );
};
export default SearchBar;
