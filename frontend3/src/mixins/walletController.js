import { getController, initController } from '../chains/terra-controller';
import {
    WalletStatus,
    ConnectType
  } from '@terra-money/wallet-controller';
  
import { combineLatest } from 'rxjs';

const installInfo = {
    EXTENSION: {
        type: "EXTENSION",
        name: "Install Terra Station",
        icon: "https://assets.terra.money/icon/station-extension/icon.png"
    }
}

export default {
    data() {
        return {
            states: {},
            availableInstallTypes: [],
            availableConnections: [],
            subscription: undefined,
            connecting: false,
        }
    },

    async mounted() {
        initController().then(() => {
            this.subscription = combineLatest([
            getController().availableInstallTypes(),
            getController().availableConnections(),
            getController().states(),
            ]).subscribe(
            ([
                _availableInstallTypes,
                _availableConnections,
                _states,
            ]) => {
                this.availableInstallTypes = _availableInstallTypes.map(it => (installInfo[it]));
                this.availableConnections = _availableConnections;
                const connected = this.states.status !== undefined && !this.IsConnected() && _states.status === WalletStatus.WALLET_CONNECTED
                this.states = _states;
                if (this.connecting && connected) {
                    this.$router.back();
                    this.connecting = false;
                }
            },
            )
        })
    },
    
    beforeUnmount() {
        this.subscription?.unsubscribe();
    },

    computed: {
        GetAvailableInstallTypes() {
            return this.availableInstallTypes;
        },
    
        GetAvailableConnections() {
            return this.availableConnections.filter(a => a.type != ConnectType.READONLY);
        },
    },

    methods: {
        IsConnected() {
            return this.states.status === WalletStatus.WALLET_CONNECTED;
        },
    
        GetNetwork() {
            return this.states.network;
        },
    
        GetWallet() {
            if (this.states.status == WalletStatus.WALLET_CONNECTED) {
                return this.states.wallets[0];
            }
            return undefined;
        },
    
        GetConnectedAddress() {
            if (this.states.status == WalletStatus.WALLET_CONNECTED) {
                return this.states.wallets[0].terraAddress;
            }
            return "";
        },
    
        Connect(type, identifier) {
            this.connecting = true;
            return getController().connect(type, identifier)
        },
    
        Disconnect() {
            getController().disconnect();
            this.$router.back();
        },
    
        Install(type) {
            getController().install(type);
        },
    }

    
}

