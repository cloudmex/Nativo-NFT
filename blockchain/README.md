# Marketplace NFT Nativo - Rust contract

## Pasos previos 
Asegurate de tener instalado la herramienta `near-cli`, esta sera utilizada para consumir los metodos en el contrato. Para instalarla usamos el comando con npm: 

`npm install -g near-cli`

# 🚀 Desplegar el contrato en la Testnet de NEAR
## Hacer login con el NEAR CLI
`near login`

## 1Construir el smart contract para Rust 
`cargo build --target wasm32-unknown-unknown --release`

## 2Desplegar contrato en testnet
`near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm`
`near deploy nativov2.testnet --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm`
 
## Migrar estado
`near deploy --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm --initFunction "migrate" --initArgs "{}" --accountId dev-1645834304094-39444932173832 `

`near deploy \
  --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm \
  --initFunction "migrate" \
  --initArgs "{}" \
  --accountId dokxo.testnet`
# 💻 Comandos del contrato

## Inicializar contrato con los valores en la metadata 
`3 near call dev-1645834304094-39444932173832 new_default_meta   --accountId dokxo.testnet`
## Obtener la metadata del contrato
`near view dev-1645834304094-39444932173832  get_market_contract --accountId dokxo.testnet`

## 4Agregar un nuevo contracto External_Contract
`near call dev-1645834304094-39444932173832 add_new_ext_contract '{"address_contract":"dev-1645120562893-85925146475611","address_owner":"dokxo.testnet","contract_name":"Nativo mkt"}' --accountId dokxo.testnet --deposit 5`

## Agregar colleciones al marketplace
 `near call dev-1645834304094-39444932173832 add_user_collection '{"address_contract":"dev-1645120562893-85925146475611","address_collection_owner":"joehank.testnet","title":"redB","descrip":"a blod collection","mediaicon":"String","mediabanner":"String"}' --deposit 0.1 --accountId dokxo.testnet`
## Minar un token 
`near call dev-1645834304094-39444932173832 market_mint_generic '{ "address_contract":"dev-1645130223898-74544896598882", "token_owner_id": "joehank.testnet","collection_id":"redB","collection":"redB","token_metadata": { "title": "Será este ", "description": "This panda", "media": "","copies":5,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'joehank.testnet'","'price'":"'5'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000`

## Comprar un token NFT
`near call dev-1645834304094-39444932173832 market_buy_generic '{"address_contract":"<direccion del contrato minero>", "token_id": "3","collection":"Hola"}' --accountId <direccion del comprador>  --amount <precio en nears> --gas=300000000000000`

## Ofertar un token NFT
`near call dev-1645834304094-39444932173832 market_bid_generic '{"address_contract":"dev-1645215283232-83708320064039", "token_id": "3","collection_id":"Hola","collection":"Hola"}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000`
## Vender un token NFT
`near call dev-1645834304094-39444932173832 revender '{"token_id": "0","price": "0"}' --accountId <tu nearId>`

## Remover desde el market
`near call dev-1645834304094-39444932173832 market_remove_generic '{"address_contract":"<direccion del contrato minero>", "token_id": "26","collection":"aaaaaaaaa"}' --accountId <tu nearId>   --gas=300000000000000`



## Tokens NFT pertenecientes a una cuenta de NEAR
`near view dev-1645834304094-39444932173832 tokens_of '{"account_id": "nearId","from_index": "0","limit": 3}'`

 
 
## Obtener informacion de un token desde el minter 
 `near view <direccion del contrato minero> nft_token '{"token_id":"22","token_owner_id":"dokxo.testnet" }'  --accountId <tu nearId>`

## guardar token minado en the graph  
`near call dev-1645834304094-39444932173832 save_mint_ttg  '{"info":" String"}' --accountId dokxo.testnet`

 

  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html