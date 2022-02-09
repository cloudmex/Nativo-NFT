## Codigo para minar muchos token desde el CLI 

for i in {1..1000}; do near call dev-1644433845094-13612285357489 market_mint_generic '{ "contractaddress":"dev-1644435847594-74924129418171", "token_owner_id": "dokxo.testnet","collection":"nativo","token_metadata": { "title": "Será este ", "description": "This panda", "media": "","copies":5,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'dokxo.testnet'","'price'":"'5'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000; done



## Minar desde el market
near call dev-1644433845094-13612285357489 market_mint_generic '{ "contractaddress":"dev-1644435847594-74924129418171", "token_owner_id": "dokxo.testnet","collection":"nativo","token_metadata": { "title": "Será este ", "description": "This panda", "media": "","copies":5,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'dokxo.testnet'","'price'":"'5'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000


## Minar desde el minter
near call dev-1644435847594-74924129418171 nft_mint_token '{ "token_owner_id": "joehank.testnet","collection":"Hola","token_metadata": { "title": "Será este el bueno?", "description": "This is Hola", "media": "","copies":1,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'joehank.testnet'","'price'":"'2'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId joehank.testnet  --amount 0.1 --gas=300000000000000


## Comprar desde el market
near call dev-1644433845094-13612285357489 market_buy_generic '{"contractaddress":"dev-1644435847594-74924129418171", "token_id": "3","collection":"Hola"}' --accountId dokxo.testnet  --amount 2 --gas=300000000000000

## Comprar desde el minter
near call dev-1644435847594-74924129418171 nft_buy_token '{ "token_id": "1","collection":"Hola"}' --accountId dokxo.testnet  --amount 2000000000000000000000000 --gas=300000000000000


## Obtener informacion de un token desde el minter 
near view dev-1644435847594-74924129418171 nft_token '{"token_id":"3","token_owner_id":"dokxo.testnet" }'  --accountId dokxo.testnet
 

