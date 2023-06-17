import React, { useEffect } from 'react';
import { GroupMemberDisplay } from '../../components/display/GroupMemberDisplay';
import { useSelector } from 'react-redux';
import { selectCurrentGroup } from '../../redux/slices/groupSlice';
import { useLocalSearchParams, useNavigation } from 'expo-router';

export default function MemberDisplay() {
    const curGroup = useSelector(selectCurrentGroup);
    const { user_uuid } = useLocalSearchParams();
    const navigation = useNavigation();

    useEffect(() => {
        navigation.setOptions({
            title: curMember?.name,
        });
    }, []);
    
    const curMember = curGroup?.members.find((m) => m.user_uuid === user_uuid);

    if (!curMember) return null;

    return (
        <GroupMemberDisplay member={curMember} />
    );
}