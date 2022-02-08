/*
NFT MKTPLACE CULTURAS LATINAS
 *
 */
//Implementación de los standards NFT de near
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, log, near_bindgen, setup_alloc, AccountId, Balance, BorshStorageKey, PanicOnDefault,
    Promise, PromiseOrValue, PromiseResult,
};

use more_asserts;

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names ar

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// Estructura que representa al contrato esta integra el NFT y su metadata
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    nTokenOnAuction: u64,
    nTokenOnSale: u64,
}

const DATA_IMAGE_SVG_LATINART_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    // Esta función incializa el contrato con los valores especificados en la metadata
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            //Metadata al momento de crear el contrato
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Nativo NFT".to_string(),
                symbol: "NTV".to_string(),
                icon: Some(DATA_IMAGE_SVG_LATINART_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            nTokenOnSale: 0,
        }
    }

    fn enum_get_token(&self, owner_id: AccountId, token_id: TokenId) -> Token {
        let metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .unwrap()
            .get(&token_id);
        let approved_account_ids = Some(
            self.tokens
                .approvals_by_id
                .as_ref()
                .unwrap()
                .get(&token_id)
                .unwrap_or_default(),
        );

        Token {
            token_id,
            owner_id,
            metadata,
            approved_account_ids,
        }
    }
    /**
     * permite minar nuevos tokens
     * @param token_owner_id {ValidAccountId} a quien le va a pertenecer el token
     * @param tokenMetadata {TokenMetadata} los metadatos
     */
    #[payable]
    pub fn minar(
        &mut self,
        token_owner_id: ValidAccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.nTokenOnSale += 1;
        self.tokens.mint(
            self.tokens.owner_by_id.len().to_string(),
            token_owner_id,
            Some(token_metadata),
        )
    }
    #[payable]
    pub fn minarauction(
        &mut self,
        token_owner_id: ValidAccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.nTokenOnAuction += 1;
        self.tokens.mint(
            self.tokens.owner_by_id.len().to_string(),
            token_owner_id,
            Some(token_metadata),
        )
    }

    pub fn storage_byte_cost() -> Balance {
        env::storage_byte_cost()
    }
    /**
     * permite a los usuarios hacer la comprar de tokens
     *
     * @param TokenId {String} es el token que se desea comprar
     * @return TokenMetadata son los datos recien modificados
     */
    #[payable]
    pub fn comprar_nft(&mut self, token_id: TokenId) -> TokenMetadata {
        //asegurarnos de que el numero sea positivo y este dentro el rango de tokens minados
        //let token_id_u64 = token_id.parse::<u64>().unwrap();

        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() < self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
        //obtener los metadatos de ese token
        let mut metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();

        // si no cuenta con los fondos hacemos rollback
        let amount = env::attached_deposit();
        assert_eq!(
            metadata.price.as_ref().unwrap().parse::<u128>().unwrap(),
            amount,
            "fondos insuficientes"
        );
        assert_eq!(
            metadata.on_sale.as_ref().unwrap(),
            &true, 
            "no esta a la venta"
        );

        //revisar que este a la venta
        //obtener el dueño del token
        let token_owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        //obtener el creador del token
        let creator_id = metadata.creator.as_ref().unwrap();
        //obtener el comprador del token
        let buyer_id = &env::signer_account_id();
         //obtener la regalia,la comision de Nativo y el pagoa al autor del token
         let mut  res:f64=0.0;
         let mut  roy:f64=0.0;
         let mut  gains:f64=0.0;
         let mut  pay:f64=0.0;
         roy = amount as f64 *0.10;
         gains=amount as f64 *0.03;
         pay=amount as f64 *0.87;
          
        let my_string = metadata.price.as_ref().unwrap().to_string();  // `parse()` works with `&str` and `String`!
        let price_meta = my_string.parse::<u128>().unwrap();
        let regal = amount-price_meta ;

       
        // el dueñp no puede comprar su propio token
        assert_eq!(buyer_id == &token_owner_id, false, "eres el dueño del token ");
        //cambiar la metadata
        metadata.on_sale = Some(false);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &metadata));
        //transferir los nears
        //TODO: entender como consultar si la transferencia fue exitosa

        /*
        let promise = Promise::new(owner_id.clone())
            .transfer(amount)
            .function_call("tx_status_callback".into(), vec![], 0, 0);
        */
        Promise::new(token_owner_id.clone()).transfer(pay as  u128);
    
        //TODO: transferir la regalia del token
        Promise::new(creator_id.clone()).transfer(gains as u128);
        //TODO: transferir la regalia del token
        Promise::new(env::predecessor_account_id().clone()).transfer(roy as u128);
        //transferir el nft
        self.tokens
            .internal_transfer_unguarded(&token_id, &token_owner_id, buyer_id);

        //cambiar el numero de nfts disponibles
        self.nTokenOnSale -= 1;
        //retornar la metadata
        metadata
    }

    pub fn tx_status_callback(args: String) -> bool {
        log!(args);
        //env::promise_result(0)

        match env::promise_result(0) {
            PromiseResult::Successful(_x) => return true,
            _x => return false,
        };
    }

    /* segun as_return debe de regresar la transaccion
    pub fn get_result(&recipient_account: AccountId, amount: Balance) {
        let prepaid_gas = env::prepaid_gas();
        Promise::new(recipient_account).transfer(amount).as_return()
    }*/
    /**
     * nos permite revender un token y poner el precio del mismo
     *
     * @param TokenId   {String} es el token que queremos poner a la venta
     * @param price     {String} es el precio al que deseamos venderlo si es 0 el precio no se modifica
     *
     * @return TokenMetadata es la informacion del token que recien acabamos de modificar
     */
    pub fn revender(&mut self, token_id: TokenId, price: String) -> TokenMetadata {
        //comprobar que el token exista
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() < self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );

        //comprobar que el revendedor sea el owner
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        assert_eq!(
            env::signer_account_id() == owner_id,
            true,
            "no eres el dueño del token "
        );

        //obtener los metadatos de ese token
        let mut metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();

        //cambiar la metadata

        if price.trim().parse::<u128>().unwrap() > 0 {
            metadata.price = Some(price);
        }
        metadata.on_sale = Some(true);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &metadata));

        //cambiar el numero de nfts disponibles
        self.nTokenOnSale += 1;
        //retornar la metadata
        metadata
    }

    /**
     * nos permite revender un token y poner el precio del mismo
     *
     * @param TokenId   {String} es el token que queremos poner a la venta
     * @param price     {String} es el precio al que deseamos venderlo si es 0 el precio no se modifica
     *
     * @return TokenMetadata es la informacion del token que recien acabamos de modificar
     */
    pub fn quitar_del_market_place(&mut self, token_id: TokenId) -> TokenMetadata {
        //comprobar que el token exista
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() < self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );

        //comprobar que el revendedor sea el owner
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        assert_eq!(
            env::signer_account_id() == owner_id,
            true,
            "no eres el dueño del token "
        );

        //obtener los metadatos de ese token
        let mut metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();

        //cambiar la metadata

        metadata.on_sale = Some(false);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &metadata));

        //cambiar el numero de nfts disponibles
        self.nTokenOnSale -= 1;
        //retornar la metadata
        metadata
    }

    /**
     * nos permite filtrar los tokens a la venta
     *
     * @param from_index {U128} representa la pagina que deseamos obtener
     * @parama limiut {u64} representa la cantidad de tokens por pagina
     */
    pub fn obtener_pagina_v2(&self, from_index: usize, limit: u64) -> Vec<Token> {
        // no estoy segyri de como convierte  de U128 a u128
        let start_index: u128 = Some(from_index).map(|v| v as u128).unwrap_or_default();
        let limit = Some(limit).map(|v| v as usize).unwrap_or(usize::MAX);
        let inicioPag = start_index as usize * limit;

        assert_ne!(limit, 0, "Cannot provide limit of 0.");
        log!(
            "el valor de start_index es {}, el valor de limit {}, el valor de ntokens es {}",
            start_index,
            limit,
            inicioPag
        );
        let mut token_id="1"
        let mut counter: usize = 0;
        self.tokens
            .owner_by_id
            .iter()
            .filter(|x| {
                if self
                    .tokens
                    .token_metadata_by_id
                    .as_ref()
                    .and_then(|by_id| by_id.get(&x.0))
                    .unwrap()
                    .on_sale
                    .unwrap()
                {
                    counter += 1;
                    if counter > inicioPag {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .take(limit)
            .map(|(token_id, owner_id)| self.enum_get_token(owner_id, token_id))
            .collect()
    }

    pub fn get_on_sale_toks(&self) -> u64 {
        self.nTokenOnSale
    }
    // Self::pass_it(Self::get_sale_status(self, x.0.clone()), counter, ntokens)
    /* #[private]
        pub fn get_sale_status(&self, token_id: TokenId) -> bool {
            self.tokens
                .token_metadata_by_id
                .as_ref()
                .and_then(|by_id| by_id.get(&token_id))
                .unwrap()
                .on_sale
                .unwrap()
        }

        #[private]
        pub fn pass_it(on_sale: bool, mut counter: usize, ntokens: usize) -> bool {
            //si esta a la venta
            if on_sale {
                //lo contamos
                counter += 1;

                //si ya llegamos al inicio de la pagina agregamos el token
                if counter >= ntokens {
                    true
                } else {
                    false
                }
            }
            //si no esta a laventa no se agrega a la colecta
            else {
                false
            }
        }
    */
    //metodos obtenidos por las extensiones del standard

    //Metodo para mostrar la información de un token por su token id ntf_token
    //near view dev-1626753781082-15253478806026 nft_token '{""token_id"": ""0""}'

    //Metodo que retorna el numero total de tokens
    //near view dev-1626753781082-15253478806026 nft_total_supply

    //Metodo que retorna un array con el total de tokens determinado por un determinado rango (desde el index hasta  el limit)
    //near view dev-1626753781082-15253478806026 nft_tokens '{""from_index"": ""0"",""limit"": "1000"}'

    //Metodo que retorna los tokens nft del owner en un determinado rango (desde el index y hasta el limit)
    //se dejo solo para hacer coincidir el nombre del metodo con el de solidity ,pero se puede llamar directamente a nft_tokens_for_owner
    pub fn tokens_of(
        &self,
        account_id: ValidAccountId,
        from_index: U128,
        limit: u64,
    ) -> Vec<Token> {
        return self
            .tokens
            .nft_tokens_for_owner(account_id, Some(from_index), Some(limit));
    }
}

//obtener los metadatos del contrato
#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

// Macros con las implementaciones de las interfaces del token NFT en el contrato
near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
