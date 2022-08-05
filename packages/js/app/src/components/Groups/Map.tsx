import React from 'react';
import MapTemplate from '../MapTemplate';

interface Location {
    longitude: number;
    latitude: number;
}

const Map: React.FunctionComponent<{
    location: Location;
    markerLocation: Location;
}> = ({ location, markerLocation }) => (
    <MapTemplate
        region={location}
        markerRegion={markerLocation}
        style={{ flex: 1 }}
    />
);

export default Map;
