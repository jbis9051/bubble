import { StatusBar } from 'expo-status-bar';
import { FlatList, Platform, ScrollView, StyleSheet, TouchableOpacity } from 'react-native';
import { Text, View } from '../../components/Themed';
import StyledText from '../../components/StyledText';
import { useNavigation } from 'expo-router';
import { useContext, useEffect, useState } from 'react';
import { Ionicons } from '@expo/vector-icons';
import StyledButton from '../../components/bubbleUI/Button';
import { useDispatch, useSelector } from 'react-redux';
import { selectCurrentGroup, setGroups } from '../../redux/slices/groupSlice';
import { GroupService } from '../../lib/bubbleApi/group';
import { UserLocal } from '../../lib/bubbleApi/user';
import Avatar from '../../components/Avatar';
import { ThemeContext } from '../../lib/Context';
import { confirmPromptDestructive } from '../../components/PromptProvider';
import { LoggingService } from '../../lib/bubbleApi/logging';

interface BubbleMemberProps {
  member: UserLocal;
}
function BubbleMember({
  member,
}: BubbleMemberProps) {
  const { name, user_uuid } = member;
  const navigation = useNavigation();

  const handlePress = () => {
    // @ts-ignore
    navigation.navigate('groupSettingsModal', { screen: "memberDisplay", params: { user_uuid } });
  }

  return (
    <TouchableOpacity
      style={{
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        padding: 15,
      }}
      onPress={handlePress}
    >
      <Avatar
        name={name}
        width={50}
      />
      <StyledText nomargin style={{ textAlign: "center", flex: 1 }}>{name}</StyledText>
    </TouchableOpacity>
  );
}

export default function GroupSettingsModal() {
  const curGroup = useSelector(selectCurrentGroup);
  const navigation = useNavigation();
  const theme = useContext(ThemeContext);
  const dispatch = useDispatch();

  const [leaving, setLeaving] = useState(false);

  useEffect(() => {
    navigation.setOptions({
      headerRight: () => (
        <TouchableOpacity onPress={() => {
          // @ts-ignore
          navigation.navigate('groupSettingsModal', { screen: "shareBubble" });
        }}>
          <Ionicons name="ios-add-sharp" size={24} color="black" />
        </TouchableOpacity>
      )
    });
  }, []);

  const handleLeaveBubble = () => {
    if (!curGroup) return null;
    confirmPromptDestructive(`Leave '${curGroup.name}'?`, "You will need to be re-invited to join back.", () => {
      setLeaving(true);
      GroupService
        .leave_group(curGroup.uuid)
        .then(() => {
          GroupService
            .get_groups()
            .then(groups => {
              dispatch(setGroups(groups));
              navigation.goBack();
              setLeaving(false);              
            })
            .catch(LoggingService.error);
        })
        .catch(LoggingService.error);
    }, undefined, "Leave")
  };

  if (!curGroup) return null;

  return (
    <View style={styles.container}>
      <ScrollView contentContainerStyle={{ height: "100%" }}>
        <StyledText nomargin style={{ marginBottom: 15 }}>Bubble Members</StyledText>
        {curGroup.members.map((m, idx) => {
          return (
            <View
              key={idx}
              style={{
                borderTopColor: theme.colors.secondaryPaper,
                borderBottomColor: theme.colors.secondaryPaper,
                borderTopWidth: idx === 0 ? 1 : 0,
                borderBottomWidth: 1,
              }}
            >
              <BubbleMember
                member={m}
              />
            </View>
          );
        })}
        <StyledButton
          color="danger"
          variant="outlined"
          onPress={handleLeaveBubble}
          style={{ marginBottom: 15, marginTop: "auto" }}
        >Leave Bubble</StyledButton>
      </ScrollView>
      <StatusBar style={Platform.OS === 'ios' ? 'light' : 'auto'} />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 15,
  },
});