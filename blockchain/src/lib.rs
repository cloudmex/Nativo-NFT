
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

// use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::sync::{Mutex};
use lazy_static::lazy_static;
use near_sdk::collections::LazyOption;
//use std::time::{Duration, Instant};
//use std::thread::sleep;
use substring::Substring;
 


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
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_total_tokens: u64,
    n_token_on_sale: u64,
    n_token_on_auction: u64,
    n_chunks: u64,
    tokenHM:HashMap<TokenId, Vec<String>>,
    tk_chunk:Vec<HashMap<TokenId, Vec<String>>>,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize )]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_total_tokens: u64,
    n_token_on_sale: u64,
    n_token_on_auction: u64,
    n_chunks: u64,
    tokenHM:HashMap<TokenId, Vec<String>>,
    tk_chunk:Vec<HashMap<TokenId, Vec<String>>>,
}
 
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Meta {
    token_id : String,
    owner_id : String,
    title : String,
    description : String,
    media : String,
    culture : String,
    country : String,
    creator : String,
    price : String,
    on_sale: bool, // sale status
    on_auction: bool, //auction status
    adressbidder: String,
    highestbidder: String,
    lowestbidder: String,
    expires_at: String,
    starts_at: String,
}
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Extras {
    culture : String,
    country : String,
    creator : String,
    price : String,
    on_sale: bool, // sale status
    on_auction: bool, //auction status
    adressbidder: String,
    highestbidder: String,
    lowestbidder: String,
    expires_at: String,
    starts_at: String,
}
 
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Thegraphstructure {
    contract_name:String,
    colecction:String,
    token_id : String,
    owner_id : String,
    title : String,
    description : String,
    media : String,
    creator : String,
    price : String,
    status: String, // sale status
    adressbidder: String,
    highestbid: String,
    lowestbid: String,
    expires_at: String,
    starts_at: String,
    extra:String,
}

lazy_static! {
    static ref USER_TOKEN_HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref CONV_MAP: HashMap<String, String> = {
        let mut map = HashMap::new();
        
        map
    };
   

}
//aqui van los nombres de los metodos que mandaremos llamar
#[ext_contract(ext_nft)]
trait NonFungibleToken {
    // change methods
    fn nft_mint_token_ext(&mut self,  token_owner_id: ValidAccountId,colecction:String,token_metadata: TokenMetadata);
    fn nft_mint_token(&mut self,  token_owner_id: ValidAccountId,colecction:String,token_metadata: TokenMetadata);
    fn nft_buy_token_ext(&mut self,token_id:TokenId);
    // view method
    fn nft_token(&self, token_id: String) -> Option<Token>;
    fn get_on_total_toks(&self) -> u64;
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn getPromiseResult(&self) -> String;
    fn saveToTheGraph(&self,info : String);

}
impl Default for Contract {
    
    fn default( ) -> Self {
      
     let meta =NFTContractMetadata {
         spec: NFT_METADATA_SPEC.to_string(),
         name: "Nativo NFT".to_string(),
         symbol: "NTV".to_string(),
         icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
         base_uri: None,
         reference: None,
         reference_hash: None,
     };
     Self {
     
         tokens:NonFungibleToken::new(
             StorageKey::NonFungibleToken,
             env::signer_account_id().try_into().unwrap(),
             Some(StorageKey::TokenMetadata),
             Some(StorageKey::Enumeration),
             Some(StorageKey::Approval),

         ) ,
         metadata: LazyOption::new(StorageKey::Metadata, Some(&meta)),
         n_total_tokens: 0,
         n_token_on_sale: 0,
         n_token_on_auction: 0,
         n_chunks: 0,
         tokenHM:HashMap::new(),
         tk_chunk:Vec::new(),
         
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
                name: "Nativo NFT".to_string(),
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
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            n_total_tokens: 0,
            n_token_on_sale: 0,
            n_token_on_auction: 0,
            n_chunks: 0,
            tokenHM:HashMap::new(),
            tk_chunk:Vec::new(),
        }
    }
    
