
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::sync::{Mutex};
use lazy_static::lazy_static;
use near_sdk::collections::LazyOption;
 
use substring::Substring;
use std::collections::HashMap;
use near_sdk::json_types::{ValidAccountId,Base64VecU8};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,Gas,ext_contract,PromiseResult
};
use std::str;
use serde_json::json;
use std::convert::TryInto;


near_sdk::setup_alloc!();
/// Balance is type for storing amounts of tokens.
pub type Balance = u128;



#[derive(BorshDeserialize, BorshSerialize )]
pub struct OldContract {
    num_whitelist: u64,
    whitelist_contracts:HashMap<u64,String>,
    market_contract_address:AccountId,
    market_contract_address_dev:AccountId,

}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize )]
pub struct Contract {
    num_whitelist: u64,
    whitelist_contracts:HashMap<u64,String >,
    market_contract_address:AccountId,
    market_contract_address_dev:AccountId,

}

 
//aqui van los nombres de los metodos que mandaremos llamar
#[ext_contract(ext_nft)]
trait NonFungibleToken {
    // change methods
    fn mint_token_ext(&mut self,  token_owner_id: ValidAccountId,collection:String,token_metadata: TokenMetadata);
    fn mint_token(&mut self,  token_owner_id: ValidAccountId,collection:String,token_metadata: TokenMetadata);
    fn buy_token_ext(&mut self,token_id:TokenId,collection:String);
    fn sell_token_ext(&mut self,token_id:TokenId,price:String,collection:String);
    fn remove_token_ext(&mut self,token_id:TokenId,collection:String);

    // view method
    fn nft_token(&self, token_id: String) -> Option<Token>;
    fn get_on_total_toks(&self) -> u64;
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn getPromiseResult(&self,method:String) -> String;
    fn saveMintTTG(&self,info : String);
    fn saveBuyTTG(&self,info : String);
    fn saveSellTTG(&self,info : String);
    fn saveRemoveTTG(&self,info : String);
    fn DontsaveTTG(&self,info : String);



}
impl Default for Contract {
    
