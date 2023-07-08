import { StyleProp, TextStyle } from 'react-native';
import { Text as BaseText, TextProps } from './Themed';

const baseTextStyle: StyleProp<TextStyle> = {
    fontFamily: 'System',
};

export interface CustomTextProps {
    variant?: 'h1' | 'h2' | 'body' | 'mini';

    nomargin?: boolean;
}

// ex: "Hi <name>!"
function Heading1(props: TextProps) {
    return (
        <BaseText
            {...props}
            style={[
                {
                    fontSize: 36,
                    fontWeight: '500',
                    marginTop: 15,
                    marginLeft: 15,
                    lineHeight: 43,
                },
                props.style,
                baseTextStyle,
            ]}
        />
    );
}

function Heading2(props: TextProps) {
    return (
        <BaseText
            {...props}
            style={[
                {
                    fontSize: 24,
                    fontWeight: '500',
                    marginTop: 15,
                    marginLeft: 15,
                    lineHeight: 29,
                },
                props.style,
                baseTextStyle,
            ]}
        />
    );
}

function Body(props: TextProps) {
    return (
        <BaseText
            {...props}
            style={[
                {
                    fontSize: 18,
                    fontWeight: '400',
                    marginTop: 15,
                    marginLeft: 15,
                    lineHeight: 21,
                },
                props.style,
                baseTextStyle,
            ]}
        />
    );
}

function Mini(props: TextProps) {
    return (
        <BaseText
            {...props}
            style={[
                {
                    fontSize: 12,
                    fontWeight: '400',
                    marginTop: 15,
                    marginLeft: 15,
                    lineHeight: 14,
                },
                props.style,
                baseTextStyle,
            ]}
        />
    );
}

export default function StyledText(props: TextProps & CustomTextProps) {
    const { variant } = props;

    let usedProps = { ...props };

    if (props.nomargin) {
        usedProps = {
            ...props,
            style: {
                marginTop: 0,
                marginLeft: 0,
                // @ts-ignore
                ...props.style,
            },
        };
    }

    switch (variant) {
        case 'h1':
            return <Heading1 {...usedProps} />;
        case 'h2':
            return <Heading2 {...usedProps} />;
        case 'body':
            return <Body {...usedProps} />;
        case 'mini':
            return <Mini {...usedProps} />;
        default:
            return <Body {...usedProps} />;
    }
}
