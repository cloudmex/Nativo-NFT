use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, log, near_bindgen, AccountId, Promise, PromiseResult};
use std::collections::HashMap;
use std::str;

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
}

//structure for whitelist information
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ExternalContract {
    owner_id: AccountId,
    contract_name: String,
}
//structure for whitelist information
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DataCollection {
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
    fn buy_token_ext(&mut self, token_id: TokenId,collection_id: String, collection: String);
    fn bid_token_ext(&mut self, token_id: TokenId,collection_id: String, collection: String);
    
    fn sell_token_ext(&mut self, token_id: TokenId, price: String,collection_id: String, collection: String);
    fn remove_token_ext(&mut self, token_id: TokenId,collection_id: String, collection: String);
    // view method
    fn nft_token(&self, token_id: String) -> Option<Token>;
    fn get_on_total_toks(&self) -> u64;
}
//los metodos existentes en el marketplace
#[ext_contract(ext_self)]
pub trait MyContract {
    fn get_promise_result(&self, method: String) -> String;
    fn save_mint_ttg(&self, info: String);
    fn save_buy_ttg(&self, info: String);
    fn save_sell_ttg(&self, info: String);
    fn save_remove_ttg(&self, info: String);
    fn save_bid_ttg(&self, info: String);

    fn dontsave_ttg(&self, info: String);
}

