
ID=dev-1644884218427-97542306092581
echo $ID


## 1Construir el smart contract para Rust 
`cargo build --target wasm32-unknown-unknown --release`

## 2Desplegar contrato en testnet
`near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm`
`near deploy nativov2.testnet --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm` 


Initializes the contract with the given total supply owned by the given `owner_id`.
near call $ID new '{"owner_id": "dokxo.testnet"}' --accountId $ID

Obtener propietario del contrato
near view $ID get_owner_id

Cambiar propietario del contrato
near call $ID set_owner_id '{"owner_id": "x123.testnet"}' --accountId dokxo.testnet
near call $ID set_owner_id '{"owner_id": "dokxo.testnet"}' --accountId x123.testnet

Obtener lista de mineros
near view $ID get_minters

Agregar minero
near call $ID add_minter '{"account_id": "joehank.testnet"}' --accountId dokxo.testnet --deposit 0.000000000000000000000001

Remover minero
near call $ID remove_minter '{"account_id": "bbtoken.testnet"}' --accountId dokxo.testnet --deposit 0.000000000000000000000001

Minar
1.5 Token
near call $ID mint '{"account_id": "joehank.testnet", "amount" : "1500000000000000000000000"}' --accountId dokxo.testnet --deposit 0.000000000000000000000001
100 Token
near call $ID mint '{"account_id": "dokxo.testnet", "amount" : "100000000000000000000000000"}' --accountId dokxo.testnet --deposit 0.000000000000000000000001

Obtener valance total
near view $ID ft_total_supply

Obtener balance de una cuenta
near view $ID ft_balance_of '{"account_id": "joehank.testnet"}'
near view $ID ft_balance_of '{"account_id": "dokxo.testnet"}'

Mostrar tokens en Wallet
near call $ID ft_transfer '{"receiver_id": "joehank.testnet", "amount":"0", "memo":""}' --accountId dokxo.testnet --deposit 0.000000000000000000000001

Minar tokens y agregarlos al wallet
100 tokens
near call $ID reward_player '{"player_owner_id": "dokxo.testnet", "tokens_mint" : "100000000000000000000000000"}' --accountId $ID --deposit 0.000000000000000000000001

32.58
near call $ID reward_player '{"player_owner_id": "dokxo.testnet", "tokens_mint" : "32580000000000000000000000"}' --accountId $ID --deposit 0.000000000000000000000001


1000000000000000000000000
2500000200000000000000000