    fn default( ) -> Self {
      
     let meta =NFTContractMetadata {
         spec: NFT_METADATA_SPEC.to_string(),
         name: "Nativo Marketplace".to_string(),
         symbol: "NTV".to_string(),
         icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
         base_uri: None,
         reference: None,
         reference_hash: None,
     };
     Self {
         num_whitelist: 0,
         whitelist_contracts:HashMap::new(),
         market_contract_address:"nativov2.near".to_string(),
         market_contract_address_dev:"dev-1644433845094-13612285357489".to_string(),

     }   }
}
 
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

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
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    // Esta función incializa el contrato con los valores especificados en la metadata

    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Nativo Marketplace".to_string(),
                symbol: "NTV".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
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
            num_whitelist: 0,
            whitelist_contracts:HashMap::new(),
            market_contract_address:"nativov2.near".to_string(),
            market_contract_address_dev:"dev-1644433845094-13612285357489".to_string(),

        }
    }
   


    pub fn my_method(&self) -> Promise {
        ext_nft::get_on_total_toks(
            &"nativov2.near".to_string(), // ft_balance_of takes an account_id as a parameter
            0, // yocto NEAR to attach
            50_000_000_000_000 // gas to attach
        )
    }
    #[payable]
    pub fn market_mint_generic(& mut self,contractaddress: String,token_owner_id: ValidAccountId,collection:String,token_metadata: TokenMetadata) -> Promise {
     let p=ext_nft::mint_token_ext(
            token_owner_id,
            collection,
            token_metadata,
            &contractaddress.to_string(), //  account_id as a parameter
            env::attached_deposit(), // yocto NEAR to attach
            30_000_000_000_000 // gas to attach
        )   .then(ext_self::getPromiseResult(
            "mint".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0, // yocto NEAR a ajuntar al callback
            30_000_000_000_000 // gas a ajuntar al callback
        ));   
        log!("market ends here");
    p
    }
    #[payable]
    pub fn market_buy_generic(& mut self,contractaddress: String,token_id: TokenId,collection:String) -> Promise {
     let p=ext_nft::buy_token_ext(
            token_id,
            collection,
            &contractaddress.to_string(), //  account_id as a parameter
            env::attached_deposit(), // yocto NEAR to attach
            30_000_000_000_000 // gas to attach
        )   .then(ext_self::getPromiseResult(
            "buy".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0, // yocto NEAR a ajuntar al callback
            30_000_000_000_000 // gas a ajuntar al callback
        ));   
        log!("market ends here");
    p
    }
    #[payable]
    pub fn market_sell_generic(& mut self,contractaddress: String,token_id: TokenId,price:String,collection:String) -> Promise {
     let p=ext_nft::sell_token_ext(
            token_id,
            price,
            collection,
            &contractaddress.to_string(), //  account_id as a parameter
            env::attached_deposit(), // yocto NEAR to attach
            30_000_000_000_000 // gas to attach
        )   .then(ext_self::getPromiseResult(
            "sell".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0, // yocto NEAR a ajuntar al callback
            30_000_000_000_000 // gas a ajuntar al callback
        ));   
        log!("market ends here");
    p
    }
    #[payable]
    pub fn market_remove_generic(& mut self,contractaddress: String,token_id: TokenId,collection:String) -> Promise {
     let p=ext_nft::remove_token_ext(
            token_id,
            collection,
            &contractaddress.to_string(), //  account_id as a parameter
            env::attached_deposit(), // yocto NEAR to attach
            30_000_000_000_000 // gas to attach
        )   .then(ext_self::getPromiseResult(
            "remove".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0, // yocto NEAR a ajuntar al callback
            30_000_000_000_000 // gas a ajuntar al callback
        ));   
        log!("market ends here");
    p
    }
    //Método para agregar una nueva coleccions
    #[payable]
    pub fn Add_user_collection(&mut self,contr:ValidAccountId,addressowner:ValidAccountId,title:String,descrip:String,mediaicon:String,mediabanner:String)  {
        log!("{},{},{},{},{},{}",contr,addressowner,title,descrip,mediaicon,mediabanner);
    }
   // Método de procesamiento para promesa
    pub fn getPromiseResult(&self,method:String )  {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {log!("falló el contracto externo");()},
            PromiseResult::Successful(result) => {
                let value = str::from_utf8(&result).unwrap();
                log!("regreso al market" );
                
                let a="mint".to_string();
                let b="buy".to_string();
                let c="sell".to_string();
                let d="remove".to_string();
               

                match method {
                    a => ext_self::saveMintTTG(value.to_string(),&self.market_contract_address_dev.to_string(),0,10_000_000_000_000),
                    b => ext_self::saveBuyTTG(value.to_string(),&self.market_contract_address_dev.to_string(),0,10_000_000_000_000),
                    c => ext_self::saveSellTTG(value.to_string(),&self.market_contract_address_dev.to_string(),0,10_000_000_000_000),
                    d => ext_self::saveRemoveTTG(value.to_string(),&self.market_contract_address_dev.to_string(),0,10_000_000_000_000),

                    
                };
                  

               // return value.to_string();
            }
        }
        
    }

    //Métodos que lanzan un log a the graph
    pub fn saveMintTTG(&self ,info :String )   {
        let res = str::replace(&info.to_string(),"\"","");
       log!("{}",res);
    }
    pub fn saveBuyTTG(&self ,info :String )   {
        let res = str::replace(&info.to_string(),"\"","");
       log!("{}",res);
    }
    pub fn saveSellTTG(&self ,info :String )   {
        let res = str::replace(&info.to_string(),"\"","");
       log!("{}",res);
    }
    pub fn saveRemoveTTG(&self ,info :String )   {
        let res = str::replace(&info.to_string(),"\"","");
       log!("{}",res);
    }
    pub fn DontsaveTTG(&self ,info :String )   {
         
       log!("{}",info);
    }
     
    
  
///////////////////////////////////////////////////////
    /// //////////////////METODOS DE MIGRACIÖN
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
       
        Self {
           
         num_whitelist: 0,
         whitelist_contracts:HashMap::new(),
         market_contract_address:"nativov2.near".to_string(),
         market_contract_address_dev:"dev-1644433845094-13612285357489".to_string(),

        }
    }
/////////////////////METODOS DE MIGRACIÖN
///////////////////////////////////////////////////////

}
    

 
 
 
 
 
 