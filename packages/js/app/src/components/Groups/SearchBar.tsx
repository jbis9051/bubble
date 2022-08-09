import React from 'react';
import { View, TextInput } from 'react-native';
import styles from './styles';

const SearchBar = () => (
    <TextInput
        style={{
            ...styles.shadow,
            ...styles.searchBar,
        }}
        placeholder="Search groups"
    />
);

export default SearchBar;