impl Default for Contract {
    fn default() -> Self {
        let def_contract = ExternalContract {
            owner_id: env::signer_account_id(),
            contract_name: "Nativo minter".to_string(),
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
            contract_name: "Nativo minter".to_string(),
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
        }
    }

    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let def_contract = ExternalContract {
            owner_id: env::signer_account_id(),
            contract_name: "Nativo minter".to_string(),
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
        }
    }
    pub fn get_market_contract(&self) {
        log!(
            "market contract : {}, owner_contract : {}",
             self.market_contract_address_dev,
             self.contract_owner
        );
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
        let p = ext_nft::mint_token_ext(
            token_owner_id,
            collection_id,
            collection,
            token_metadata,
            &address_contract.to_string(), //  account_id as a parameter
            env::attached_deposit(),       // yocto NEAR to attach
            30_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "mint".to_string(),
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
        let p = ext_nft::buy_token_ext(
            token_id,
            collection_id,
            collection,
            &address_contract.to_string(), //  account_id as a parameter
            env::attached_deposit(),       // yocto NEAR to attach
            30_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "buy".to_string(),
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

        let p = ext_nft::bid_token_ext(
            token_id,
            collection_id,
            collection,
            &address_contract.to_string(), //  account_id as a parameter
            env::attached_deposit(),       // yocto NEAR to attach
            30_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "bid".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0,                                             // yocto NEAR a ajuntar al callback
            30_000_000_000_000,                            // gas a ajuntar al callback
        ));
        log!("market ends here");
        p
    }
    #[payable]
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
            price,
            collection_id,
            collection,
            &address_contract.to_string(), //  account_id as a parameter
            env::attached_deposit(),       // yocto NEAR to attach
            30_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "sell".to_string(),
            &self.market_contract_address_dev.to_string(), // el mismo contrato local
            0,                                             // yocto NEAR a ajuntar al callback
            30_000_000_000_000,                            // gas a ajuntar al callback
        ));
        log!("market ends here");
        p
    }
    #[payable]
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
            env::attached_deposit(),       // yocto NEAR to attach
            30_000_000_000_000,            // gas to attach
        )
        .then(ext_self::get_promise_result(
            "remove".to_string(),
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
        let amount = env::attached_deposit();
        assert_eq!("100000000000000000000000".to_string().parse::<u128>().unwrap(),amount,"Cantidad incorrecta,verifica el costo exacto!");
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
            contract_address:address_contract.clone(),
            owner_id: address_collection_owner.clone(),
            title: title.clone(),
            description:descrip.clone(),
            icon_media: mediaicon.clone(),
            banner_media:mediabanner.clone(),
            };
        self.whitelist_collections
        .insert(unique_id_collecion, new_col);
        
        
        log!(
            "{},{},{},{},{},{},{}",
            address_contract, 
            address_collection_owner,
            title,
            descrip,
            mediaicon,
            mediabanner,
            self.num_collections.clone()
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
        //validate that only the owner contract add new contract address
        assert_eq!(
            self.contract_owner==env::signer_account_id(),
            true,
            "!the you are not the contract owner address¡"
        );
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
        let amount = env::attached_deposit();
        assert_eq!(
            "5000000000000000000000000"
                .to_string()
                .parse::<u128>()
                .unwrap(),
            amount,
            "Wrong amount deposited,please check for the correct amount!"
        );
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
        };
        //modify  and save the information
        self.num_whitelist += 1;

        self.whitelist_contracts
            .insert(address_contract.clone(), new_ext_contract);
        let cont = self.whitelist_contracts.get(&address_contract.clone());

        log!(
            "{},{},{}",
            address_contract,
            cont.unwrap().owner_id,
            cont.unwrap().contract_name
        );
    }
    // Método de procesamiento para promesa
    pub fn get_promise_result(&self, method: String) {
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

                let a = "mint".to_string();
                let b = "buy".to_string();
                let c = "bid".to_string();
                let d = "sell".to_string();
                let e = "remove".to_string();

                if method == a {
                    log!("se va a minar");
                    ext_self::save_mint_ttg(
                        value.to_string(),
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                } else if method == b {
                    log!("se va a comprar");
                    ext_self::save_buy_ttg(
                        value.to_string(),
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                }else if method == c {
                    log!("se va a ofertar");
                    ext_self::save_bid_ttg(
                        value.to_string(),
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                } else if method == d {
                    log!("se va a vender");
                    ext_self::save_sell_ttg(
                        value.to_string(),
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                } else if method == e {
                    log!("se va a remover");
                    ext_self::save_remove_ttg(
                        value.to_string(),
                        &self.market_contract_address_dev.to_string(),
                        0,
                        10_000_000_000_000,
                    );
                }
            }
        }
    }
    //Métodos que lanzan un log a the graph
    pub fn save_mint_ttg(&self, info: String) {
        
         // validate if the contract already exist,dont create a new one
        self.whitelist_contracts.get(&env::predecessor_account_id()).expect("the contract isnt approved");
          
        let res = str::replace(&info.to_string(), "\"", "");
        log!("{}", res);
    }
    pub fn save_buy_ttg(&self, info: String) {
        // validate if the contract already exist,dont create a new one
        self.whitelist_contracts.get(&env::predecessor_account_id()).expect("the contract isnt approved");

        let res = str::replace(&info.to_string(), "\"", "");
        log!("{}", res);
    }
    pub fn save_bid_ttg(&self, info: String) {
        // validate if the contract already exist,dont create a new one
        self.whitelist_contracts.get(&env::predecessor_account_id()).expect("the contract isnt approved");

        let res = str::replace(&info.to_string(), "\"", "");
        log!("{}", res);
    }
    pub fn save_sell_ttg(&self, info: String) {
        // validate if the contract already exist,dont create a new one
        self.whitelist_contracts.get(&env::predecessor_account_id()).expect("the contract isnt approved");

        let res = str::replace(&info.to_string(), "\"", "");
        log!("{}", res);
    }
    pub fn save_remove_ttg(&self, info: String) {
        // validate if the contract already exist,dont create a new one
        self.whitelist_contracts.get(&env::predecessor_account_id()).expect("the contract isnt approved");

        let res = str::replace(&info.to_string(), "\"", "");
        log!("{}", res);
    }
    pub fn dontsave_ttg(&self, info: String) {
        // validate if the contract already exist,dont create a new one
        self.whitelist_contracts.get(&env::predecessor_account_id()).expect("the contract isnt approved");

        log!("{}", info);
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
        }
    }
    /////////////////////METODOS DE MIGRACIÖN
    ///////////////////////////////////////////////////////
}