    pub fn enum_get_token(&self, owner_id: AccountId, token_id: TokenId) -> Token {
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


    pub fn my_method(&self) -> Promise {
        ext_nft::get_on_total_toks(
            &"nativov2.near".to_string(), // ft_balance_of takes an account_id as a parameter
            0, // yocto NEAR to attach
            50_000_000_000_000 // gas to attach
        )
    }
    #[payable]
    pub fn market_mint_generic(& mut self,contractaddress: String,token_owner_id: ValidAccountId,colecction:String,token_metadata: TokenMetadata) -> Promise {
     let p=ext_nft::nft_mint_token_ext(
            token_owner_id,
            colecction,
            token_metadata,
            &contractaddress.to_string(), //  account_id as a parameter
            env::attached_deposit(), // yocto NEAR to attach
            30_000_000_000_000 // gas to attach
        )   .then(ext_self::getPromiseResult(
            &"nativov2.near", // el mismo contrato local
            0, // yocto NEAR a ajuntar al callback
            30_000_000_000_000 // gas a ajuntar al callback
        ));   
        log!("market ends here");
    p
    }
    #[payable]
    pub fn market_buy_generic(& mut self,contractaddress: String,token_id: TokenId) -> Promise {
     let p=ext_nft::nft_buy_token_ext(
            token_id,
            &contractaddress.to_string(), //  account_id as a parameter
            env::attached_deposit(), // yocto NEAR to attach
            30_000_000_000_000 // gas to attach
        )   .then(ext_self::getPromiseResult(
            &"nativov2.near", // el mismo contrato local
            0, // yocto NEAR a ajuntar al callback
            30_000_000_000_000 // gas a ajuntar al callback
        ));   
        log!("market ends here");
    p
    }
    #[payable]
    pub fn Add_user_collection(&mut self,contr:ValidAccountId,addressowner:ValidAccountId,title:String,descrip:String)  {
       

        log!("{},{},{},{}",contr,addressowner,title,descrip);
    }
    #[payable]
    pub fn saveBuyToTheGraph(&mut self,contr:ValidAccountId,addressowner:ValidAccountId,title:String,descrip:String)  {
       

        log!("{},{},{},{}",contr,addressowner,title,descrip);
    }
    pub fn getPromiseResult(&self )  {
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
                //  Contract::getlog2(self,value.to_string());

                 let p2 = ext_self::saveToTheGraph(
                    value.to_string(),
                    &"nativov2.near",
                    0,
                    10_000_000_000_000
                ); 

               // return value.to_string();
            }
        }
        
    }

    
    pub fn saveToTheGraph(&self ,info :String )   {
        let res = str::replace(&info.to_string(),"\"","");
       log!("{}",res);
    }
    /* pub fn register_NFT(&self, graphdata: Thegraphstructure){

      log!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",graphdata.contract_name.to_string(),&graphdata.token_id.to_string(),&graphdata.owner_id.to_string(),&graphdata.title.to_string(),&graphdata.description.to_string(),&graphdata.media.to_string(),&graphdata.creator.to_string(),&graphdata.price.to_string(),&graphdata.status.to_string(),&graphdata.adressbidder.to_string(),&graphdata.highestbid.to_string(),&graphdata.lowestbid.to_string(),&graphdata.expires_at.to_string(),&graphdata.starts_at.to_string(),&graphdata.extra.to_string(),&graphdata.colecction.to_string());

     } */
    
  
///////////////////////////////////////////////////////
    /// //////////////////METODOS DE MIGRACIÖN
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
        log!("old state readed {}", old_state.n_total_tokens);
        Self {
           
            tokens:old_state.tokens,
            metadata: old_state.metadata,
            n_total_tokens:old_state.n_total_tokens,
            n_token_on_sale: old_state.n_token_on_sale,
            n_token_on_auction:old_state.n_token_on_auction,
            n_chunks: old_state.n_chunks,
            tokenHM:old_state.tokenHM,
            tk_chunk:old_state.tk_chunk,

        }
    }
/////////////////////METODOS DE MIGRACIÖN
///////////////////////////////////////////////////////

}
    

 
 
 
 
near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
/* 
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

   


    #[test]
    fn t_get_tokens_on_sale()  {  
         let account = "sub.nativov2.testnet".to_string();
        Promise::new(account).function_call(
             b"get_on_sale_toks".to_vec(),
             b"{}".to_vec(),
             0 as Balance,
             5_000_000_000_000 as Gas) ;
        Ok("ok");
    }

     
}

 */