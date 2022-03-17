use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract,Gas, log,serde_json::json, near_bindgen, AccountId, Promise, PromiseResult};
use std::collections::HashMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::env::BLOCKCHAIN_INTERFACE;

use std::str;
 

pub const TGAS: u64 = 1_000_000_000_000;
/// Gas for upgrading this contract on promise creation + deploying new contract.
pub const GAS_FOR_UPGRADE_SELF_DEPLOY: Gas = 30_000_000_000_000;

pub const GAS_FOR_UPGRADE_REMOTE_DEPLOY: Gas = 10_000_000_000_000;

near_sdk::setup_alloc!();
/// Balance is type for storing amounts of tokens.
pub type Balance = u128;
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldContract {
    contract_owner:AccountId,
    num_whitelist: u64,
    num_collections: u64,

    whitelist_contracts: HashMap<AccountId, ExternalContract>,
    whitelist_collections: HashMap<String, DataCollection>,
    market_contract_address: AccountId,
    market_contract_address_dev: AccountId,
    market_contract_treasury: AccountId,
    market_roy:f64,
    creator_roy:f64,
    owner_roy:f64,
    fee_comision:u128,

}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_owner:AccountId,
    num_whitelist: u64,
    num_collections: u64,

    whitelist_contracts: HashMap<AccountId, ExternalContract>,
    whitelist_collections: HashMap<String, DataCollection>,
    market_contract_address: AccountId,
    market_contract_address_dev: AccountId,
    market_contract_treasury: AccountId,
    market_roy:f64,
    creator_roy:f64,
    owner_roy:f64,
    fee_comision:u128,

}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Thegraphstructure {
    contract_name:String,
    collection:String,
    collection_id:String,
    token_id : String,
    owner_id : String,
    last_owner_id:String,
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
//structure for whitelist information
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ExternalContract {
    owner_id: AccountId,
    contract_name: String,
    contract_balance:u128,
}
//structure for whitelist information
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DataCollection {
    id:String,
    contract_address:AccountId,
    owner_id: AccountId,
    title: String,
    description:String,
    icon_media: String,
    banner_media:String,
}

//aqui van los nombres de los metodos que mandaremos llamar
#[ext_contract(ext_nft)]
trait NonFungibleToken {
    // change methods
    fn mint_token_ext(
        &mut self,
        token_owner_id: AccountId,
        collection_id: String,
        collection: String,
        token_metadata: TokenMetadata,
    );
    fn mint_token(
        &mut self,
        token_owner_id: AccountId,
        collection_id: String,
        collection: String,
        token_metadata: TokenMetadata,
    );
    fn buy_token_ext(&mut self, token_id: TokenId,collection_id: String, collection: String,amount:Balance);
    fn bid_token_ext(&mut self, token_id: TokenId,collection_id: String, collection: String,amount:Balance);
    fn close_bid_token_ext(&mut self, token_id: TokenId,collection_id: String, collection: String,status:bool);

    
    fn sell_token_ext(&mut self, token_id: TokenId, price:u128,collection_id: String, collection: String);
    fn remove_token_ext(&mut self, token_id: TokenId,collection_id: String, collection: String);

    fn set_actual_mint_fee(&mut self,mint_fee:u128);
    fn set_actual_roy_fee(&mut self,market_roy:u128,owner_roy:u128,creator_roy:u128);

    // view method
    
    fn get_actual_mint_fee(&self);
    fn get_actual_roy_fee(&self);
    fn nft_token(&self, token_id: String) -> Option<Token>;
    fn get_on_total_toks(&self) -> u64;
}
//los metodos existentes en el marketplace
#[ext_contract(ext_self)]
pub trait MyContract {
    fn get_promise_result(&self, method: String,complement: String) -> String;
    fn get_promise_result_for_info(&self, method: String) -> String;

    fn save_mint_ttg(&self, info: Thegraphstructure);
    fn save_buy_ttg(&self, info: Thegraphstructure);
    fn save_sell_ttg(&self, info: Thegraphstructure);
    fn save_remove_ttg(&self, info: Thegraphstructure);
    fn save_bid_ttg(&self, info: Thegraphstructure);
    fn save_close_bid_ttg(&self, info: Thegraphstructure,status:String);

    fn show_fee_ttg(&self, info: String);
    fn show_roy_ttg(&self, info: String);

    
   
    fn dontsave_ttg(&self, info: Thegraphstructure);
}

impl Default for Contract {
    fn default() -> Self {
        let def_contract = ExternalContract {
            owner_id: env::signer_account_id(),
            contract_name: "Nativo market".to_string(),
            contract_balance:0,
        };
        let def_hash = HashMap::from([(env::current_account_id(), def_contract)]);
        Self {
            contract_owner:env::signer_account_id(),
            num_whitelist: 0,
            num_collections: 0,

            whitelist_contracts: def_hash,
            whitelist_collections:HashMap::new(),
            market_contract_address: env::current_account_id(),
            market_contract_address_dev: env::current_account_id(),
            market_contract_treasury: env::current_account_id(),
            market_roy:0.03,
            creator_roy:0.87,
            owner_roy:0.10,
            fee_comision:100000000000000000000000,
        }
    }
}
#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    // Esta función incializa el contrato con los valores especificados en la metadata

    #[init]
    pub fn new_default_meta() -> Self {
        let def_contract = ExternalContract {
            owner_id: env::signer_account_id(),
            contract_name: "Nativo market".to_string(),
            contract_balance:0,
        };
        let def_hash = HashMap::from([(env::current_account_id(), def_contract)]);
        Self {
            contract_owner:env::signer_account_id(),
            num_whitelist: 0,
            num_collections: 0,

            whitelist_contracts: def_hash,
            whitelist_collections:HashMap::new(),
            market_contract_address: env::current_account_id(),
            market_contract_address_dev: env::current_account_id(),
            market_contract_treasury: env::current_account_id(),
            market_roy:0.03,
            creator_roy:0.87,
            owner_roy:0.10,
            fee_comision:100000000000000000000000,

        }
    }

    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let def_contract = ExternalContract {
            owner_id: env::signer_account_id(),
            contract_name: "Nativo minter".to_string(),
            contract_balance:0,
        };
        let def_hash = HashMap::from([("ntv-mint.near".to_string(), def_contract)]);
        Self {
            contract_owner:env::signer_account_id(),
            num_whitelist: 0,
            num_collections: 0,

            whitelist_contracts: def_hash,
            whitelist_collections:HashMap::new(),
            market_contract_address: env::current_account_id(),
            market_contract_address_dev: env::current_account_id(),
            market_contract_treasury: env::current_account_id(),
            market_roy:0.03,
            creator_roy:0.87,
            owner_roy:0.10,
            fee_comision:100000000000000000000000,

        }
    }
    pub fn get_market_contract(&self) {
        log!(
            "market contract : {}, owner_contract : {}",
             self.market_contract_address_dev,
             self.contract_owner
        );
    }
    fn pay_royalties(&self,amount:String,account_owner:AccountId,account_creator:AccountId){
         //obtener la regalia,la comision de Nativo y el pagoa al autor del token
         let creator_r:f64= amount.parse::<u128>().unwrap() as f64 * self.creator_roy;
         let market_r:f64= amount.parse::<u128>().unwrap() as f64 * self.market_roy;
         let owner_r:f64= amount.parse::<u128>().unwrap() as f64 * self.owner_roy;
         //transferir los nears
         //TODO: se le paga antiguo owner
         Promise::new(account_owner.clone().to_string()).transfer(owner_r as  u128);
         //TODO: se le paga al creador 
         Promise::new(account_creator.clone()).transfer(creator_r as u128);
        //TODO: la regalia del market se va al treasury
        Promise::new(self.market_contract_treasury.to_string()).transfer(market_r as u128);
    }
    fn store_bid_amount(&self,last_owner_id:AccountId,last_amount_bidder:String,amount:String){

             if last_amount_bidder.parse::<u128>().unwrap() > 0 {
                Promise::new(last_owner_id).transfer(last_amount_bidder.parse::<u128>().unwrap());
                Promise::new(self.market_contract_treasury.to_string()).transfer(amount.parse::<u128>().unwrap().clone()-last_amount_bidder.parse::<u128>().unwrap().clone() );

            }
        else {
            Promise::new(self.market_contract_treasury.to_string()).transfer(amount.parse::<u128>().unwrap() );
        }

    }
    
    #[payable]
    pub fn market_mint_generic(
        &mut self,
        address_contract: AccountId,
        token_owner_id: AccountId,
        collection_id: String,
        collection: String,
        token_metadata: TokenMetadata,
    ) -> Promise {
        let contract_exist = self.whitelist_contracts.get(&address_contract.clone());
        if contract_exist.is_none() {
            panic!(
                "This address {} is not approved to mint tokens!",
                address_contract.clone()
            );
        }

        self.is_payed_the_fee();
        //0.1
        //0.00009 --0.000009788084  se le envia al minero
        //0.099990
        let p = ext_nft::mint_token_ext(
            token_owner_id,
            collection_id,
            collection,
            token_metadata,
            &address_contract.to_string(), //  account_id as a parameter
            97600000000000000000000,       // yocto NEAR to attach
            100_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "mint".to_string(),
            "null".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0,                                             // yocto NEAR a ajuntar al callback
            30_000_000_000_000,                            // gas a ajuntar al callback
        ));
        log!("market ends here");
        p
    }
    #[payable]
    pub fn market_buy_generic(
        &mut self,
        address_contract: String,
        token_id: TokenId,
        collection_id: String,
        collection: String,
    ) -> Promise {
        let contract_exist = self.whitelist_contracts.get(&address_contract.clone());
        if contract_exist.is_none() {
            panic!(
                "This address {} is not approved to buy tokens!",
                address_contract.clone()
            );
        }
        //declaro el amount enviado
        let amount = env::attached_deposit();

        let p = ext_nft::buy_token_ext(
            token_id,
            collection_id,
            collection,
            amount,
            &address_contract.to_string(), //  account_id as a parameter
            0,       // yocto NEAR to attach
            100_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "buy".to_string(),
            "null".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0,                                             // yocto NEAR a ajuntar al callback
            30_000_000_000_000,                            // gas a ajuntar al callback
        ));
        log!("market ends here");
        p
    }
    #[payable]
    pub fn market_bid_generic(
        &mut self,
        address_contract: String,
        token_id: TokenId,
        collection_id: String,
        collection: String,
    ) -> Promise {
        let contract_exist = self.whitelist_contracts.get(&address_contract.clone());
        if contract_exist.is_none() {
            panic!(
                "This address {} is not approved to buy tokens!",
                address_contract.clone()
            );
        }
        //100000000000000000000000000
        //1e23
        let amount = env::attached_deposit();
        let p = ext_nft::bid_token_ext(
            token_id,
            collection_id,
            collection,
            amount,
            &address_contract.to_string(), //  account_id as a parameter
            0,       // yocto NEAR to attach
            100_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "bid".to_string(),
            "null".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0,                                             // yocto NEAR a ajuntar al callback
            30_000_000_000_000,                            // gas a ajuntar al callback
        ));
        log!("market ends here");
        p
    }
    #[payable]
    pub fn market_close_bid_generic(
        &mut self,
        address_contract: String,
        token_id: TokenId,
        collection_id: String,
        collection: String,
        status:bool
    ) -> Promise {
        let contract_exist = self.whitelist_contracts.get(&address_contract.clone());
        if contract_exist.is_none() {
            panic!(
                "This address {} is not approved to buy tokens!",
                address_contract.clone()
            );
        }
        self.is_payed_the_fee();

        let p = ext_nft::close_bid_token_ext(
            token_id,            
            collection_id,
            collection,
            status.clone(),
            &address_contract.to_string(), //  account_id as a parameter
            0,       // yocto NEAR to attach
            100_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "close_bid".to_string(),
            status.to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0,                                             // yocto NEAR a ajuntar al callback
            30_000_000_000_000,                            // gas a ajuntar al callback
        ));
        log!("market ends here");
        p
    }
     
    pub fn market_sell_generic(
        &mut self,
        address_contract: String,
        token_id: TokenId,
        price: String,
        collection_id: String,
        collection: String,
    ) -> Promise {
        let contract_exist = self.whitelist_contracts.get(&address_contract.clone());
        if contract_exist.is_none() {
            panic!(
                "This address {} is not approved to sell tokens!",
                address_contract.clone()
            );
        }

         
        let p = ext_nft::sell_token_ext(
            token_id,
            price.parse::<u128>().unwrap(),
            collection_id,
            collection,
            &address_contract.to_string(), //  account_id as a parameter
            0,       // yocto NEAR to attach
            100_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "sell".to_string(),
            "null".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0,                                             // yocto NEAR a ajuntar al callback
            30_000_000_000_000,                            // gas a ajuntar al callback
        ));
        log!("market ends here");
        p
    }
     
    pub fn market_remove_generic(
        &mut self,
        address_contract: String,
        token_id: TokenId,
        collection_id: String,
        collection: String,
    ) -> Promise {
        let contract_exist = self.whitelist_contracts.get(&address_contract.clone());
        if contract_exist.is_none() {
            panic!(
                "This address {} is not approved to remove tokens!",
                address_contract.clone()
            );
        }
         
        let p = ext_nft::remove_token_ext(
            token_id,
            collection_id,
            collection,
            &address_contract.to_string(), //  account_id as a parameter
            0,       // yocto NEAR to attach
            100_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "remove".to_string(),
            "null".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0,                                             // yocto NEAR a ajuntar al callback
            30_000_000_000_000,                            // gas a ajuntar al callback
        ));
        log!("market ends here");
        p
    }
    //Método para agregar una nueva coleccions
    #[payable]
    pub fn add_user_collection(
        &mut self,
        address_contract: AccountId,
        address_collection_owner: AccountId,
        title: String,
        descrip: String,
        mediaicon: String,
        mediabanner: String,
    ) {
        //validate that info isnt empty
        assert_eq!(address_contract.to_string().is_empty(),false,"the contract address cannot be empty");
        assert_eq!(address_collection_owner.clone().to_string().is_empty(),false,"the owner address cannot be empty");
        assert_eq!(title.is_empty(), false, "the title cannot be empty");
        assert_eq!(descrip.is_empty(), false, "the description cannot be empty");
        assert_eq!(mediaicon.is_empty(),false,"the media icon link cannot be empty");
        assert_eq!(mediabanner.is_empty(),false,"the media banners link  cannot be empty");
        //validate the amount send is correct
        self.is_payed_the_fee();
         //validate that the address_contract  exist in the white lists
        let contract_exist = self.whitelist_contracts.get(&address_contract.clone());
        if contract_exist.is_none() {
            panic!("This address {} is not approved to create collections!",address_contract.clone());
        }
        //validate that the collection doesnt exits in the whitelist_collections
        let unique_id_collecion= address_contract.clone()+&address_collection_owner.clone()+&self.num_collections.clone().to_string();
        let collection_exist=self.whitelist_collections.get(&unique_id_collecion.clone());
        if !collection_exist.is_none() {
            panic!("This collection {} already exist in the contract!",title.clone());
        }
        else {
            let new_col= DataCollection {
            id:self.num_collections.to_string(),
            contract_address:address_contract.clone(),
            owner_id: address_collection_owner.clone(),
            title: title.clone(),
            description:descrip.clone(),
            icon_media: mediaicon.clone(),
            banner_media:mediabanner.clone(),
            };
        self.whitelist_collections
        .insert(unique_id_collecion, new_col);
        
        
    
        env::log(
            json!({
            "id":self.num_collections.to_string(),
            "contract_address":address_contract.clone(),
            "owner_id": address_collection_owner.clone(),
            "title": title.clone(),
            "description":descrip.clone(),
            "icon_media": mediaicon.clone(),
            "banner_media":mediabanner.clone(),})
            .to_string()
            .as_bytes(),
        );
        self.num_collections += 1;
        }//end else
        

        
    }

    #[payable]
    pub fn add_new_ext_contract(
        &mut self,
        address_contract: AccountId,
        address_owner: AccountId,
        contract_name: String,
    ) {
         
        self.is_the_owner();
        // validate that the info sended isnt empty
        assert_eq!(
            address_contract.to_string().is_empty(),
            false,
            "the contract address cannot be empty"
        );
        assert_eq!(
            address_owner.to_string().is_empty(),
            false,
            "the owner address cannot be empty"
        );
        assert_eq!(contract_name.is_empty(), false, "the title cannot be empty");
        // validate that the attached sended is correct
       /* let amount = env::attached_deposit();
        assert_eq!(
            "5000000000000000000000000"
                .to_string()
                .parse::<u128>()
                .unwrap(),
            amount,
            "Wrong amount deposited,please check for the correct amount!"
        );*/
        // validate if the contract already exist,dont create a new one
        let contract_exist = self.whitelist_contracts.get(&address_contract.clone());
        if !contract_exist.is_none() {
            assert_eq!(
                contract_exist.unwrap().contract_name.is_empty(),
                true,
                "the contract already exist"
            );
        }
        // create a new contract structure
        let new_ext_contract = ExternalContract {
            owner_id: env::signer_account_id(),
            contract_name: contract_name.clone(),
            contract_balance:0,
        };
        //modify  and save the information
        self.num_whitelist += 1;

        self.whitelist_contracts
            .insert(address_contract.clone(), new_ext_contract);
        let cont = self.whitelist_contracts.get(&address_contract.clone());

        env::log(
            json!({
                "address_contract":address_contract,
                "owner_id":cont.unwrap().owner_id,
                "contract_name":cont.unwrap().contract_name})
            .to_string()
            .as_bytes(),
        );
         
    }
    // Método de procesamiento para promesa
    pub fn get_promise_result(&self, method: String,complement: String) {
        self.is_white_listed();
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                log!("falló el contracto externo");
                ()
            }
            PromiseResult::Successful(result) => {
                let value = str::from_utf8(&result).unwrap();
                log!("regreso al market");

                let mut newstring = str::replace(&value, "\\", "");
                newstring.remove(0);
                newstring.pop();
               
                let tg: Thegraphstructure = serde_json::from_str(&newstring).unwrap();  
             
                let a = "mint".to_string();
                let b = "buy".to_string();
                let c = "bid".to_string();
                let d = "sell".to_string();
                let e = "remove".to_string();
                let f = "close_bid".to_string();

                
                if method == a {
                    log!("se va a minar");
                    ext_self::save_mint_ttg(
                        tg,
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                } else if method == b {
                    log!("se va a comprar");
                    ext_self::save_buy_ttg(
                        tg,
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                }else if method == c {
                    log!("se va a ofertar");
                    ext_self::save_bid_ttg(
                       tg,
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                } else if method == d {
                    log!("se va a vender");
                    ext_self::save_sell_ttg(
                        tg,
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                } else if method == e {
                    log!("se va a remover");
                    ext_self::save_remove_ttg(
                        tg,
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                }else if method == f {
                    log!("se va a cerrar oferta");
                    ext_self::save_close_bid_ttg(
                        tg,
                        complement,
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                } 
            }
        }
    }
    // Método de procesamiento para promesa
    pub fn get_promise_result_for_info(&self, method: String) {
        self.is_white_listed();
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                log!("falló el contracto externo");
                ()
            }
            PromiseResult::Successful(result) => {
                let value = str::from_utf8(&result).unwrap();
                log!("regreso al market");
 
                
             
                let a = "get_fee".to_string();
                let b = "set_fee".to_string();
                let c = "get_roy_fees".to_string();
                let d = "set_roy_fees".to_string();
               /*     let e = "remove".to_string();
                let f = "close_bid".to_string();*/

                
                if method == a {
                     
                    ext_self::show_fee_ttg(
                        value.to_string(),
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                } else if method == b {
                    
                    ext_self::show_fee_ttg(
                        value.to_string(),
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                }else if method == c {
                   
                    ext_self::show_roy_ttg(
                       value.to_string(),
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                } else if method == d {
                    
                    ext_self::show_roy_ttg(
                        value.to_string(),
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                }/* else if method == e {
                    log!("se va a remover");
                    ext_self::save_remove_ttg(
                        value,
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                }else if method == f {
                    log!("se va a ofertar");
                    ext_self::save_close_bid_ttg(
                        value,
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                } */
            }
        }
    }
    //Métodos que lanzan un log a the graph
    pub fn save_mint_ttg(&self, info: Thegraphstructure) {
        
         // validate if the contract already exist,dont create a new one
         self.is_white_listed();  
          
        
        env::log(
            json!(info)
            .to_string()
            .as_bytes(),
        );
    }
    pub fn save_buy_ttg(&self, info: Thegraphstructure) {
        // validate if the contract already exist,dont create a new one
         self.is_white_listed();  
        
         
         
         self.pay_royalties(info.price.clone(), info.last_owner_id.clone(), info.creator.clone());
         
        env::log(
            json!(info)
            .to_string()
            .as_bytes(),
        );
    }
    pub fn save_bid_ttg(&self, info: Thegraphstructure) {
        // validate if the contract already exist,dont create a new one
         self.is_white_listed();  

        self.store_bid_amount(info.last_owner_id.clone(),info.lowestbid.clone(),info.highestbid.clone());

        env::log(
            json!(info)
            .to_string()
            .as_bytes(),
        );
    }
    pub fn save_sell_ttg(&self, info: Thegraphstructure) {
        // validate if the contract already exist,dont create a new one
         self.is_white_listed();  

          
        env::log(
            json!(info)
            .to_string()
            .as_bytes(),
        );
    }
    pub fn save_remove_ttg(&self, info: Thegraphstructure) {
        // validate if the contract already exist,dont create a new one
         self.is_white_listed();  

       
        env::log(
            json!(info)
            .to_string()
            .as_bytes(),
        );
    }
    pub fn save_close_bid_ttg(&self, info: Thegraphstructure,status:String) {
        // validate if the contract already exist,dont create a new one
         self.is_white_listed();  
            
         if status.parse::<bool>().unwrap() {
            if info.highestbid.parse::<u128>().unwrap_or_default() > 0 {
                self.pay_royalties(info.highestbid.clone(), info.last_owner_id.clone(), info.creator.clone());
         
            }
        }else{
            //En caso de no aceptar la oferta, transferir el dinero al ultimo bidder
            Promise::new(info.adressbidder.clone()).transfer(info.highestbid.clone().parse::<u128>().unwrap());
        }
        env::log(
            json!(info)
            .to_string()
            .as_bytes(),
        );
    }
     
    pub fn dontsave_ttg(&self, info: Thegraphstructure) {
        // validate if the contract already exist,dont create a new one
         self.is_white_listed();  
       
       
        env::log(
            json!(info)
            .to_string()
            .as_bytes(),
        );
   
    }
    //Métodos que lanzan un log a the graph acercaa de la DAO
    pub fn show_fee_ttg(&self, info: String) {
        
     // validate if the contract already exist,dont create a new one
         self.is_white_listed();     
       
       env::log(
           json!(info)
           .to_string()
           .as_bytes(),
       );
    }
    pub fn show_roy_ttg(&self, info: String) {
                
     // validate if the contract already exist,dont create a new one
         self.is_white_listed();  
        
        env::log(
            json!(info)
            .to_string()
            .as_bytes(),
        );
        }
     //modificar cuentas
    
   
    pub fn get_treasury_account(&self,) {
        // validate if the contract already exist,dont create a new one
        
        log!("the treasury is : {} ",self.market_contract_treasury);
    }
    //modificar cuentas
    pub fn set_treasury_account(&mut self,new_account:AccountId) {
        self.is_the_owner();
            //if the caller is the owner
        self.market_contract_treasury=new_account;
        log!("the new treasury is : {} ",self.market_contract_treasury)
    }
    pub fn get_owner_account(&self,) {
        // validate if the contract already exist,dont create a new one
            
        log!("the owner is : {} ",self.contract_owner);
    }
        //modificar cuentas
    pub fn set_owner_account(&mut self,new_account:AccountId) {
        self.is_the_owner();
            //if the caller is the owner
        self.contract_owner=new_account;
        log!("the new owner is : {} ",self.contract_owner);

    }

    //////////////////////////////
    /// datos de la fee al crear un nuevo token 
    pub fn market_get_actual_mint_fee(&self,) {
        log!("the fee comision  is:{}",self.fee_comision);
        
    }
    pub fn market_set_actual_mint_fee(&mut self,mint_fee:u128) {
        self.is_the_owner();
        self.fee_comision=mint_fee;
        
    }
    pub fn market_get_actual_roy_fee(&self) {
       
         
        log!("the roy for market is:{}",self.market_roy);
        log!("the roy for the owner is:{}",self.owner_roy);
        log!("the roy for the creator is:{}",self.creator_roy);

        
    }
    pub fn market_set_actual_roy_fee(& mut self,market_r:f64,owner_r:f64,creator_r:f64) {
       
       self.is_the_owner();
       let total=market_r.clone()+owner_r.clone()+creator_r.clone();
            assert_eq!(total!=1.0,true,"the total amount must be equal to 100%");
           self.market_roy=market_r;
           self.owner_roy=owner_r;
           self.creator_roy=creator_r;
            
           log!("the roy for market is:{}",self.market_roy);
           log!("the roy for the owner is:{}",self.owner_roy);
           log!("the roy for the creator is:{}",self.creator_roy); 
    }
        //modificar cuentas
  /*  pub fn set_owner_account(&mut self,new_account:AccountId) {
        // validate if the contract already exist,dont create a new one
        self.whitelist_contracts.get(&env::predecessor_account_id()).expect("the contract isnt approved");
        //if the caller is the owner
        self.contract_owner=new_account;
        log!("the new owner is : {} ",self.contract_owner);

    }*/

   fn is_the_owner(&self)   {
        //validate that only the owner contract add new contract address
        assert_eq!(
            self.contract_owner==env::predecessor_account_id(),
            true,
            "!the you are not the contract owner address¡"
        );
   }
   fn is_white_listed(&self)   {
         // validate if the contract already exist,dont create a new one
         self.whitelist_contracts.get(&env::predecessor_account_id()).expect("the contract isnt approved");
   }
   fn is_payed_the_fee(&self) {
    assert_eq!(self.fee_comision==env::attached_deposit(),true,"the comision wasnt payed");
    //send the comision to the treasury
    let amount = env::attached_deposit();
    Promise::new(self.market_contract_treasury.to_string()).transfer(amount );
   }
   //This method will be removed from the DAO
 /* pub fn new_methods(){
    log!("heres the new");
}*/
    /////////////////////////////////////////
    //////////////REMOTE UPGRADE
    /// ////////////////////////////
    
    #[cfg(target_arch = "wasm32")]
    pub fn upgrade(self) {
        self.is_the_owner();
        // assert!(env::predecessor_account_id() == self.minter_account_id);
        //input is code:<Vec<u8> on REGISTER 0
        //log!("bytes.length {}", code.unwrap().len());
        const GAS_FOR_UPGRADE: u64 = 20 * TGAS; //gas occupied by this fn
        const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";
        //after upgrade we call *pub fn migrate()* on the NEW CODE
        let current_id = env::current_account_id().into_bytes();
        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas() - env::used_gas() - GAS_FOR_UPGRADE;
        unsafe {
            BLOCKCHAIN_INTERFACE.with(|b| {
                // Load input (new contract code) into register 0
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .input(0);

                //prepare self-call promise
                let promise_id = b
                    .borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_create(current_id.len() as _, current_id.as_ptr() as _);

                //1st action, deploy/upgrade code (takes code from register 0)
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);

                // 2nd action, schedule a call to "migrate()".
                // Will execute on the **new code**
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_function_call(
                        promise_id,
                        migrate_method_name.len() as _,
                        migrate_method_name.as_ptr() as _,
                        0 as _,
                        0 as _,
                        0 as _,
                        attached_gas,
                    );
            });
        }
    }

    ///////////////////////////////////////////////////////
    /// //////////////////METODOS DE MIGRACIÖN
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        log!("reading state");
        let old_state: OldContract = env::state_read().expect("failed");
        Self {
            contract_owner:old_state.contract_owner,
            num_whitelist: old_state.num_whitelist,
            num_collections: old_state.num_collections,

            whitelist_contracts: old_state.whitelist_contracts,
            whitelist_collections:HashMap::new(),
            market_contract_address: old_state.market_contract_address,
            market_contract_address_dev: old_state.market_contract_address_dev,
            market_contract_treasury: env::current_account_id(),
            market_roy:old_state.market_roy,
            creator_roy:old_state.creator_roy,
            owner_roy:old_state.owner_roy,
            fee_comision:old_state.fee_comision,

        }
    }
    /////////////////////METODOS DE MIGRACIÖN
    ///////////////////////////////////////////////////////
}
