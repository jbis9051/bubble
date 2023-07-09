import { createSlice } from '@reduxjs/toolkit';
import { UserLocal } from '../../lib/bubbleApi/user';

export const authSlice = createSlice({
    name: 'auth',
    initialState: {
        status: 'idle',
        user: null,
    },
    reducers: {
        setAuth: (state, action) => {
            state.status = 'fulfilled';
            state.user = action.payload;
        },
    },
});

export const { setAuth } = authSlice.actions;

export const selectUser = (state: any): UserLocal => state.auth.user;

export default authSlice.reducer;
