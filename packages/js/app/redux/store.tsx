import { configureStore } from '@reduxjs/toolkit';

import authReducer from './slices/authSlice';
import groupsReducer from './slices/groupSlice';

export interface RootState {
    auth: ReturnType<typeof authReducer>;
    groups: ReturnType<typeof groupsReducer>;
}

export default configureStore<RootState>({
    reducer: {
        auth: authReducer,
        groups: groupsReducer,
    },
});
