import React, { useContext, useEffect, useState } from 'react';
import {
    StyleSheet,
    View,
    TextInput as RNTextInput,
    TextInputProps as RNTextInputProps,
    TouchableOpacity,
    StyleProp,
    ViewStyle,
    TextStyle,
} from 'react-native';
import Animated, {
    useAnimatedStyle,
    useSharedValue,
    withDelay,
    withTiming,
} from 'react-native-reanimated';
import { Feather, FontAwesome } from '@expo/vector-icons';
import { layoutDefaults } from '../constants/Layout';
import { ThemeContext } from '../lib/Context';
import StyledText from './StyledText';
import { Loading } from './display/Loading';

interface TextInputProps {
    value: string;
    onChange: (value: string) => void;
    label: string;
    viewStyle?: StyleProp<ViewStyle>;
    showSubmit?: boolean;
    onSubmit?: () => void;
    submitLoading?: boolean;
    disableEdit?: boolean;
    onPress?: () => void;
    secureTextEntry?: boolean;
    textContentType?:
    | 'none'
    | 'URL'
    | 'addressCity'
    | 'addressCityAndState'
    | 'addressState'
    | 'countryName'
    | 'creditCardNumber'
    | 'emailAddress'
    | 'familyName'
    | 'fullStreetAddress'
    | 'givenName'
    | 'jobTitle'
    | 'location'
    | 'middleName'
    | 'name'
    | 'namePrefix'
    | 'nameSuffix'
    | 'nickname'
    | 'organizationName'
    | 'postalCode'
    | 'streetAddressLine1'
    | 'streetAddressLine2'
    | 'sublocality'
    | 'telephoneNumber'
    | 'username'
    | 'password'
    | 'newPassword'
    | 'oneTimeCode'
    | undefined;
}
export function StyledInput(props: TextInputProps) {
    const {
        value,
        onChange,
        label,
        viewStyle,
        showSubmit,
        onSubmit,
        submitLoading,
        disableEdit,
        onPress,
        textContentType,
        secureTextEntry,
    } = props;
    const theme = useContext(ThemeContext);
    const [focused, setFocused] = useState(false);

    const labelScale = useSharedValue(0.8);
    const labelOpacity = useSharedValue(0);
    const animatedLabelStyle = useAnimatedStyle(() => ({
        opacity: labelOpacity.value,
        transform: [{ scale: labelScale.value }],
    }));

    const textFieldTranslateY = useSharedValue(0);
    const animatedTextFieldStyle = useAnimatedStyle(() => ({
        transform: [{ translateY: textFieldTranslateY.value }],
    }));

    useEffect(() => {
        if (value.length) {
            labelScale.value = withTiming(1);
            labelOpacity.value = withDelay(50, withTiming(1, { duration: 100 }));
            textFieldTranslateY.value = withTiming(8);
        } else {
            labelScale.value = withTiming(0.95);
            labelOpacity.value = withTiming(0);
            textFieldTranslateY.value = withTiming(0);
        }
    }, [focused, value]);

    const displaySubmit = showSubmit && value.length > 0;

    return (
        <View
            style={[
                {
                    backgroundColor: theme.colors.primaryPaper,
                    height: 70,
                    borderRadius: layoutDefaults.paperBorderRadius,
                    borderColor: theme.colors.secondaryPaper,
                    borderWidth: 1,
                },
                viewStyle,
            ]}
        >
            <Animated.View style={animatedLabelStyle}>
                <StyledText
                    variant="mini"
                    style={{
                        marginLeft: 22,
                        color: theme.colors.secondaryPaper,
                        position: "absolute"
                    }}
                >
                    {label}
                </StyledText>
            </Animated.View>
            <Animated.View style={[
                {
                    position: 'relative',
                    height: "100%",
                    display: 'flex',
                    justifyContent: 'center',
                },
                animatedTextFieldStyle]}>
                <RNTextInput
                    style={[
                        styles.input,
                        showSubmit
                            ? {
                                paddingRight: 50,
                            }
                            : undefined,
                    ]}
                    placeholder={label}
                    onFocus={() => setFocused(true)}
                    onBlur={() => setFocused(false)}
                    value={value}
                    onChangeText={onChange}
                    editable={!submitLoading && !disableEdit && !onPress}
                    onPressOut={onPress}
                    textContentType={textContentType}
                    secureTextEntry={secureTextEntry}
                />
            </Animated.View>
            {displaySubmit && (
                <TouchableOpacity
                    style={{
                        position: 'absolute',
                        right: 20,
                        height: '100%',
                        display: 'flex',
                        justifyContent: 'center',
                    }}
                    onPress={onSubmit}
                >
                    {submitLoading ? (
                        <Loading />
                    ) : (
                        <Feather name="send" size={24} color="black" />
                    )}
                </TouchableOpacity>
            )}
            {disableEdit && !onPress && (
                <View
                    style={{
                        position: 'absolute',
                        right: 20,
                        height: '100%',
                        display: 'flex',
                        justifyContent: 'center',
                    }}
                >
                    <FontAwesome name="lock" size={24} color="black" />
                </View>
            )}
        </View>
    );
}

interface BasicTextInputProps extends RNTextInputProps {
    style?: StyleProp<TextStyle>;
}
export function BasicTextInput(props: BasicTextInputProps) {
    const { style } = props;

    return (
        <RNTextInput
            {...props}
            style={[styles.input, style]}
            textAlignVertical="top"
        />
    );
}

const styles = StyleSheet.create({
    input: {
        height: 40,
        marginLeft: 12,
        padding: 10,
        // fontFamily: 'sf-pro-rounded-regular',
        fontSize: 18,
        fontWeight: '400',
    },
});
