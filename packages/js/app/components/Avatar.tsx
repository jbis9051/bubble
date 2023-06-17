import React, { useContext } from "react";
import { View } from "./Themed";
import StyledText, { CustomTextProps } from "./StyledText";
import { ThemeContext } from "../lib/Context";

interface AvatarProps {
    name: string;
    width: string | number;
    textVariant?: CustomTextProps['variant'];
}
export default function Avatar({ name, width, textVariant }: AvatarProps) {
    const initials = name.split(" ").map((n) => n[0]).join("");
    const theme = useContext(ThemeContext);

    return (
        <View style={{
            width,
            aspectRatio: 1,
            borderRadius: 9999,
            backgroundColor: theme.colors.secondaryPaper,
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
        }}>
            <StyledText nomargin style={{ color: "white" }} variant={textVariant}>{initials}</StyledText>
        </View>
    );
}