# no-fucking-thing
Collateralizable NFT ecosystem

### To deplyo as migrable
```
terrain deploy gen0 --signer validator --set-signer-as-admin
```

### To migrate

Payload needed by the `MigrateMsg` should be placed in the `instantiate` block of `config.terrain.json`

Deploy a new contract but do not update the address in the `refs.terrain.json`.
Get the new `code_id`

```
terrain contract:migrate gen0 --signer validator --code-id=code_id
```

TODO: investigate why `terrain contract:migrate gen0 --signer validator` do not update properly the contract


## Useful links
https://github.com/iboss-ptk/terrain#terrain-deploy-contract
https://docs.cosmwasm.com/docs/1.0/smart-contracts/migration/#using-migrate-to-update-otherwise-immutable-state