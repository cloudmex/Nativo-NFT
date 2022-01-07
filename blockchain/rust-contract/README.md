# Marketplace NFT culturas latinas - Rust contract

## Pasos previos 
Asegurate de tener instalado la herramienta `near-cli`, esta sera utilizada para consumir los metodos en el contrato. Para instalarla usamos el comando con npm: 

`npm install -g near-cli`

# 🚀 Desplegar el contrato en la Testnet de NEAR
## Hacer login con el NEAR CLI
`near login`

## Construir el smart contract para WebAssambly 
`cargo build --target wasm32-unknown-unknown --release`

## Desplegar contrato en testnet
`near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/non_fungible_token.wasm`
` near deploy nativov2.testnet --wasmFile target/wasm32-unknown-unknown/release/non_fungible_token.wasm`
 
# 💻 Comandos del contrato

## Inicializar contrato con los valores en la metadata 
`near call <direccion del contrato> new_default_meta '{"owner_id": "owner nearId"}' --accountId <tu nearId>`
## Obtener la metadata del contrato
`near view <direccion del contrato> nft_metadata`

## Minar un token 
`near call <direccion del contrato> minar '{"token_owner_id": "owner nearId", "token_metadata": '{ "title": "nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash": "imagenimagenimagenimagenimagenim","price": "1000000","on_sale": true}}' --accountId <tu nearId> --amount 0.1`

## Comprar un token NFT
`near call <dirección del contrato> comprar_nft '{"token_id": "token id"}' --accountId <tu nearId> --amount 0.01`

## Revender un token NFT
`near call <direccion del contrato> revender '{"token_id": "0","price": "0"}' --accountId <tu nearId>`

## Tokens NFT pertenecientes a una cuenta de NEAR
`near view <direccion del contrato> tokens_of '{"account_id": "nearId","from_index": "0","limit": 3}'`

## Quitar un token a la venta del marketplace
`near call <direccion del contrato> quitar_del_market_place '{"token_id": "0"}' --accountId <tu nearId>`

## Nos permite obtener los tokens NFT filtrados por pagina a la venta 
`near view <direccion del contrato> obtener_pagina_v2 '{"from_index": 0,"limit": 7}'`


  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html