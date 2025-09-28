/* global BigInt */
// AlexandrIA Blockchain Configuration

export const BLOCKCHAIN_CONFIG = {
  NODE_ENDPOINT: 'ws://127.0.0.1:9944',
  CONTRACT_ADDRESS: '5HGanrAXjnqhpq3wFzRg5wXd4FZR619FXtugZHNz9bE7rwCB',
  CODE_HASH: '0xd4271e4966f97de0c2c7ba20b2b679fca7084b9992de619ebddd5028c5a82c07',
  NETWORK: {
    name: 'Substrate Contracts Node',
    token: 'UNIT',
    decimals: 12,
    prefix: 42,
  },
  CONTRACT_SETTINGS: {
    packPrice: '1000000000000',
    evolutionFee: '100000000000',
    marketplaceFee: 250,
  },
  GAS_LIMITS: {
    mint: '100000000000',
    transfer: '50000000000',
    listForSale: '50000000000',
    buyCard: '100000000000',
    evolve: '75000000000',
    initiateBattle: '100000000000',
    completeBattle: '75000000000',
  },
  TEST_ACCOUNTS: {
    Alice: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
    Bob: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
    Charlie: '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y',
    Dave: '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
    Eve: '5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw',
  },
};

export const formatBalance = (balance) => {
  if (balance === null || balance === undefined) {
    return '0.0000';
  }

  try {
    const raw = typeof balance === 'string' ? BigInt(balance) : BigInt(balance.toString());
    const decimals = BigInt(10) ** BigInt(BLOCKCHAIN_CONFIG.NETWORK.decimals);
    const whole = raw / decimals;
    const fraction = raw % decimals;
    const fractionStr = fraction.toString().padStart(BLOCKCHAIN_CONFIG.NETWORK.decimals, '0');
    return `${whole.toString()}.${fractionStr.slice(0, 4)}`;
  } catch (error) {
    console.warn('Unable to format balance', balance, error);
    return '0.0000';
  }
};

export const parseBalance = (balance) => {
  if (!balance) {
    return '0';
  }

  const [wholeRaw, fractionRaw = ''] = balance.split('.');
  const decimals = BLOCKCHAIN_CONFIG.NETWORK.decimals;
  const whole = BigInt(wholeRaw || '0');
  const fraction = BigInt((fractionRaw + '0'.repeat(decimals)).slice(0, decimals) || '0');
  const scale = BigInt(10) ** BigInt(decimals);
  return (whole * scale + fraction).toString();
};

export const formatAddress = (address) => {
  if (!address) return '';
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
};

export default BLOCKCHAIN_CONFIG;
