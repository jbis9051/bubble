import { StatusBar } from 'expo-status-bar';
import { FlatList, Platform, StyleSheet, TouchableOpacity, View } from 'react-native';
import { useEffect } from 'react';
import { Ionicons, MaterialCommunityIcons } from '@expo/vector-icons';
import { useNavigation } from 'expo-router';
import StyledText from '../../components/StyledText';
import { summarizeNames } from '../../lib/formatText';
import Colors from "../../constants/Colors";
import MainStore from "../../stores/MainStore";
import {Group} from '@bubble/react-native-bubble-rust';
import {observer} from "mobx-react-lite";

function BubbleDisplay({ group }: { group: Group }) {
    const navigation = useNavigation();

    const handleSetActive = () => {
        navigation.goBack();
        MainStore.current_group = group;
    };

    return (
        <TouchableOpacity
            style={{
                display: 'flex',
                flexDirection: 'row',
                alignItems: 'center',
                justifyContent: 'flex-start',
                padding: '6%',
            }}
            onPress={handleSetActive}
        >
            <MaterialCommunityIcons
                name="chart-bubble"
                size={48}
                color="black"
            />
            <View
                style={{
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'flex-start',
                    justifyContent: 'center',
                    paddingHorizontal: 45,
                }}
            >
                <StyledText nomargin variant="h2" numberOfLines={1}>
                    {group.name}
                </StyledText>
                {Object.entries(group.members).length > 0 ? (
                    <StyledText nomargin variant="body">
                        {summarizeNames(Object.entries(group.members).map(([user_uuid, info]) => info.info.name))}
                    </StyledText>
                ) : null}
            </View>
        </TouchableOpacity>
    );
}

 const Groups = observer(() => {
    const navigation = useNavigation();

    useEffect(() => {
        navigation.setOptions({
            headerRight: () => (
                <TouchableOpacity
                    onPress={() => {
                        // @ts-ignore
                        navigation.navigate('groups', {
                            screen: 'newGroup',
                        });
                    }}
                >
                    <Ionicons name="ios-add-sharp" size={24} color="black" />
                </TouchableOpacity>
            ),
        });
    }, []);

    return (
        <View style={styles.container}>
            <FlatList
                data={MainStore.groups}
                renderItem={({ item, index }) => (
                    <View
                        style={[
                            index % 2 === (MainStore.groups.length % 2 === 1 ? 0 : 1)
                                ? {
                                      borderTopColor:
                                          Colors.colors.secondaryPaper,
                                      borderTopWidth: 1,
                                      borderBottomColor:
                                          Colors.colors.secondaryPaper,
                                      borderBottomWidth: 1,
                                      borderStyle: 'solid',
                                  }
                                : undefined,
                        ]}
                    >
                        <BubbleDisplay group={item} />
                    </View>
                )}
            />
            <StatusBar style={Platform.OS === 'ios' ? 'light' : 'auto'} />
        </View>
    );
});
export default Groups;

const styles = StyleSheet.create({
    container: {
        flex: 1,
    },
    title: {
        fontSize: 20,
        fontWeight: 'bold',
    },
    separator: {
        marginVertical: 30,
        height: 1,
        width: '80%',
    },
});
