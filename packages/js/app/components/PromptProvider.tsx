import { BackHandler, Platform, StyleSheet, View } from "react-native";
import prompt, { PromptOptions } from "react-native-prompt-android";
import Dialog from 'react-native-dialog';
import React, { useEffect, useState } from "react";


let androidPromptRef: (title: string, message: string, buttons: Array<PromptButton>, options: PromptOptions) => void;
let promptLastCancelled = 0;

type PromptButton = {
    text?: string;
    onPress?: (message: string) => void;

    /** @platform ios */
    style?: 'default' | 'cancel' | 'destructive';
};

export default function compatiblePrompt(
    title?: string,
    message?: string,
    callbackOrButtons?: ((value: string) => void) | Array<PromptButton>,
    options?: PromptOptions,
) {
    if (!options) {
        options = { type: "default" };
    }
    if (Platform.OS === "android") {
        androidPromptRef(title || "", message || "", callbackOrButtons as Array<PromptButton>, options);
        return;
    }
    prompt(title, message, callbackOrButtons, options);
};

export function alertPrompt(title: string, message?: string, onClose?: () => void) {
    compatiblePrompt(title, message, [
        {
            text: "OK",
            onPress: onClose,
        }
    ]);
}

export function confirmPrompt(title: string, message?: string, onConfirm?: () => void, onCancel?: () => void) {
    compatiblePrompt(title, message, [
        {
            text: "Cancel",
            onPress: onCancel,
        },
        {
            text: "OK",
            onPress: onConfirm,
        }
    ]);
}

export function confirmPromptDestructive(title: string, message?: string, onConfirm?: () => void, onCancel?: () => void, customLeaveText?: string) {
    compatiblePrompt(title, message, [
        {
            text: "Cancel",
            onPress: onCancel,
            style: "cancel",
        },
        {
            text: customLeaveText ?? "Discard",
            onPress: onConfirm,
            style: 'destructive',
        }
    ]);
}

export function AndroidPromptProvider() {
    const [visible, setVisible] = useState(false);
    const [title, setTitle] = useState("");
    const [message, setMessage] = useState("");
    const [buttons, setButtons] = useState<PromptButton[]>([]);
    const [options, setOptions] = useState<PromptOptions>({ type: "default" });
    const [input, setInput] = useState("");

    androidPromptRef = (title: string, message: string, buttons: Array<PromptButton>, options: PromptOptions) => {
        const f = () => {
            setTitle(title);
            setMessage(message);
            if (buttons) {
                setButtons(buttons);
            } else {
                setButtons([
                    {
                        style: "cancel",
                        text: "OK",
                    }
                ]);
            }
            setOptions(options);
            setInput("");
            setVisible(true);
        };
        if (Date.now() - promptLastCancelled < 500) {
            setTimeout(f, 500);
        } else {
            f();
        }
    }

    const handleCancel = () => {
        hide();
    }

    const hide = () => {
        promptLastCancelled = Date.now();
        setVisible(false);
    }

    if (Platform.OS !== "android") {
        return null;
    }

    return (
        <View style={[StyleSheet.absoluteFill, styles.container]}>
            <Dialog.Container visible={visible} onRequestClose={() => setVisible(false)}>
                <Dialog.Title>{title}</Dialog.Title>
                {message ? (
                    <Dialog.Description>
                        {message}
                    </Dialog.Description>
                ) : null}
                {options.type === "plain-text" && <Dialog.Input onChangeText={(t) => setInput(t)} autoFocus />}
                {buttons.map((button, index) => {
                    if (button.style === "cancel") {
                        return <Dialog.Button key={index} label={button.text} onPress={handleCancel} />
                    }
                    return <Dialog.Button key={index} label={button.text} onPress={() => {
                        hide();
                        if (button.onPress) {
                            button.onPress(input);
                        }
                    }} />
                })
                }
            </Dialog.Container>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: "#fff",
        alignItems: "center",
        justifyContent: "center",
    }
})