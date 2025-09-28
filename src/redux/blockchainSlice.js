import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import blockchainService from "../services/blockchain";
import { formatAddress, formatBalance } from "../config/blockchain";

export const initBlockchain = createAsyncThunk(
  "blockchain/initBlockchain",
  async (_, { rejectWithValue }) => {
    try {
      const connected = await blockchainService.init();
      if (!connected) {
        return rejectWithValue("Unable to reach Substrate node");
      }
      return connected;
    } catch (error) {
      return rejectWithValue(error.message || "Blockchain init failed");
    }
  }
);

export const connectPolkadotWallet = createAsyncThunk(
  "blockchain/connectWallet",
  async (_, { rejectWithValue }) => {
    try {
      const accounts = await blockchainService.connectWallet();
      return accounts.map((account) => ({
        address: account.address,
        name: account.meta?.name || formatAddress(account.address),
      }));
    } catch (error) {
      return rejectWithValue(error.message || "Wallet connection failed");
    }
  }
);

export const selectPolkadotAccount = createAsyncThunk(
  "blockchain/selectAccount",
  async (address, { rejectWithValue }) => {
    try {
      await blockchainService.setActiveAccount(address);
      const balance = await blockchainService.getBalance(address);
      return {
        address,
        balance,
      };
    } catch (error) {
      return rejectWithValue(error.message || "Unable to select account");
    }
  }
);

export const disconnectBlockchain = createAsyncThunk(
  "blockchain/disconnect",
  async () => {
    await blockchainService.disconnect();
    return true;
  }
);

const initialState = {
  status: "idle",
  nodeStatus: "disconnected",
  accounts: [],
  connectingWallet: false,
  activeAccount: null,
  balance: null,
  error: null,
};

const blockchainSlice = createSlice({
  name: "blockchain",
  initialState,
  reducers: {
    resetBlockchainState: () => ({ ...initialState }),
    clearBlockchainError: (state) => {
      state.error = null;
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(initBlockchain.pending, (state) => {
        state.status = "connecting";
        state.error = null;
      })
      .addCase(initBlockchain.fulfilled, (state) => {
        state.status = "connected";
        state.nodeStatus = "connected";
      })
      .addCase(initBlockchain.rejected, (state, action) => {
        state.status = "failed";
        state.nodeStatus = "failed";
        state.error = action.payload || action.error.message;
      })
      .addCase(connectPolkadotWallet.pending, (state) => {
        state.connectingWallet = true;
        state.error = null;
      })
      .addCase(connectPolkadotWallet.fulfilled, (state, action) => {
        state.connectingWallet = false;
        state.accounts = action.payload;
      })
      .addCase(connectPolkadotWallet.rejected, (state, action) => {
        state.connectingWallet = false;
        state.error = action.payload || action.error.message;
      })
      .addCase(selectPolkadotAccount.pending, (state) => {
        state.error = null;
      })
      .addCase(selectPolkadotAccount.fulfilled, (state, action) => {
        state.activeAccount = {
          address: action.payload.address,
          shortAddress: formatAddress(action.payload.address),
        };
        state.balance = action.payload.balance
          ? {
              free: formatBalance(action.payload.balance.free.replace(/,/g, "")),
              reserved: formatBalance(
                action.payload.balance.reserved.replace(/,/g, "")
              ),
            }
          : null;
      })
      .addCase(selectPolkadotAccount.rejected, (state, action) => {
        state.error = action.payload || action.error.message;
      })
      .addCase(disconnectBlockchain.fulfilled, () => ({ ...initialState }));
  },
});

export const { resetBlockchainState, clearBlockchainError } =
  blockchainSlice.actions;

export default blockchainSlice.reducer;
