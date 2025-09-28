import { configureStore, combineReducers } from "@reduxjs/toolkit";
import { CardSlice } from "./CardSlice";
import { DeckSlice } from "./DeckSlice";
import blockchainReducer from "./blockchainSlice";

import AsyncStorage from "@react-native-async-storage/async-storage";

import {
  persistReducer,
  persistStore,
  FLUSH,
  REHYDRATE,
  PAUSE,
  PERSIST,
  PURGE,
  REGISTER,
} from "redux-persist";

const persistConfig = {
  key: "root",
  storage: AsyncStorage,
  blacklist: ["card", "blockchain"],
};
const rootReducer = combineReducers({
  card: CardSlice.reducer,
  deck: DeckSlice.reducer,
  blockchain: blockchainReducer,
});

const persistedReducer = persistReducer(persistConfig, rootReducer);

export const store = configureStore({
  reducer: persistedReducer,
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: {
        ignoredActions: [FLUSH, REHYDRATE, PAUSE, PERSIST, PURGE, REGISTER],
      },
    }),
});

export const persistor = persistStore(store);
