import { configureStore } from "@reduxjs/toolkit";

import authReducer from "./slices/authSlice";

export interface RootState {
    auth: ReturnType<typeof authReducer>;
}

export default configureStore<RootState>({
    reducer: {
        auth: authReducer,
    }
});