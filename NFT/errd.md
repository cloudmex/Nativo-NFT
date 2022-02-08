near call dev-1638228892358-63955043240214 new_default_meta '{"owner_id":"joehank.testnet"}' --accountId dev-1638228892358-63955043240214

near call dev-1636747327239-18935385243808 minar '{ "token_owner_id":"dokxo.testnet","token_metadata":"{"title":"nombre del token","description":"descripción","media":"imagenim","media_hash":"imageni"}"}' --accountId dev-1636747327239-18935385243808 --amount 0.1 


//////////////////////////////////////
funciona chido con colleción haciendo el xxc

near call dev-1643329021198-22907018449665 nft_mint_token '{ "contractaddress":"dev-1643331107973-95015694722073", "token_owner_id": "joehank.testnet","colecction":"nativo","token_metadata": { "title": "Será este el bueno?x2", "description": "This is Hola x36", "media": "","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'joehank.testnet'","'price'":"'5'","'status'":"'S'","'on_sale'":'false',"'on_auction'":'false',"'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId joehank.testnet  --amount 0.1 --gas=300000000000000

////////////////////
FUNCIONA AUN MAS CHIDO CON TODO Y TAGS

near call dev-1643659132538-80320824962807 nft_mint_token '{ "contractaddress":"dev-1643331107973-95015694722073", "token_owner_id": "joehank.testnet","colecction":"nativo","token_metadata": { "title": "Será este el bueno?x2", "description": "This is Hola x36", "media": "","extra":"{'"'tags'":"'#hola#adios#bye'","'creator'":"'joehank.testnet'","'price'":"'5'","'status'":"'S'","'on_sale'":'false',"'on_auction'":'false',"'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId joehank.testnet  --amount 0.1 --gas=300000000000000


///////////////////////////////////////
near call dev-1636751893359-19496702378959 minar '{"token_owner_id": "dev-1636751893359-19496702378959", "token_metadata":'{"title":"nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash":"imagenimagenimagenimagenimagenim"}}' --accountId dev-1636751893359-19496702378959 --amount 0.1

near call dev-1636747327239-18935385243808 minar "'{"token_owner_id": "dokxo.testnet", "token_metadata": "'{ "title": "nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash": "imagenimagenimagenimagenimagenim"}}'" --accountId dev-1636747327239-18935385243808 --amount 0.1

near call dev-1636747327239-18935385243808 minar "'{"token_owner_id": ""dokxo.testnet"", "token_metadata": "'{ "title": "nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash": "imagenimagenimagenimagenimagenim"}'" }'" --accountId dev-1636747327239-18935385243808 --amount 0.1

near call dev-1636747327239-18935385243808 minar ‘{token_owner_id: “dokxo.testnet", “token_metadata”: { “title”: “nombre del token”, “description”: “descripción”, “media”: “imagenimagenimagenimagenimagenim”,“extra”:“{‘“hp”:“20",“attack”:“10",“defense”:“15",“speed”:“13"}‘“}}’ --accountId dev-1636747327239-18935385243808 --deposit 0.1


EL CHIDO CON EL QUE SÍ FUNCIONA LA MINACIÓN 

near call dev-1636751893359-19496702378959 minar '{"token_owner_id": "dev-1636751893359-19496702378959", "token_metadata":{"title":"nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash":"imagenimagenimagenimagenimagenim"}}' --accountId dev-1636751893359-19496702378959 --amount 0.1

near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_marketplace.wasm --initFunction "migrate" --initArgs "{}" --accountId nativov2.testnet


near call dev-1636751893359-19496702378959 minar '{"token_owner_id": "dev-1636751893359-19496702378959", "token_metadata": { "title": "Tenochtitlan", "description": "This is Tenochtitlan", "media": "imagenimagenimagenimagenimagenim","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'dev-1636751893359-19496702378959'","'price'":"'10'"}'"}}' --accountId dev-1636751893359-19496702378959 --amount 0.1

near call dev-1638316774838-87871339743164 get_token '{"token_id": "1"}' --accountId joehank.testnet


near call dev-1636751893359-19496702378959 update_token '{"token_id": "1", "extra":"{'"'culture'":"'Burriroca'","'country'":"'BurritoLand'","'creator'":"'dev-1636751893359-19496702378959'","'price'":"'20'"}'"}' --accountId dev-1636751893359-19496702378959

