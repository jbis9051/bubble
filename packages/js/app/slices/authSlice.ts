import { createSlice } from '@reduxjs/toolkit';

export const authSlice = createSlice({
    name: 'auth',
    initialState: {
        status: 'idle',
        user: null,
    },
    reducers: {
        setAuth: (state, action) => {
            // eslint-disable-next-line no-param-reassign
            state.status = 'fulfilled';
            // eslint-disable-next-line no-param-reassign
            state.user = action.payload;
        },
    },
});

export interface AuthUser {
    displayName: string;
    email: string;
    photoURL: string;
    uid: string;
    onboarded: boolean;
}

export const { setAuth } = authSlice.actions;

export const selectUser = (state: any): AuthUser => state.auth.user;

export default authSlice.reducer;
