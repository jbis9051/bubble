import { StyleProp, Text, TextProps, TextStyle } from 'react-native';

const baseTextStyle: StyleProp<TextStyle> = {
    fontFamily: 'System',
};

export interface CustomTextProps {
    variant?: 'h1' | 'h2' | 'body' | 'mini';

    nomargin?: boolean;
}

const DEFAULT_MARGIN = 15;

function Heading1(props: TextProps) {
    return (
        <Text
            {...props}
            style={[
                {
                    fontSize: 36,
                    fontWeight: '500',
                    marginTop: DEFAULT_MARGIN,
                    marginLeft: DEFAULT_MARGIN,
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
        <Text
            {...props}
            style={[
                {
                    fontSize: 24,
                    fontWeight: '500',
                    marginTop: DEFAULT_MARGIN,
                    marginLeft: DEFAULT_MARGIN,
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
        <Text
            {...props}
            style={[
                {
                    fontSize: 18,
                    fontWeight: '400',
                    marginTop: DEFAULT_MARGIN,
                    marginLeft: DEFAULT_MARGIN,
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
        <Text
            {...props}
            style={[
                {
                    fontSize: 12,
                    fontWeight: '400',
                    marginTop: DEFAULT_MARGIN,
                    marginLeft: DEFAULT_MARGIN,
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
