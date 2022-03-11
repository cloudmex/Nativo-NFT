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
 



# 💻 Comandos del contrato

## Inicializar contrato con los valores en la metadata 
`near call dev-1647023303499-50316848036455 new_default_meta   --accountId dokxo.testnet`

## Obtener la metadata del contrato
`near view dev-1647023303499-50316848036455  get_market_contract --accountId dokxo.testnet`

## Obtener el dueño del contrato
`near view dev-1647023303499-50316848036455  get_owner_account --accountId dokxo.testnet`

## Obtener el tesorero del contrato
`near view dev-1647023303499-50316848036455  get_treasury_account --accountId dokxo.testnet`
`near call dev-1647023303499-50316848036455  set_treasury_account '{"new_account":"dokxo.testnet"}' --accountId dokxo.testnet`

## Obtener el fee de minar del contrato( no testeado)
`near call dev-1647023303499-50316848036455  market_get_actual_mint_fee '{"address_contract":"contract.testnet"}' --accountId dokxo.testnet`
## Obtener las regalias de compra/venta del contrato( no testeado)
`near call dev-1647023303499-50316848036455  market_get_actual_roy_fee --accountId dokxo.testnet`
`near call dev-1647023303499-50316848036455  market_set_actual_roy_fee '{"market_r":0.05,"owner_r":0.85,"creator_r":0.10}' --accountId dokxo.testnet`

## Agregar un nuevo contracto External_Contract
`near call dev-1647023303499-50316848036455 add_new_ext_contract '{"address_contract":"dev-1647023453177-17253653260549","address_owner":"joehank.testnet","contract_name":"Nativo minter"}' --accountId dokxo.testnet --deposit 5`

## Agregar colleciones al marketplace
 `near call dev-1647023303499-50316848036455 add_user_collection '{"address_contract":"dev-1647023453177-17253653260549","address_collection_owner":"dokxo.testnet","title":"redB","descrip":"a blod collection","mediaicon":"String","mediabanner":"String"}' --deposit 0.1 --accountId dokxo.testnet`
## Minar un token 
 `near call dev-1647023303499-50316848036455 market_mint_generic '{ "address_contract":"dev-1647023453177-17253653260549", "token_owner_id": "dokxo.testnet","collection_id":"0","collection":"redB","token_metadata": { "title": "Será este ", "description": "This panda", "media": "","copies":5,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'dokxo.testnet'","'price'":"'500000000000000000000000'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000`

## Comprar un token NFT
`near call dev-1647023303499-50316848036455 market_buy_generic '{"address_contract":"dev-1647023453177-17253653260549", "token_id": "1","collection_id":"0","collection":"redB"}' --accountId nativo-mkt.testnet  --amount 0.5 --gas=300000000000000`
## Vender un token NFT 
`near call dev-1647023303499-50316848036455 market_sell_generic '{"address_contract":"dev-1647023453177-17253653260549", "token_id": "0","price":1000000000000000,"collection_id":"0","collection":"redB"}' --accountId dokxo.testnet   --gas=300000000000000`
## Remover desde el market
`near call dev-1647023303499-50316848036455 market_remove_generic '{"address_contract":"dev-1647023453177-17253653260549", "token_id": "0","collection_id":"0","collection":"redB"}' --accountId dokxo.testnet   --gas=300000000000000`

## Ofertar un token NFT
`near call dev-1647023303499-50316848036455 market_bid_generic '{"address_contract":"dev-1647023453177-17253653260549", "token_id": "3","collection_id":"0","collection":"redB"}' --accountId nativo-mkt.testnet  --amount 0.1 --gas=300000000000000`
## Aceptar/Rechazar una oferta token NFT
`near call dev-1647023303499-50316848036455 market_close_bid_generic '{"address_contract":"dev-1647023453177-17253653260549", "token_id": "3","collection_id":"0","collection":"redB","status":false}' --accountId nativo-mkt.testnet --deposit 0.1 --gas=300000000000000`



## Migrar estado
`near deploy --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm --initFunction "migrate" --initArgs "{}" --accountId dev-1647023303499-50316848036455 `

# Métodos para la utilizacion de la DAO

## Crear una nueva propusta de actualizacion desde la DAO
`sputnikdao proposal upgrade target/wasm32-unknown-unknown/release/nativo_marketplace.wasm dev-1647023303499-50316848036455 --daoAcc nativo-mktdao --accountId dokxo.testnet`

## Crear una nueva propusta para la actualizacion del dueño del market desde la DAO
`sputnikdao proposal call  dev-1647023303499-50316848036455 set_owner_account '{"new_account":"nativo-mkt.testnet"}' --daoAcc nativo-mktdao --accountId another-mkt.testnet`

## Crear una nueva propusta para la actualizacion del tesorero del market desde la DAO
`sputnikdao call  contract_address.testnet set_treasury_account '{"new_account":"dokxo2.tesnet"}' --daoAcc nativo-mktdao --accountId another-mkt.testnet`

## Crear una nueva propusta para la actualizacion del fee para minar un nuevo token del market desde la DAO( no testeado)
`sputnikdao proposal  contract_address.testnet market_set_actual_mint_fee '{"address_contract":"address.testnet","mint_fee":"1"}' --daoAcc nativo-mktdao --accountId another-mkt.testnet`

## Crear una nueva propusta para la actualizacion de las regalias al comprar/Vender un token del market desde la DAO( no testeado)
`sputnikdao proposal  contract_address.testnet market_set_actual_roy_fee '{"address_contract":"address.testnet",market_roy:u128,owner_roy:u128,creator_roy:u128}' --daoAcc nativo-mktdao --accountId another-mkt.testnet`

## minar multiples token desde CLI para prueba de estres 
``repeat 10000 { near call dev-1646411413558-23589508762199 market_mint_generic '{ "address_contract":"dev-1646411564157-86083887856580", "token_owner_id": "dokxo.testnet","collection_id":"1","collection":"fly high to the space","token_metadata": { "title": "Será este ", "description": "This hard test", "media": "bafybeidsatbfoweclfplaph3iq3z7l3q2ikutq6an2k63o2dmh4klqcjbq","copies":5,"extra":"{'"'tags'":"'#fly'","'creator'":"'dokxo.testnet'","'price'":"'5000000000000000000000000'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000 };``