near call dev-1636751893359-19496702378959 update_token '{"token_id": "1", "extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'joehank.testnet'","'price'":"'10'","'on_sale'":"true","'on_auction'":"false","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}' --accountId dev-1636751893359-19496702378959

near view dev-1638228892358-63955043240214 obtener_pagina_v2 '{"from_index":1,"limit":3}' --accountId dev-1638228892358-63955043240214

near call dev-1638316774838-87871339743164 minar '{"token_owner_id": "joehank.testnet", "token_metadata": { "title": "Hola Nada", "description": "This is Hola x34", "media": "imagenimagenimagenimagenimagenim","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'dokxo.testnet'","'price'":"'5'","'on_sale'":"true","'on_auction'":"false","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId joehank.testnet --amount 0.1

near call nft nft_approve '{ "token_id": "0", "account_id": "ejemplo.testnet" }' --accountId joehank --amount .000000000000000000000001

near call dev-1638320499944-43021039384357 minar '{"token_owner_id": "dokxo.testnet", "token_metadata": { "title": "Auction_1", "description": "Auction_1", "media": "imagenimagenimagenimagenimagenim","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'dokxo.testnet'","'price'":"'0'","'on_sale'":"false","'on_auction'":"true","'adressbidder'":"'accountbidder'","'highestbidder'":"'5'","'lowestbidder'":"'1'","'expires_at'":"'1640887469649'","'starts_at'":"'1638295624280'"}'"}}' --accountId dokxo.testnet --amount 0.1
near call dev-1638228892358-63955043240214 new_default_meta '{"owner_id":"joehank.testnet"}' --accountId dev-1638228892358-63955043240214

near call dev-1636747327239-18935385243808 minar '{ "token_owner_id":"dokxo.testnet","token_metadata":"{"title":"nombre del token","description":"descripción","media":"imagenim","media_hash":"imageni"}"}' --accountId dev-1636747327239-18935385243808 --amount 0.1 

near call dev-1636751893359-19496702378959 minar '{"token_owner_id": "dev-1636751893359-19496702378959", "token_metadata":'{"title":"nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash":"imagenimagenimagenimagenimagenim"}}' --accountId dev-1636751893359-19496702378959 --amount 0.1

near call dev-1636747327239-18935385243808 minar "'{"token_owner_id": "dokxo.testnet", "token_metadata": "'{ "title": "nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash": "imagenimagenimagenimagenimagenim"}}'" --accountId dev-1636747327239-18935385243808 --amount 0.1

near call dev-1636747327239-18935385243808 minar "'{"token_owner_id": ""dokxo.testnet"", "token_metadata": "'{ "title": "nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash": "imagenimagenimagenimagenimagenim"}'" }'" --accountId dev-1636747327239-18935385243808 --amount 0.1

near call dev-1636747327239-18935385243808 minar ‘{token_owner_id: “dokxo.testnet", “token_metadata”: { “title”: “nombre del token”, “description”: “descripción”, “media”: “imagenimagenimagenimagenimagenim”,“extra”:“{‘“hp”:“20",“attack”:“10",“defense”:“15",“speed”:“13"}‘“}}’ --accountId dev-1636747327239-18935385243808 --deposit 0.1


EL CHIDO CON EL QUE SÍ FUNCIONA LA MINACIÓN 

near call dev-1636751893359-19496702378959 minar '{"token_owner_id": "dev-1636751893359-19496702378959", "token_metadata":{"title":"nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash":"imagenimagenimagenimagenimagenim"}}' --accountId dev-1636751893359-19496702378959 --amount 0.1

near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_marketplace.wasm --initFunction "migrate" --initArgs "{}" --accountId dev-1636751893359-19496702378959


near call dev-1636751893359-19496702378959 minar '{"token_owner_id": "dev-1636751893359-19496702378959", "token_metadata": { "title": "Tenochtitlan", "description": "This is Tenochtitlan", "media": "imagenimagenimagenimagenimagenim","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'dev-1636751893359-19496702378959'","'price'":"'10'"}'"}}' --accountId dev-1636751893359-19496702378959 --amount 0.1

near call dev-1636751893359-19496702378959 get_token '{"token_id": "1"}' --accountId dev-1636751893359-19496702378959


near call dev-1636751893359-19496702378959 update_token '{"token_id": "1", "extra":"{'"'culture'":"'Burriroca'","'country'":"'BurritoLand'","'creator'":"'dev-1636751893359-19496702378959'","'price'":"'20'"}'"}' --accountId dev-1636751893359-19496702378959

