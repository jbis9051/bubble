import React from 'react';
import MapboxGL from '@rnmapbox/maps';
import MapTemplate from '../MapTemplate';

MapboxGL.setAccessToken(process.env.REACT_APP_MAPBOX_ACCESS_TOKEN as string);

const Map = () => (
    <MapTemplate
        style={{
            overflow: 'hidden',
            height: 300,
            borderRadius: 15,
            marginBottom: 10,
        }}
    />
);

export default Map;
