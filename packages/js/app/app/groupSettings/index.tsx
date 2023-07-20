import { StatusBar } from 'expo-status-bar';
import {
    Alert,
    Platform,
    ScrollView,
    StyleSheet,
    TouchableOpacity,
    View,
} from 'react-native';
import { useNavigation } from 'expo-router';
import { useEffect, useState } from 'react';
import { Ionicons } from '@expo/vector-icons';
import { observer } from 'mobx-react-lite';
import { UserOut } from '@bubble/react-native-bubble-rust';
import StyledButton from '../../components/bubbleUI/Button';
import StyledText from '../../components/StyledText';
import Avatar from '../../components/Avatar';
import Colors from '../../constants/Colors';
import MainStore from '../../stores/MainStore';
import FrontendInstanceStore from '../../stores/FrontendInstanceStore';

interface BubbleMemberProps {
    member: UserOut;
    OnPress?: () => void;
}

export function BubbleMember({ member, OnPress }: BubbleMemberProps) {
    return (
        <TouchableOpacity
            style={{
                display: 'flex',
                flexDirection: 'row',
                alignItems: 'center',
                padding: 15,
                height: 100,
            }}
            onPress={OnPress}
        >
            <Avatar name={member.name} width={50} />
            <View
                style={{
                    flex: 1,
                    display: 'flex',
                    marginLeft: 10,
                    alignItems: 'flex-start',
                    justifyContent: 'center',
                    height: '100%',
                }}
            >
                <StyledText nomargin>{member.name}</StyledText>
                <StyledText nomargin style={{ fontSize: 14 }}>
                    {member.username}
                </StyledText>
            </View>
        </TouchableOpacity>
    );
}

const GroupSettings = observer(() => {
    const navigation = useNavigation();

    const [leaving, setLeaving] = useState(false);

    useEffect(() => {
        navigation.setOptions({
            headerRight: () => (
                <>
                    <TouchableOpacity
                        style={{ marginRight: 15 }}
                        onPress={() => {
                            Alert.alert('Are you sure you want to logout?', '', [
                                {
                                    text: 'Cancel',
                                    style: 'cancel',
                                },
                                {
                                    text: 'OK',
                                    style: 'destructive',
                                    onPress: () => {
                                        FrontendInstanceStore.instance
                                            .logout()
                                            .then(async() => {
                                                MainStore.current_group = null;
                                                MainStore.groups = [];
                                                MainStore.status = await FrontendInstanceStore.instance.status();
                                            })
                                            .catch((err) => {
                                                Alert.alert('Error', err.message);
                                            });
                                    },
                                },
                            ]);
                        }}
                    >
                        <Ionicons name={"log-out"} size={24} color={"black"}/>
                    </TouchableOpacity>
                <TouchableOpacity
                    onPress={() => {
                        // @ts-ignore
                        navigation.navigate('groupSettings', {
                            screen: 'shareBubble',
                        });
                    }}
                >
                    <Ionicons name="ios-add-sharp" size={24} color="black" />
                </TouchableOpacity>
                </>
            ),
        });
    }, []);

    const handleLeaveBubble = () => {
        if (!MainStore.current_group) {
            return null;
        }
        Alert.alert(
            `Leave '${MainStore.current_group.name}'?`,
            'You will need to be re-invited to join back.',
            [
                {
                    text: 'OK',
                    style: 'destructive',
                    onPress: () => {
                        if (!MainStore.current_group) {
                            return;
                        }
                        setLeaving(true);
                        FrontendInstanceStore.instance
                            .leave_group(MainStore.current_group.uuid)
                            .then(() =>
                                FrontendInstanceStore.instance.get_groups()
                            )
                            .then((groups) => {
                                MainStore.groups = groups;
                                MainStore.current_group =
                                    MainStore.groups.length > 0
                                        ? MainStore.groups[0]
                                        : null;
                                navigation.goBack();
                            })
                            .catch((err) => {
                                Alert.alert('Error', err.message);
                            })
                            .finally(() => setLeaving(false));
                    },
                },
                {
                    text: 'Cancel',
                    style: 'cancel',
                },
            ]
        );
    };

    if (!MainStore.current_group) {
        return null;
    }

    return (
        <View style={styles.container}>
            <ScrollView contentContainerStyle={{ height: '100%' }}>
                <StyledText nomargin style={{ marginBottom: 15 }}>
                    Bubble Members
                </StyledText>
                {Object.entries(MainStore.current_group.members).map(
                    ([user_uuid, info], idx) => (
                        <View
                            key={idx}
                            style={{
                                borderTopColor: Colors.colors.secondaryPaper,
                                borderBottomColor: Colors.colors.secondaryPaper,
                                borderTopWidth: idx === 0 ? 1 : 0,
                                borderBottomWidth: 1,
                            }}
                        >
                            <BubbleMember
                                OnPress={() => {
                                    // @ts-ignore
                                    navigation.navigate('groupSettings', {
                                        screen: 'memberDisplay',
                                        params: { user_uuid },
                                    });
                                }}
                                member={info.info}
                            />
                        </View>
                    )
                )}
                <StyledButton
                    color="danger"
                    variant="outlined"
                    onPress={handleLeaveBubble}
                    style={{ marginBottom: 15, marginTop: 'auto' }}
                >
                    Leave Bubble
                </StyledButton>
            </ScrollView>
            <StatusBar style={Platform.OS === 'ios' ? 'light' : 'auto'} />
        </View>
    );
});
export default GroupSettings;

const styles = StyleSheet.create({
    container: {
        flex: 1,
        padding: 15,
    },
});
