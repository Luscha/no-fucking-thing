<template>
    <div>
        <h1>Connect Sample</h1>
        <pre v-if="!!states">{{
            JSON.stringify(
            {
                availableConnectTypes,
                availableInstallTypes,
                availableConnections,
                states,
                supportFeatures,
            },
            null,
            2,
            )
        }}</pre>
        <footer v-if="!connected">
            <button
                v-for="(connectType, index) in availableInstallTypes"
                :key="connectType+index"
                v-on:click="controller.install(connectType)"
                >
                Install {{ connectType }}
            </button>
            <button
                v-for="(connectType, index) in availableConnectTypes"
                :key="connectType+index"
                v-on:click="controller.connect(connectType)"
                >
                Connect {{ connectType }}
            </button>
            <br />
            <button
                v-for="(connection, index) in availableConnections"
                :key="connection+index"
                v-on:click="controller.connect(connection.type, connection.identifier)"
                >
                <img
                    v-bind:src="connection.icon"
                    v-bind:alt="connection.name"
                    style="width: 1em; height: 1em"
                />
                {{ connection.name }} [{{ connection.identifier }}]
            </button>
        </footer>
        <footer v-if="connected">
            <button v-on:click="controller.disconnect()">Disconnect</button>
        </footer>
    </div>
</template>


<script>
import {
  WalletStatus,
} from '@terra-money/wallet-controller';
import { getController } from '../../controller';
import { combineLatest } from 'rxjs';

let subscription = null;

export default {
  name: 'ConnectSample',

  data() {
    return {
      availableConnectTypes: [],
      availableInstallTypes: [],
      availableConnections: [],
      states: {},
      supportFeatures: [],
      connected: false,
      controller: getController()
    }
  },

  mounted() {
    subscription = combineLatest([
      getController().availableConnectTypes(),
      getController().availableInstallTypes(),
      getController().availableConnections(),
      getController().states(),
    ]).subscribe(
      ([
        _availableConnectTypes,
        _availableInstallTypes,
        _availableConnections,
        _states,
      ]) => {
        this.availableInstallTypes = _availableInstallTypes;
        this.availableConnectTypes = _availableConnectTypes;
        this.availableConnections = _availableConnections;
        this.states = _states;
        this.supportFeatures =
          _states.status === WalletStatus.WALLET_CONNECTED
            ? Array.from(_states.supportFeatures)
            : [];
        this.connected = _states.status === WalletStatus.WALLET_CONNECTED;
      },
    )
  },

  beforeUnmount() {
    subscription?.unsubscribe();
  }
}


</script>