import React from 'react';
import { Region } from 'react-native-maps';
import MapTemplate from '../MapTemplate';

const Map: React.FunctionComponent<{
    locations: Region[];
}> = ({ locations }) => (
    <MapTemplate locations={locations} style={{ flex: 1 }} />
);

export default Map;
