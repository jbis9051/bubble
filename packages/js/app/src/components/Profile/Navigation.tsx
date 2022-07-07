import React from 'react';
import {
    Pressable,
    Text
} from 'react-native';

import NavigationTemplate from '../NavigationTemplate';
import Styles from './Styles';

const ChildrenComponent = (
    <Pressable>
        <Text style={Styles.editButton}>Edit</Text>
    </Pressable>
);

const Navigation = () => (
    <NavigationTemplate children={ChildrenComponent} />
);

export default Navigation;