import React from 'react';
import { Region } from 'react-native-maps';
import MapTemplate from '../MapTemplate';

type UserLocation = {
    name?: string;
    location: Region;
};

const Map: React.FunctionComponent<{
    locations: UserLocation[];
}> = ({ locations }) => (
    <MapTemplate locations={locations} style={{ flex: 1 }} />
);

export default Map;
