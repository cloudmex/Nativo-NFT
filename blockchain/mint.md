## Codigo para minar muchos token desde el CLI 

for i in {1..1000}; do near call dev-1644433845094-13612285357489 market_mint_generic '{ "contractaddress":"dev-1644451200817-81163460651460

", "token_owner_id": "dokxo.testnet","collection":"nativo","token_metadata": { "title": "Será este ", "description": "This panda", "media": "","copies":5,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'dokxo.testnet'","'price'":"'5'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000; done



## Minar desde el market
near call dev-1644433845094-13612285357489 market_mint_generic '{ "contractaddress":"dev-1644523323613-61099606761670", "token_owner_id": "dokxo.testnet","collection":"cars","token_metadata": { "title": "token 2", "description": "This panda", "media": "bafybeibqelz55qgfcwctv6tzvolf4ubmmk4eumh7gelt4pkx25nxccmrbm","copies":5,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'dokxo.testnet'","'price'":"'500000000000000000000000'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000


## Minar desde el minter
near call dev-1644451200817-81163460651460 mint_token '{ "token_owner_id": "dokxo.testnet","collection":"Hola","token_metadata": { "title": "Será este el bueno?", "description": "This is Hola", "media": "","copies":1,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'dokxo.testnet'","'price'":"'2'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000


## Comprar desde el market
near call dev-1644433845094-13612285357489 market_buy_generic '{"contractaddress":"dev-1644523323613-61099606761670", "token_id": "10","collection":"cars"}' --accountId dokxotest.testnet  --amount .5 --gas=300000000000000

## Comprar desde el minter
near call dev-1644451200817-81163460651460

 nft_buy_token '{ "token_id": "1","collection":"Hola"}' --accountId dokxo.testnet  --amount 2000000000000000000000000 --gas=300000000000000

## Vender desde el market
near call dev-1644433845094-13612285357489 market_sell_generic '{"contractaddress":"dev-1644451200817-81163460651460
", "token_id": "3","price":"12","collection":"Hola"}' --accountId dokxo.testnet   --gas=300000000000000

## Vender desde el minter
near call dev-1644451200817-81163460651460
 nft_buy_token '{ "token_id": "1","collection":"Hola"}' --accountId dokxo.testnet  --amount 2000000000000000000000000 --gas=300000000000000


## Obtener informacion de un token desde el minter 
near view dev-1644451200817-81163460651460
 nft_token '{"token_id":"3","token_owner_id":"dokxo.testnet" }'  --accountId dokxo.testnet
 

