import { ApiPromise, WsProvider } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
import { web3Enable, web3Accounts, web3FromAddress } from '@polkadot/extension-dapp';
import contractAbi from '../assets/alexandria_cards.json';
import { BLOCKCHAIN_CONFIG } from '../config/blockchain';

class BlockchainService {
  constructor() {
    this.api = null;
    this.contract = null;
    this.accounts = [];
    this.activeAccount = null;
    this.signer = null;
    this.connected = false;
    this.initializing = false;
  }

  async init(wsEndpoint = BLOCKCHAIN_CONFIG.NODE_ENDPOINT) {
    if (this.connected || this.initializing) {
      return this.connected;
    }

    this.initializing = true;
    try {
      const provider = new WsProvider(wsEndpoint);
      this.api = await ApiPromise.create({ provider });
      this.connected = true;
      this.initializing = false;
      this.initContract();
      return true;
    } catch (error) {
      console.error('Failed to initialize blockchain service', error);
      this.connected = false;
      this.initializing = false;
      return false;
    }
  }

  initContract(address = BLOCKCHAIN_CONFIG.CONTRACT_ADDRESS) {
    if (!this.api) {
      return false;
    }
    try {
      this.contract = new ContractPromise(this.api, contractAbi, address);
      return true;
    } catch (error) {
      console.error('Failed to initialise contract instance', error);
      return false;
    }
  }

  async connectWallet() {
    const extensions = await web3Enable('Alexandriaverse Client');
    if (extensions.length === 0) {
      throw new Error('Polkadot.js extension not detected. Please install it first.');
    }

    this.accounts = await web3Accounts();
    if (this.accounts.length === 0) {
      throw new Error('No accounts available in the Polkadot.js extension.');
    }
    return this.accounts;
  }

  async setActiveAccount(address) {
    if (!address) {
      throw new Error('Address is required to set the active account');
    }
    const injector = await web3FromAddress(address);
    this.signer = injector.signer;
    this.activeAccount = address;
    return true;
  }

  async getBalance(address) {
    if (!this.api || !address) {
      return null;
    }
    try {
      const balance = await this.api.query.system.account(address);
      return {
        free: balance.data.free.toString(),
        reserved: balance.data.reserved.toString(),
        frozen: balance.data.frozen.toString(),
      };
    } catch (error) {
      console.error('Failed to fetch balance', error);
      return null;
    }
  }

  async disconnect() {
    try {
      if (this.api) {
        await this.api.disconnect();
      }
    } catch (error) {
      console.warn('Error while disconnecting API', error);
    }
    this.api = null;
    this.contract = null;
    this.accounts = [];
    this.activeAccount = null;
    this.signer = null;
    this.connected = false;
  }
}

const blockchainService = new BlockchainService();
export default blockchainService;
