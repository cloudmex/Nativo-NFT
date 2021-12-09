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

near call dev-1638219687333-75508460603043 minar '{"token_owner_id": "joehank.testnet", "token_metadata": { "title": "Hola Nada", "description": "This is Hola x34", "media": "imagenimagenimagenimagenimagenim","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'joehank.testnet'","'price'":"'5'","'on_sale'":"true","'on_auction'":"false","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId joehank.testnet --amount 0.1

near call nft nft_approve '{ "token_id": "0", "account_id": "ejemplo.testnet" }' --accountId joehank --amount .000000000000000000000001


near call nativodeploy.testnet nft nft_approve '{"token_id": "20","account_id": "joehank.testnet","msg":""}' --accountId joehank.testnet --amount .000000000000000000000001

near call nativodeploy.testnet ofertar_subasta '{"token_id": "10"}' --accountId dokxo.testnet --amount 5

near view nativodeploy.testnet obtener_pagina_v3 '{"from_index":0,"limit":136}' --accountId nativodeploy.testnet

near view nativodeploy.testnet obtener_pagina_v3_auction '{"from_index":0,"limit":136}' --accountId nativodeploy.testnet

near deploy --wasmFile target/wasm32-unknown-unknown/release/non_fungible_token.wasm --initFunction "migrate" --initArgs "{}" --accountId nativodeploy.testnet

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