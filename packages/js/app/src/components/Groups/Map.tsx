import React from 'react';
import { Region } from 'react-native-maps';
import MapTemplate from '../MapTemplate';

const Map: React.FunctionComponent<{
    location: Region;
}> = ({ location }) => <MapTemplate region={location} style={{ flex: 1 }} />;

export default Map;