near call dev-1636751893359-19496702378959 update_token '{"token_id": "1", "extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'joehank.testnet'","'price'":"'10'","'on_sale'":"true","'on_auction'":"false","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}' --accountId dev-1636751893359-19496702378959

near view nativodeploy.testnet obtener_pagina_v3 '{"from_index":0,"limit":3}' --accountId nativodeploy.testnet

near call dev-1641323011542-50713101816242 minar '{"token_owner_id": "joehank.testnet", "token_metadata": { "title": "Hola Nada", "description": "This is Hola x34", "media": "","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'joehank.testnet'","'price'":"'5'","'on_sale'":"true","'on_auction'":"false","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId joehank.testnet --amount 0.1

near call nft nft_approve '{ "token_id": "0", "account_id": "ejemplo.testnet" }' --accountId joehank --amount .000000000000000000000001


near call nativodeploy.testnet nft nft_approve '{"token_id": "20","account_id": "joehank.testnet","msg":""}' --accountId joehank.testnet --amount .000000000000000000000001

near call nativodeploy.testnet ofertar_subasta '{"token_id": "10"}' --accountId dokxo.testnet --amount 5

near view nativodeploy.testnet obtener_pagina_v3 '{"from_index":0,"limit":136}' --accountId nativodeploy.testnet

near view nativodeploy.testnet obtener_pagina_v3_auction '{"from_index":0,"limit":136}' --accountId nativodeploy.testnet

near deploy --wasmFile target/wasm32-unknown-unknown/release/non_fungible_token.wasm --initFunction "migrate" --initArgs "{}" --accountId nativov2.testnet

near view nativodeploy.testnet obtener_pagina_v3_by_filtros '{"from_index":0,"limit":30,"culture":"null","country":"null"}' --accountId nativodeploy.testnet

#obtener los id por paginacion para galeria
near view nativodeploy.testnet get_id_onsale '{"tokens":30}' --accountId nativodeploy.testnet

#Busqueda sin filtros para galeria 
near view nativodeploy.testnet obtener_pagina_v3_by_filtros '{"from_index":0,"limit":30,"culture":"null","country":"null"}' --accountId nativodeploy.testnet
#Busqueda con filtro de cultura para galeria 
near view nativodeploy.testnet obtener_pagina_v3_by_filtros '{"from_index":0,"limit":30,"culture":"carros","country":"null"}' --accountId nativodeploy.testnet
#Busqueda con filtro de country para galeria 
near view nativodeploy.testnet obtener_pagina_v3_by_filtros '{"from_index":0,"limit":30,"culture":"null","country":"México"}' --accountId nativodeploy.testnet
#Busqueda con filtro de culture y country  para galeria 
near view nativodeploy.testnet obtener_pagina_v3_by_filtros '{"from_index":0,"limit":30,"culture":"carros","country":"México"}' --accountId nativodeploy.testnet


near view nativodeploy.testnet obtener_pagina_v4 '{"from_index":0,"limit":30}' --accountId nativodeploy.testnet

near view nativodeploy.testnet obtener_pagina_v5 '{"from_index":0,"limit":30,"culture":"carros","country":"México"}' --accountId nativodeploy.testnet


