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
`near deploy --wasmFile target/wasm32-unknown-unknown/release/nativo_marketplace.wasm --initFunction "migrate" --initArgs "{}" --accountId dev-1645031914114-90152106261098 `
# 💻 Comandos del contrato

## Inicializar contrato con los valores en la metadata 
`near call <direccion del contrato> new_default_meta '{"owner_id": "owner nearId"}' --accountId <tu nearId>`
## Obtener la metadata del contrato
`near view dev-1645042835527-47925140756016 nft_metadata`

## Agregar un nuevo contracto External_Contract
`near call dev-1645043961189-94216357988452 add_new_ext_contract '{"address_contract":"dev-1645043385143-61792682906489","address_owner":"dokxo.testnet","contract_name":"Nativo nft minter"}' --accountId dokxo.testnet --deposit 5`

## Agregar colleciones al marketplace
 `near call dev-1645043961189-94216357988452 add_user_collection '{"address_contract":"dev-1645043385143-61792682906489","address_collection_owner":"dokxo.testnet","title":"redB","descrip":"a blod collection","mediaicon":"String","mediabanner":"String"}' --deposit 0.1 --accountId dokxo.testnet`
## Minar un token 
`near call dev-1645043961189-94216357988452 market_mint_generic '{ "address_contract":"dev-1645043385143-61792682906489", "token_owner_id": "dokxo.testnet","collection":"redB","token_metadata": { "title": "Será este ", "description": "This panda", "media": "","copies":5,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'dokxo.testnet'","'price'":"'5'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000`

## Comprar un token NFT
`near call <direccion del contrato market> market_buy_generic '{"address_contract":"<direccion del contrato minero>", "token_id": "3","collection":"Hola"}' --accountId <direccion del comprador>  --amount <precio en nears> --gas=300000000000000`

## Vender un token NFT
`near call <direccion del contrato> revender '{"token_id": "0","price": "0"}' --accountId <tu nearId>`

## Remover desde el market
`near call <direccion del contrato> market_remove_generic '{"address_contract":"<direccion del contrato minero>", "token_id": "26","collection":"aaaaaaaaa"}' --accountId <tu nearId>   --gas=300000000000000`



## Tokens NFT pertenecientes a una cuenta de NEAR
`near view <direccion del contrato> tokens_of '{"account_id": "nearId","from_index": "0","limit": 3}'`

 
 
## Obtener informacion de un token desde el minter 
 `near view <direccion del contrato minero> nft_token '{"token_id":"22","token_owner_id":"dokxo.testnet" }'  --accountId <tu nearId>`


 

  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html