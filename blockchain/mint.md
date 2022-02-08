

for i in {1..10}; do near call dev-1643314838825-38374018844065 market_mint_generic '{ "contractaddress":"dev-1643320681606-60313486665423", "token_owner_id": "'yairnava.testnet'", "colecction": "'Items_Nativo_X'", "token_metadata": { "title": "Thunder Sword", "description": "Thunder Sword", "media": "","extra":"{'"'attack'":"'3'","'defense'":"'0'","'speed'":"'0'"}'"}}'  --accountId yairnava.testnet --amount 0.1 --gas=300000000000000; done

near call dev-1643743327176-53980467452922 market_mint_generic '{ "contractaddress":"dev-1643329021198-22907018449665", "token_owner_id": "dokxo.testnet","colecction":"nativo","token_metadata": { "title": "Será este el bueno?x2", "description": "This is Hola x36", "media": "","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'joehank.testnet'","'price'":"'5'","'status'":"'S'","'on_sale'":'false',"'on_auction'":'false',"'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000


for i in {1..1000}; do near call dev-1643659132538-80320824962807 nft_mint_token '{ "contractaddress":"dev-1643331107973-95015694722073", "token_owner_id": "dokxo.testnet","colecction":"nativo","token_metadata": { "title": "Será este el bueno?x2", "description": "This is Hola x36", "media": "","copies":10,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'dokxo.testnet'","'price'":"'5'","'status'":"'S'","'on_sale'":'false',"'on_auction'":'false',"'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000; done



for i in {1..1000}; do near call dev-1643332282510-97924906831834 nft_mint_token '{"token_owner_id": "'dokxo.testnet'", "colecction": "'Burritos'", "token_metadata": { "title": "Doped Burrito", "description": "This is a doped burrito", "media": "","extra":""}}' --accountId dokxo.testnet --deposit 0.1 --gas=300000000000000; done


for i in {1..1000}; do near call dev-1643332286113-57268447308228 nft_mint_token '{"token_owner_id": "'dokxo.testnet'", "colecction": "'Items'", "token_metadata": { "title": "Thunder Sword", "description": "Thunder Sword 2", "media": "","extra":"{'"'attack'":"'3'","'defense'":"'0'","'speed'":"'0'"}'"}}' --accountId dokxo.testnet --deposit 0.1 --gas=300000000000000; done


near call dev-1643329021198-22907018449665 nft_mint_token '{ "contractaddress":"dev-1643331107973-95015694722073", "token_owner_id": "dokxo.testnet","colecction":"nativo100","token_metadata": status'":"'S'","'on_sale'":'false',"'on_auction'":'false',"'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000


near call dev-1643331107973-95015694722073 market_mint_generic '{ "contractaddress":"dev-1643329021198-22907018449665", "token_owner_id": "joehank.testnet","colecction":"dokxodeded","token_metadata": { "title": "Será este el bueno?x2", "description": "This is Hola x36", "media": "","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'joehank.testnet'","'price'":"'5'","'status'":"'S'","'on_sale'":'false',"'on_auction'":'false',"'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000



near call dev-1643659132538-80320824962807 nft_transfer_call '{ "receiver_id": "dokxo.testnet" , "token_id": "5", "approval_id": null, "memo": "", "msg":"" }'  --accountId dokxo.testnet --deposit 0.1







near call dev-1643659132538-80320824962807 nft_transfer_call '{"owner_id":"dokxo.testnet","receiver_id":"missael.testnet","token_id":"5","approved_account_ids":""}'  --accountId dokxo.testnet

near call dev-1643659132538-80320824962807 nft_transfer_call '{"receiver_id":"missael.testnet","token_id":"5","approval_id":null,"memo":null,"msg":"hola"}'  --accountId dokxo.testnet



near call dev-1643831579485-65926926025695 market_mint_generic '{ "contractaddress":"dev-1643838495610-28974280649934", "token_owner_id": "'yairnava.testnet'", "colecction": "'Burritos'", "token_metadata": { "title": "Doped Burrito", "description": "This is a doped burrito", "media": "","extra":""}}'  --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000

 near view dev-1643838495610-28974280649934  nft_token '{"token_id":"1","token_owner_id": "dokxo.testnet"}' --accountId dokxo.testnet



 near call dev-1643782582362-23960957659801 nft_transfer '{"receiver_id": "yairnava.testnet","token_id": "2","approval_id": null, "memo": null}' --accountId dokxo.testnet --deposit 0.000000000000000000000001


 near call dev-1643782582362-23960957659801 nft_transfer '{"token_id":"1","receiver_id":"yairnava.testnet"}' --accountId dokxo.testnet --deposit 0.000000000000000000000001


near call dev-1643838495610-28974280649934 fight_player_cpu '{"burrito1_id": "2","accesorio1_burrito1_id":"0","accesorio2_burrito1_id":"1","accesorio3_burrito1_id":"2","burrito_cpu_level":5}' --accountId dokxo.testnet --gas=300000000000000



'{"token_id":next_tokenid_counter,"token_owner_id": "'dokxo.testnet'",  "token_metadata": { "title": "Some titile", "description": "Some description", "media": "","extra":"","copies":copies_number}}'