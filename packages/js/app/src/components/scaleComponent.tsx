import { Dimensions } from 'react-native';

const iphone13LoginWidth = 390;
const iphone13LoginHeight = 844;

const scalePhone = (unscaled: number, height: boolean) => {
    if (height) {
        return (
            (unscaled * Dimensions.get('window').height) / iphone13LoginHeight
        );
    }
    return (unscaled * Dimensions.get('window').width) / iphone13LoginWidth;
};

export default scalePhone;
