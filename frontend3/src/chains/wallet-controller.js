import { initController, getController } from './terra-controller';
import {
    WalletStatus,
    ConnectType
  } from '@terra-money/wallet-controller';
  
import { combineLatest } from 'rxjs';

let walletController = undefined;

export async function getWalletController() {
  if (walletController) {
    return walletController;
  }

  await initController();

  walletController = new WalletController();
  return walletController;
}

export class WalletController {
    states = {};
    availableInstallTypes = [];
    availableConnections = [];
    
    constructor() {
        this.setup()
    }

    setup() {
        // Terra
        /*let subscription = */combineLatest([
            getController().availableInstallTypes(),
            getController().availableConnections(),
            getController().states(),
            ]).subscribe(
            ([
                _availableInstallTypes,
                _availableConnections,
                _states,
            ]) => {
                this.availableInstallTypes = _availableInstallTypes;
                this.availableConnections = _availableConnections;
                this.states = _states;
            },
        )
    }

    IsConnected = () => {
        return this.states.status === WalletStatus.WALLET_CONNECTED;
    }

    GetNetwork = () => {
        return this.states.network;
    }

    GetWallet = () => {
        if (this.states.status == WalletStatus.WALLET_CONNECTED) {
            return this.states.wallets[0];
        }
        return undefined;
    }

    GetConnectedAddress = () => {
        if (this.states.status == WalletStatus.WALLET_CONNECTED) {
            return this.states.wallets[0].terraAddress;
        }
        return "";
    }

    static installInfo = {
        EXTENSION: {
            type: "EXTENSION",
            name: "Install Terra Station",
            icon: "https://assets.terra.money/icon/station-extension/icon.png"
        }
    }

    GetAvailableInstallTypes = () => {
        console.log(this.availableInstallTypes)
        console.log(this.availableInstallTypes.map(it => (this.installInfo[it])))
        return this.availableInstallTypes.map(it => (this.installInfo[it]));
    }

    GetAvailableConnections = () =>  {
        console.log(this.availableConnections)
        return this.availableConnections.filter(a => a.type != ConnectType.READONLY);
    }

    Connect = (type, identifier) => {
        getController().connect(type, identifier).then(() => this.setup())
        
    }

    Disconnect = () => {
        getController().disconnect().then(() => this.setup())
    }

    Install = (type) => {
        getController().install(type);
    }
}