#Busqueda sin filtros para galeria 
near view nativodeploy.testnet obtener_pagina_v5 '{"from_index":0,"limit":30,"culture":"null","country":"null"}' --accountId nativodeploy.testnet
#Busqueda con filtro de cultura para galeria 
near view nativodeploy.testnet obtener_pagina_v5 '{"from_index":0,"limit":30,"culture":"carros","country":"null"}' --accountId nativodeploy.testnet
#Busqueda con filtro de country para galeria 
near view nativodeploy.testnet obtener_pagina_v5 '{"from_index":0,"limit":30,"culture":"null","country":"México"}' --accountId nativodeploy.testnet
#Busqueda con filtro de culture y country  para galeria 
near view nativodeploy.testnet obtener_pagina_v5 '{"from_index":0,"limit":30,"culture":"carros","country":"México"}' --accountId nativodeploy.testnet


near call nativoapp.testnet tryhash '{"token_id":"web","status":"onsale"}' --accountId dokxo.testnet
 

 near deploy --wasmFile target/wasm32-unknown-unknown/release/non_fungible_token.wasm --initFunction "migrate" --initArgs "{}" --accountId nativoapp.testnet

cargo build --all --target wasm32-unknown-unknown --release
 -////////////////////////////////////////////////////////////////////////////////////////////////
near call nativoapp.testnet get_pagination_onsale  '{"tokens":25}' --accountId dokxo.testnet

 near call nativoapp.testnet inserthash '{"token_id":"52","info":["onsale","dokxo.testnet"]}' --accountId dokxo.testnet
 near call nativoapp.testnet gethash '{"tokens":"33"}' --accountId dokxo.testnet
 near call nativoapp.testnet fillhash  --accountId dokxo.testnet
 near call nativoapp.testnet resethash --accountId dokxo.testnet

 near call nativoapp.testnet obtener_pagina_on_sale '{"tokens":25,"_start_index":0}' --accountId dokxo.testnet
 near call nativoapp.testnet obtener_pagina_on_auction '{"tokens":25,"_start_index":0}' --accountId dokxo.testnet
 near call nativoapp.testnet obtener_pagina_by_owner '{"account":"syi216.testnet"}' --accountId dokxo.testnet

  near view nativoapp.testnet obtener_pagina_on_sale '{"tokens":25,"_start_index":0,"_creatoraccount":"tes","_owneraccount":"tes"}' --accountId joehank.testnet


  near call dev-1640023698418-93584970975929 fillhash  --accountId dokxo.testnet
 near call dev-1640023698418-93584970975929 gethash '{"tokens":"1999"}' --accountId dokxo.testnet
 near call sub.nativoapp.testnet inserthash '{"token_id":"2000","info":["S","joehank.testnet","joehank.testnet","3","1640205511361358482"]}' --accountId joehank.testnet
near view dev-1640023698418-93584970975929 get_pagination_onsale  '{"tokens":25}' --accountId dokxo.testnet
 near call dev-1640023698418-93584970975929 resethash --accountId dokxo.testnet


near deploy --wasmFile target/wasm32-unknown-unknown/release/non_fungible_token.wasm --initFunction "migrate" --initArgs "{}" --accountId nativov2.testnet --initGas=300000000000000



near view nativov3.testnet get_pagination_onsale_filters_v2   '{"tokens":25,"_start_index":0,"_minprice":0,"_maxprice":0,"_mindate":0,"_maxdate":0}' --accountId dokxo.testnet

//get pagination 4 creator
near view nativov2.testnet get_pagination_creator_filters   '{"account":"alantest.testnet","tokens":25,"_start_index":0,"_minprice":0,"_maxprice":0,"_mindate":0,"_maxdate":0}' --accountId dokxo.testnet


//get tokens by creator
near view nativov2.testnet obtener_pagina_creator   '{"account":"alantest.testnet","chunk":0,"tokens":25,"_start_index":0,"_minprice":0,"_maxprice":0,"_mindate":0,"_maxdate":0}' --accountId dokxo.testnet

//get pagination 4 owner
near view nativov2.testnet get_pagination_owner_filters   '{"account":"alantest.testnet","tokens":25,"_start_index":0,"_minprice":0,"_maxprice":0,"_mindate":0,"_maxdate":0}' --accountId dokxo.testnet


//get tokens by owner
near view nativov2.testnet obtener_pagina_owner   '{"account":"alantest.testnet","chunk":0,"tokens":25,"_start_index":0,"_minprice":0,"_maxprice":0,"_mindate":0,"_maxdate":0}' --accountId dokxo.testnet