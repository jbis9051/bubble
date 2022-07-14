import React, { Dispatch, SetStateAction } from 'react';
import SegmentedControlTemplate from '../SegmentedControlTemplate';

const MapSegmentedControl: React.FunctionComponent<{ setMap: React.Dispatch<React.SetStateAction<string>> }> = ({ setMap }) => (
    <SegmentedControlTemplate 
        options={['Street', 'Satellite']} 
        setState={setMap}
    />
);

export default MapSegmentedControl