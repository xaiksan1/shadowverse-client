import { useEffect, useMemo } from "react";
import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  CircularProgress,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  Stack,
  Typography,
} from "@mui/material";
import { useDispatch, useSelector } from "react-redux";
import {
  clearBlockchainError,
  connectPolkadotWallet,
  disconnectBlockchain,
  initBlockchain,
  selectPolkadotAccount,
} from "../../redux/blockchainSlice";

const WalletPanel = () => {
  const dispatch = useDispatch();
  const {
    status,
    nodeStatus,
    accounts,
    connectingWallet,
    activeAccount,
    balance,
    error,
  } = useSelector((state) => state.blockchain);

  useEffect(() => {
    dispatch(initBlockchain());
  }, [dispatch]);

  useEffect(() => {
    if (accounts.length > 0 && !activeAccount) {
      dispatch(selectPolkadotAccount(accounts[0].address));
    }
  }, [accounts, activeAccount, dispatch]);

  useEffect(() => {
    if (error) {
      const timeout = setTimeout(() => dispatch(clearBlockchainError()), 8000);
      return () => clearTimeout(timeout);
    }
    return undefined;
  }, [dispatch, error]);

  const disabled = useMemo(() => status === "connecting", [status]);

  return (
    <Card
      sx={{
        width: "100%",
        maxWidth: 420,
        backgroundColor: "rgba(12, 18, 32, 0.85)",
        border: "1px solid rgba(255,255,255,0.08)",
        color: "#f0f6ff",
        boxShadow: "0 0 40px rgba(0,0,0,0.4)",
      }}
    >
      <CardContent>
        <Stack spacing={2}>
          <Box>
            <Typography variant="h6" fontWeight={600} gutterBottom>
              Blockchain Link
            </Typography>
            <Typography variant="body2" sx={{ opacity: 0.7 }}>
              Connect your Polkadot.js wallet to sync AlexandrIA cards on-chain.
            </Typography>
          </Box>

          <Stack direction="row" spacing={1} alignItems="center">
            <Box
              sx={{
                width: 12,
                height: 12,
                borderRadius: "50%",
                backgroundColor:
                  nodeStatus === "connected"
                    ? "#4caf50"
                    : nodeStatus === "connecting"
                    ? "#ff9800"
                    : "#f44336",
              }}
            />
            <Typography variant="body2" sx={{ opacity: 0.8 }}>
              Node status: {nodeStatus}
            </Typography>
          </Stack>

          {error && (
            <Alert severity="error" sx={{ backgroundColor: "rgba(244,67,54,0.1)" }}>
              {error}
            </Alert>
          )}

          <Stack spacing={2}>
            {accounts.length === 0 ? (
              <Button
                variant="contained"
                onClick={() => dispatch(connectPolkadotWallet())}
                disabled={disabled}
                sx={{
                  textTransform: "none",
                  fontWeight: 600,
                  borderRadius: "999px",
                  background:
                    "linear-gradient(135deg, rgba(168,85,247,0.9) 0%, rgba(14,165,233,0.9) 100%)",
                  boxShadow: "0 8px 24px rgba(14,165,233,0.35)",
                }}
                startIcon={connectingWallet ? <CircularProgress size={18} /> : null}
              >
                {connectingWallet ? "Unlocking wallet..." : "Connect Polkadot.js"}
              </Button>
            ) : (
              <Stack spacing={2}>
                <FormControl fullWidth size="small">
                  <InputLabel id="wallet-select-label" sx={{ color: "#90caf9" }}>
                    Active account
                  </InputLabel>
                  <Select
                    labelId="wallet-select-label"
                    value={activeAccount?.address || ""}
                    label="Active account"
                    onChange={(event) =>
                      dispatch(selectPolkadotAccount(event.target.value))
                    }
                    sx={{
                      color: "#f0f6ff",
                      ".MuiOutlinedInput-notchedOutline": {
                        borderColor: "rgba(255,255,255,0.2)",
                      },
                      "&.Mui-focused .MuiOutlinedInput-notchedOutline": {
                        borderColor: "rgba(129,140,248,0.6)",
                      },
                      "& .MuiSvgIcon-root": { color: "#f0f6ff" },
                    }}
                  >
                    {accounts.map((account) => (
                      <MenuItem key={account.address} value={account.address}>
                        {account.name}
                      </MenuItem>
                    ))}
                  </Select>
                </FormControl>

                <Box
                  sx={{
                    backgroundColor: "rgba(255,255,255,0.04)",
                    borderRadius: 2,
                    padding: 2,
                  }}
                >
                  <Typography variant="subtitle2" sx={{ opacity: 0.7 }}>
                    Address
                  </Typography>
                  <Typography variant="body2" fontFamily="monospace">
                    {activeAccount?.shortAddress || "-"}
                  </Typography>

                  <Box mt={2}>
                    <Typography variant="subtitle2" sx={{ opacity: 0.7 }}>
                      Balance (UNIT)
                    </Typography>
                    <Typography variant="body1" fontWeight={600}>
                      {balance?.free ?? "0.0000"}
                    </Typography>
                  </Box>
                </Box>

                <Button
                  variant="outlined"
                  color="secondary"
                  onClick={() => dispatch(disconnectBlockchain())}
                  sx={{
                    textTransform: "none",
                    fontWeight: 600,
                    borderRadius: "999px",
                    borderColor: "rgba(255,255,255,0.2)",
                    color: "#f0f6ff",
                    "&:hover": {
                      borderColor: "rgba(255,255,255,0.4)",
                      backgroundColor: "rgba(255,255,255,0.08)",
                    },
                  }}
                >
                  Disconnect
                </Button>
              </Stack>
            )}
          </Stack>
        </Stack>
      </CardContent>
    </Card>
  );
};

export default WalletPanel;
