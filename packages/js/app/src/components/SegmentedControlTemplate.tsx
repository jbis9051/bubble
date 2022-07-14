import React, { Dispatch, useState } from 'react';
import {
    View,
    TouchableOpacity,
    Text
} from 'react-native';

import Style from './Styles';

const SegmentedControlTemplate: React.FunctionComponent<{ 
    options: string[], 
    setState: React.Dispatch<React.SetStateAction<string>>
}
> = ({ options, setState }) => {
    const [selected, setSelected] = useState(options[0]);

    return (
        <View style={Style.segmentedControl}>
            {
                options.map((option, index) => (
                    <TouchableOpacity 
                        key={index}
                        onPress={() => {
                            setSelected(option);
                            setState(option);
                        }}
                        style={{
                            flex: 1,
                            justifyContent: 'center'
                        }}
                    >
                        {
                            selected === option ? (
                                <Text 
                                    style={{ 
                                        fontSize: 18, 
                                        fontWeight: '600',
                                        backgroundColor: '#ffffff',
                                        borderRadius: 10,
                                        overflow: 'hidden',
                                        textAlign: 'center',
                                        padding: 5
                                    }}
                                >
                                    {option}
                                </Text>
                            ) : (
                                <Text style={{ fontSize: 18, fontWeight: '600', textAlign: 'center' }}>{option}</Text>
                            )
                        }
                    </TouchableOpacity>
                ))
            }
        </View>
    );
}

export default SegmentedControlTemplate;