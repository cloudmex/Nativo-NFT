# Marketplace NFT Nativo - Rust contract

## Pasos previos 
Asegurate de tener instalado la herramienta `near-cli`, esta sera utilizada para consumir los metodos en el contrato. Para instalarla usamos el comando con npm: 

`npm install -g near-cli`

# 🚀 Desplegar el contrato en la Testnet de NEAR
## Hacer login con el NEAR CLI
`near login`

## Construir el smart contract para Rust 
`cargo build --target wasm32-unknown-unknown --release`

## Desplegar contrato en testnet
`near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm`
`near deploy nativov2.testnet --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm`
 
## Migrar estado
`near deploy --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm --initFunction "migrate" --initArgs "{}" --accountId dev-1646677308467-72482834496553 `


# 💻 Comandos del contrato

## Inicializar contrato con los valores en la metadata 
`near call dev-1646677308467-72482834496553 new_default_meta   --accountId dokxo.testnet`
## Obtener la metadata del contrato
`near view dev-1646677308467-72482834496553  get_market_contract --accountId dokxo.testnet`

## Agregar un nuevo contracto External_Contract
`near call dev-1646677308467-72482834496553 add_new_ext_contract '{"address_contract":"dev-1646676989514-55574580011756","address_owner":"joehank.testnet","contract_name":"Nativo minter"}' --accountId dokxo.testnet --deposit 5`

## Agregar colleciones al marketplace
 `near call dev-1646677308467-72482834496553 add_user_collection '{"address_contract":"dev-1646676989514-55574580011756","address_collection_owner":"joehank.testnet","title":"redB","descrip":"a blod collection","mediaicon":"String","mediabanner":"String"}' --deposit 0.1 --accountId dokxo.testnet`
## Minar un token 
`near call dev-1646677308467-72482834496553 market_mint_generic '{ "address_contract":"dev-1646676989514-55574580011756", "token_owner_id": "dokxo.testnet","collection_id":"0","collection":"redB","token_metadata": { "title": "Será este ", "description": "This panda", "media": "","copies":5,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'dokxo.testnet'","'price'":"'500000000000000000000000'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000`

## Comprar un token NFT
`near call dev-1646677308467-72482834496553 market_buy_generic '{"address_contract":"dev-1646676989514-55574580011756", "token_id": "5","collection_id":"0","collection":"redB"}' --accountId nativo-mkt.testnet  --amount 0.5 --gas=300000000000000`
## Vender un token NFT 
`near call dev-1646677308467-72482834496553 market_sell_generic '{"address_contract":"dev-1646676989514-55574580011756", "token_id": "0","price":1000000000000000,"collection_id":"0","collection":"redB"}' --accountId dokxo.testnet   --gas=300000000000000`
## Remover desde el market
`near call dev-1646677308467-72482834496553 market_remove_generic '{"address_contract":"dev-1646676989514-55574580011756", "token_id": "0","collection_id":"0","collection":"redB"}' --accountId dokxo.testnet   --gas=300000000000000`

## Ofertar un token NFT
`near call dev-1646677308467-72482834496553 market_bid_generic '{"address_contract":"dev-1646676989514-55574580011756", "token_id": "5","collection_id":"0","collection":"redB"}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000`
## Aceptar/Rechazar una oferta token NFT
`near call dev-1646677308467-72482834496553 market_offer_bid_generic '{"address_contract":"dev-1646676989514-55574580011756", "token_id": "5","collection_id":"0","collection":"redB","status":true}' --accountId nativo-mkt.testnet  --gas=300000000000000`
