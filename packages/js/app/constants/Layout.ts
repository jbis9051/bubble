import { Dimensions } from 'react-native';

const width = Dimensions.get('window').width;
const height = Dimensions.get('window').height;

export const layoutDefaults = {
    gutters: 15,
    paperBorderRadius: 12,
    imageBorderRadius: 15,
    profileContentBorderRadius: 15,
    viewPaddingTop: 90,
    headerToContentSpacing: 20,
};

export default {
    window: {
        width,
        height,
    },
    isSmallDevice: width < 375,
    defaults: layoutDefaults,
};

export const calculateImageWidth = (
    imageColCount: number,
    gutterWidth: number,
    spacing: number
) => {
    const totalGutterWidth = gutterWidth * (imageColCount - 1);
    const totalSpacing = spacing * 2;
    return (width - totalGutterWidth - totalSpacing) / imageColCount;
};
