import React from 'react';
import { 
    View, 
    Image, 
    Text, 
    ImageSourcePropType 
} from 'react-native';

const TabIcon: React.FC<{
    name: string;
    image: ImageSourcePropType;
    size: number;
    color: string;
    focused: boolean
}> = ({ name, image, size, color, focused }) => (
    <View style={{alignItems: 'center', justifyContent: 'center', top: 10} }>
        <Image
            source={image}
            resizeMode='contain'
            style={{
                width: size,
                height: size,
                tintColor: focused ? color : '#748c94'
            }}
        />
        <Text
            style={{color: focused ? color : '#748c94', fontSize: 12 }}
        >
            {name}
        </Text>
    </View>
);

export default TabIcon;
