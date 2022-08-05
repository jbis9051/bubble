import React from 'react';
import MapTemplate from '../MapTemplate';

interface Location {
    longitude: number;
    latitude: number;
}

const Map: React.FunctionComponent<{
    location: Location;
}> = ({ location }) => <MapTemplate region={location} style={{ flex: 1 }} />;

export default Map;
