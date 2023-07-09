import { createEntityAdapter, createSlice } from '@reduxjs/toolkit';
import { UserLocal } from '../../lib/bubbleApi/user';
import { Group, uuid } from '../../lib/bubbleApi/group';
import { RootState } from '../store';

type GroupStateStatus = 'idle' | 'pending' | 'fulfilled' | 'rejected';

const groupsAdapter = createEntityAdapter<Group>({
    selectId: (group) => group.uuid,
});

export const authSlice = createSlice({
    name: 'group',
    initialState: groupsAdapter.getInitialState<{
        status: GroupStateStatus;
        activeGroupUuid: uuid;
    }>({
        status: 'idle',
        activeGroupUuid: '',
    }),
    reducers: {
        setGroups: (state, action) => {
            state.status = 'fulfilled';
            if (
                action.payload.length > 0 &&
                (!state.activeGroupUuid ||
                    !state.ids.includes(state.activeGroupUuid))
            ) {
                state.activeGroupUuid = action.payload[0].uuid;
            }
            groupsAdapter.setAll(state, action.payload);
        },
        setActiveGroup: (state, action) => {
            console.log(state);
            state.activeGroupUuid = action.payload;
        },
    },
});

export const { setGroups, setActiveGroup } = authSlice.actions;

const groupsSelector = groupsAdapter.getSelectors<RootState>(
    (state) => state.groups
);

export const selectGroups = groupsSelector.selectAll;
export const selectGroupById = (state: RootState, id: uuid) =>
    groupsSelector.selectById(state, id);
export const selectCurrentGroup = (state: RootState) =>
    groupsSelector.selectById(state, state.groups.activeGroupUuid);

export default authSlice.reducer;
