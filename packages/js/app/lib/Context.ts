import React from 'react';
import Colors from '../constants/Colors';

export const ThemeContext = React.createContext<typeof Colors.light>(
    Colors.light
);
