import React, { useContext, useEffect, useState } from 'react';
import { StyleSheet, View, TextInput as RNTextInput, TextInputProps as RNTextInputProps, TouchableOpacity, Image, ImageBase, ScrollView, StyleProp, ViewStyle, TextStyle } from 'react-native';
import Animated, { useAnimatedStyle, useSharedValue, withTiming } from 'react-native-reanimated';
import { layoutDefaults } from '../constants/Layout';
import { ThemeContext } from '../lib/Context';
import StyledText from './StyledText';
import * as ImagePicker from 'expo-image-picker';
import { AntDesign, Feather, FontAwesome } from '@expo/vector-icons';
import { MaterialCommunityIcons } from '@expo/vector-icons';
import Layout from '../constants/Layout';
import { Loading } from './display/Loading';
import { ImageDisplay } from './display/ImageDisplay';
import * as Haptics from 'expo-haptics';

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

    const labelScale = useSharedValue(0);
    const labelHeight = useSharedValue(0);
    const animatedLabelStyle = useAnimatedStyle(() => {
        return {
            height: labelHeight.value,
            transform: [{ scale: labelScale.value }]
        }
    });

    const textFieldMarginTop = useSharedValue(12);
    const animatedTextFieldStyle = useAnimatedStyle(() => {
        return {
            marginTop: textFieldMarginTop.value,
        }
    });

    useEffect(() => {
        if (focused && value.length) {
            labelScale.value = withTiming(1);
            labelHeight.value = withTiming(12);
            textFieldMarginTop.value = withTiming(0);
        } else {
            labelScale.value = withTiming(0);
            labelHeight.value = withTiming(0);
            textFieldMarginTop.value = withTiming(12);
        }
    }, [focused, value]);

    const displaySubmit = showSubmit && value.length > 0;

    return (
        <View style={[{
            backgroundColor: theme.colors.primaryPaper,
            height: 70,
            borderRadius: layoutDefaults.paperBorderRadius
        }, viewStyle]}>
            <Animated.View style={animatedLabelStyle}>
                <StyledText variant="mini" style={{
                    marginLeft: 20,
                    color: theme.colors.secondaryPaper,
                }}>{label}</StyledText>
            </Animated.View>
            <Animated.View style={animatedTextFieldStyle}>
                <RNTextInput
                    style={[
                        styles.input,
                        showSubmit ? {
                            paddingRight: 50
                        } : undefined,
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
                        position: "absolute",
                        right: 20,
                        height: "100%",
                        display: "flex",
                        justifyContent: "center",
                    }}
                    onPress={onSubmit}
                >
                    {submitLoading ? <Loading /> : <Feather name="send" size={24} color="black" />}
                </TouchableOpacity>
            )}
            {(disableEdit && !onPress) && (
                <View
                    style={{
                        position: "absolute",
                        right: 20,
                        height: "100%",
                        display: "flex",
                        justifyContent: "center",
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
    const {
        style
    } = props;

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