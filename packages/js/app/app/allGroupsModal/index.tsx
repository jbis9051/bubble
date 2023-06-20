import { StatusBar } from 'expo-status-bar';
import { FlatList, Platform, StyleSheet, TouchableOpacity } from 'react-native';
import { Text, View } from '../../components/Themed';
import { useDispatch, useSelector } from 'react-redux';
import { selectCurrentGroup, selectGroups, setActiveGroup } from '../../redux/slices/groupSlice';
import { useContext, useEffect } from 'react';
import { Group } from '../../lib/bubbleApi/group';
import { Ionicons, MaterialCommunityIcons } from '@expo/vector-icons';
import { ThemeContext } from '../../lib/Context';
import StyledText from '../../components/StyledText';
import { useNavigation } from 'expo-router';
import { summarizeNames } from '../../lib/formatText';

function BubbleDisplay({
  group,
}: {
  group: Group;
}) {
  const dispatch = useDispatch();
  const navigation = useNavigation();
  const theme = useContext(ThemeContext);

  const handleSetActive = () => {
    navigation.goBack();
    dispatch(setActiveGroup(group.uuid));
  }


  return (
    <TouchableOpacity
      style={{
        display: 'flex',
        flexDirection: 'row',
        alignItems: 'center',
        justifyContent: 'flex-start',
        padding: "6%",
      }}
      onPress={handleSetActive}
    >
      <MaterialCommunityIcons
        name="chart-bubble"
        size={48}
        color="black"
      />
      <View style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'flex-start',
        justifyContent: 'center',
        paddingHorizontal: 45,
      }}>
        <StyledText nomargin variant='h2' numberOfLines={1}>{group.name}</StyledText>
        {(group.members.length) ? <StyledText nomargin variant='body'>{summarizeNames(group.members.map(m => m.name))}</StyledText> : null}
      </View>
    </TouchableOpacity>
  );
}

export default function BubbleListModal() {
  const groups = useSelector(selectGroups);
  const theme = useContext(ThemeContext);

  const navigation = useNavigation();

  useEffect(() => {
    navigation.setOptions({
      headerRight: () => (
        <TouchableOpacity onPress={() => {
          // @ts-ignore
          navigation.navigate('allGroupsModal', { screen: 'newGroup' });
        }}>
          <Ionicons name="ios-add-sharp" size={24} color="black" />
        </TouchableOpacity>
      )
    })
  }, []);

  return (
    <View style={styles.container}>
      <FlatList
        data={groups}
        renderItem={({ item, index }) => (
          <View style={[(index % 2 === (groups.length % 2 === 1 ? 0 : 1)) ? ({
            borderTopColor: theme.colors.secondaryPaper,
            borderTopWidth: 1,
            borderBottomColor: theme.colors.secondaryPaper,
            borderBottomWidth: 1,
            borderStyle: "solid",
          }) : undefined]}>
            <BubbleDisplay group={item} />
          </View>
        )}
      />
      <StatusBar style={Platform.OS === 'ios' ? 'light' : 'auto'} />
    </View>
  );
}

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