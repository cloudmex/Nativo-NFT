
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

// use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
///home/josecanales/Github/Nativo-NFT/blockchain/rust-contract/near-contract-standards-3.2.0/src/non_fungible_token/core/core_impl.rs
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::sync::{Mutex};
use lazy_static::lazy_static;
use near_sdk::collections::LazyOption;
//use std::time::{Duration, Instant};
//use std::thread::sleep;


use near_sdk::json_types::{ValidAccountId,Base64VecU8};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
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
    tokenHM:HashMap<TokenId, Vec<String>>,
 
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize )]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_total_tokens: u64,
    n_token_on_sale: u64,
    n_token_on_auction: u64,
    tokenHM:HashMap<TokenId, Vec<String> >,
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
 
 
 
lazy_static! {
    static ref USER_TOKEN_HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref CONV_MAP: HashMap<String, String> = {
        let mut map = HashMap::new();
        
        map
    };
   

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
         tokenHM:HashMap::new(),
         
         
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
            tokenHM:HashMap::new(),
            
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


    //all methods created 
      /**
     * permite minar nuevos tokens
     * @param token_owner_id {ValidAccountId} a quien le va a pertenecer el token
     * @param tokenMetadata {TokenMetadata} los metadatos
     */
    #[payable]
    pub fn minar(&mut self,token_owner_id: ValidAccountId,token_metadata: TokenMetadata) -> Token {
        let Contractaccount: AccountId = "nativoapp.testnet".parse().unwrap();
        let  account: ValidAccountId = Contractaccount.clone().try_into().unwrap();
       // log!("token_owner_id {} ,contr {} ",&token_owner_id ,&account.to_string());
       let token_id: TokenId =self.n_total_tokens.to_string();
       let mut info:Vec<String>=Vec::new();

       let mined= self.tokens.mint(
            token_id.clone(),
            account,   // token_owner_id,//cambiar por la direccion del contrato this.address
            Some(token_metadata.clone())
        ); //Retorna    x.token_id
            //una vez minada hacer un tranfer del contract al owner
             //transferir el nft
              
              log!("tokenid {}",&token_id);
          
        self.tokens
        .internal_transfer_unguarded(&token_id, &Contractaccount.to_string(), &token_owner_id.to_string());
        log!("tranfer done");
        self.n_total_tokens  +=1;
        self.n_token_on_sale += 1;

        let newextradata = str::replace(&token_metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();  

        //info[0] status codeinfo
        if extradatajson.on_sale{
            info.push("S".to_string());
        }
        else if extradatajson.on_auction {
            info.push("A".to_string());
        }
        //info[1] Creator
        
        info.push(extradatajson.creator.clone());
        //info[2] owner en este punto creador y owner son el mismo
        info.push(extradatajson.creator);
        //info[3] precio del token
        let priceint = (extradatajson.price.parse::<u128>().unwrap()) /1000000000000000000000000 as u128;
        info.push(priceint.to_string());
        //info[4] fecha creacion
        info.push(env::block_timestamp().to_string());
         
         //insertar nuevo token a Hashmap
         let mut _map =self.tokenHM.clone();
         _map.insert(token_id.clone(),info);
         self.tokenHM=_map.clone();

        let mut originaltoken = Contract::enum_get_token( &self,self.tokens.owner_by_id.get(&token_id.to_string()).unwrap(),token_id.clone());
        originaltoken
    }

    #[payable]
    pub fn comprar_nft(&mut self, token_id: TokenId,info:Vec<String>) -> TokenMetadata {
        let Contractaccount: AccountId = "nativoapp.testnet".parse().unwrap();

        //asegurarnos de que el numero sea positivo y este dentro el rango de tokens minados
        //let token_id_u64 = token_id.parse::<u64>().unwrap();
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
        //obtener los metadatos de ese token
        let mut originaltoken = self
            .tokens
            .token_metadata_by_id.as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
            
            let owner_id = self.tokens.owner_by_id.get(&token_id);
            let owner_value = owner_id.as_deref().unwrap_or("default string");
            let mut metadataextra = Contract::get_token(self, token_id.clone());
      
        let amount = env::attached_deposit();
        assert_eq!(
            metadataextra.price.parse::<u128>().unwrap(),
            amount,
            "Cantidad incorrecta,verifica el costo exacto!"
        );
        assert_eq!(
            metadataextra.on_sale,
            true, 
            "no esta a la venta"
        );
        
        //revisar que este a la venta
        //obtener el dueño del token
        //let token_owner_id = self.tokens.owner_by_id.get(token_id).unwrap();
        //obtener el creador del token
        let creator_id = metadataextra.creator;
        //obtener el comprador del token
        let buyer_id = &env::signer_account_id();

        // el dueñp no puede comprar su propio token
        assert_eq!(buyer_id == &owner_value, false, "eres el dueño del token ");
         //obtener la regalia,la comision de Nativo y el pagoa al autor del token
         let mut  res:f64=0.0;
         let mut  roy:f64=0.0;
         let mut  gains:f64=0.0;
         let mut  pay:f64=0.0;
         roy = amount as f64 *0.10;
         gains=amount as f64 *0.03;
         pay=amount as f64 *0.87;
          
        /* let my_string = metadataextra.price.as_ref().unwrap().to_string();  // `parse()` works with `&str` and `String`!
        let price_meta = my_string.parse::<u128>().unwrap();
        let regal = amount-price_meta ; */

        log!("{}",&originaltoken.extra.as_ref().unwrap().to_string());
       
        //cambiar la metadata
        
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
        //Se modifica el json
        extradatajson.on_sale = false;
        //  extradatajson: Extras = serde_json::to(&newextradata).unwrap();    
        log!("{}",&extradatajson.on_sale.to_string());
        // se convierte el Json a un String plano
        let mut extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
        log!("{}",&finalextrajson.to_string());
        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        //transferir los nears
        //TODO: entender como consultar si la transferencia fue exitosa

        /*
        let promise = Promise::new(owner_id.clone())
            .transfer(amount)
            .function_call("tx_status_callback".into(), vec![], 0, 0);
        */
        Promise::new(owner_value.clone().to_string()).transfer(pay as  u128);
        //TODO: transferir la regalia del token
        Promise::new(creator_id.clone()).transfer(roy as u128);
        //TODO: transferir la regalia del token
        Promise::new(Contractaccount.clone()).transfer(gains as u128);
        //transferir el nft
        self.tokens
            .internal_transfer_unguarded(&token_id, &owner_value.to_string(), buyer_id);

        //cambiar el numero de nfts disponibles
        self.n_token_on_sale -= 1;
         //insertar nuevo token a Hashmap
         let mut _map =self.tokenHM.clone();
         _map.insert(token_id.clone(),info);
         self.tokenHM=_map.clone();
        //retornar la metadata
        originaltoken
    }

   
    
    pub fn revender(&mut self, token_id: TokenId, price: String,info:Vec<String>) -> TokenMetadata {
        //comprobar que el token exista
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
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
        let mut originaltoken = self
            .tokens
            .token_metadata_by_id.as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
        //Se modifica el json
        assert_eq!(
            extradatajson.on_auction==true,
            false,
            "Lo sentimos,este token se encuentra en subasta y aun no termina!"
        );
        if price.trim().parse::<u128>().unwrap() > 0 {
            extradatajson.price =  price ;
        }
        extradatajson.on_sale =  true ;
        //  extradatajson: Extras = serde_json::to(&newextradata).unwrap();    
        log!("{}",&extradatajson.on_sale.to_string());
        // se convierte el Json a un String plano
        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
        log!("{}",&finalextrajson.to_string());
        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        //cambiar el numero de nfts disponibles
        self.n_token_on_sale += 1;
         //insertar nuevo token a Hashmap
         let mut _map =self.tokenHM.clone();
         _map.insert(token_id.clone(),info);
         self.tokenHM=_map.clone();
        //retornar la metadata
        originaltoken
    }

    pub fn subastar_nft(&mut self, token_id: TokenId,lowestbidder:String,starts_at:String,expires_at:String,info:Vec<String>){
     
      
        // Verificar  si existe:
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
            //recuperar el token
            let mut  originaltoken = self
              .tokens
              .token_metadata_by_id.as_ref()
              .and_then(|by_id| by_id.get(&token_id))
              .unwrap();
              //recuperar el owner del token
            let token_owner_id = self.tokens.owner_by_id.get(&token_id);
           //1.- Verificar que seas el owner del token 
            assert_eq!(token_owner_id != Some(env::signer_account_id().to_string()), false, "no eres el dueño del token ");
           
  
              //cambiar la metadata
                  
                  //se  reemplaza los ' por \" en un string plano                "'", "\""
                  let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
                  //el string plano se convierte a JSon
                  let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
                  //Se modifica el json
                  extradatajson.on_sale = false;
                  extradatajson.on_auction = true;
                  extradatajson.lowestbidder=lowestbidder.clone();
                  extradatajson.price=lowestbidder;
                  extradatajson.highestbidder=0.to_string();
                  extradatajson.expires_at=expires_at;
                  extradatajson.starts_at=starts_at; 
                  // se convierte el Json a un String plano
                  let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
                  let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
                 
                  originaltoken.extra = Some(finalextrajson);
                  //remplazamos la metadata
                  self.tokens
                      .token_metadata_by_id
                      .as_mut()
                      .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
         
                       //insertar nuevo token a Hashmap
                    let mut _map =self.tokenHM.clone();
                    _map.insert(token_id.clone(),info);
                    self.tokenHM=_map.clone();
                //cambiar el numero de nfts disponibles
                  
                  self.n_token_on_auction+=1;
    }
    
    #[payable]
    pub fn ofertar_subasta(&mut self, token_id: TokenId   ){
        let Contractaccount: AccountId = "nativoapp.testnet".parse().unwrap();
        let mut amountsended=env::attached_deposit();
        
        // Verificar  si existe:
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
            //recuperar el token
            let mut  originaltoken = self
              .tokens
              .token_metadata_by_id.as_ref()
              .and_then(|by_id| by_id.get(&token_id))
              .unwrap();
              //recuperar el owner del token
            let token_owner_id = self.tokens.owner_by_id.get(&token_id);
           //1.- Verificar que no seas el owner del token
            assert_eq!(token_owner_id == Some(env::signer_account_id().to_string()), false, "Eres el dueño del token ");
              //cambiar la metadata
                  //se  reemplaza los ' por \" en un string plano                "'", "\""
                  let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
                  //el string plano se convierte a JSon
                  let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
                  //Se modifica el json
                   //Validar sino ha finalizado  
                let expires_at_token=   extradatajson.expires_at.parse::<i64>().unwrap();
                //let local: DateTime = Local::now();
                
              //   let now = Instant::now();
               
            //   //  let noww= chrono::offset::Utc::now().timestamp() ;
                    log!("token date : {},today: {}",(expires_at_token as u64)*1000000,env::block_timestamp());  
             assert_eq!((expires_at_token as u64)*1000000
                        < env::block_timestamp(),
                           false,// self.finalizar_subasta(token_id.clone())
                             "la subasta ya terminó" );  
                             log!("low:{},high:{},send:{}", extradatajson.lowestbidder.parse::<u128>().unwrap(),extradatajson.highestbidder.parse::<u128>().unwrap(),amountsended);
                assert_eq!( extradatajson.lowestbidder.parse::<u128>().unwrap() > amountsended, false, "la cantidad enviada es menos que el minimo");
                assert_eq!( extradatajson.highestbidder.parse::<u128>().unwrap() >= amountsended, false, "la cantidad enviada es menor o igual que la ultima puja");
                                                
                  if extradatajson.lowestbidder.parse::<u128>().unwrap() <= amountsended
                         && extradatajson.highestbidder.parse::<u128>().unwrap() < amountsended {
                    if extradatajson.highestbidder.parse::<u128>().unwrap() > 0 {
                              //regresar el bid al singer anterior
                                Promise::new(extradatajson.adressbidder.clone().to_string()).transfer(extradatajson.highestbidder.parse::<u128>().unwrap());
                        }
                            //actualizar el nuevo  signer y bid
                                extradatajson.highestbidder =amountsended.to_string();
                                extradatajson.adressbidder=env::signer_account_id().to_string();
                                Promise::new(Contractaccount.clone().to_string()).transfer( amountsended.clone() );

                                // se convierte el Json a un String plano
                        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
                        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
                        originaltoken.extra = Some(finalextrajson);
                        //remplazamos la metadata
                        self.tokens
                            .token_metadata_by_id
                            .as_mut()
                            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
                        //cambiar el numero de nfts disponibles
                        // List<token ,List<signer,bid>>  lists de subasta -> lista de offertas
                        let mut bidding_info = HashMap::new();
                        let mut _map = USER_TOKEN_HASHMAP.lock().unwrap();
                        bidding_info.insert(env::signer_account_id().to_string(),amountsended.to_string());
                     //   _map.insert(token_id,bidding_info);
                         
                  }
                  
    }
    pub fn finalizar_subasta(&mut self, token_id: TokenId,info:Vec<String>) -> bool {
        let Contractaccount: AccountId = "nativoapp.testnet".parse().unwrap();
        // Verificar  si existe:
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
            //recuperar el token
            let mut  originaltoken = self
              .tokens
              .token_metadata_by_id.as_ref()
              .and_then(|by_id| by_id.get(&token_id))
              .unwrap();
              //recuperar el owner del token
            let token_owner_id = self.tokens.owner_by_id.get(&token_id);
            //se  reemplaza los ' por \" en un string plano                "'", "\""
            let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
            //el string plano se convierte a JSon
            let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
            //Validar si ha finalizado
           
              /*   assert_eq!( extradatajson.expires_at.parse::<i64>().unwrap()
            <
                 chrono::offset::Utc::now().timestamp()  , false,"la subasta aun no termina" );
            */
            let amount =extradatajson.highestbidder.parse::<f64>().unwrap();
            //si ya termino la subasta
            //pagamos las ganancias al al onwer anterior,ganancias al contrato y  regalias al creator
                     //obtener la regalia,la comision de Nativo y el pagoa al autor del token
                        let mut  res:f64=0.0;
                        let mut  roy:f64=0.0;
                        let mut  gains:f64=0.0;
                        let mut  pay:f64=0.0;
                        roy  = amount  *0.10;
                        gains= amount  *0.03;
                        pay  = amount  *0.87;
                    //transferir los nears
                    Promise::new( token_owner_id.as_ref().unwrap().to_string() ).transfer(pay as  u128);
                    //TODO: transferir la regalia del token
                    Promise::new(extradatajson.creator.clone()).transfer(roy as u128);
                    //TODO: transferir la regalia del token
                    Promise::new(Contractaccount.clone()).transfer(gains as u128);
                        //cambiamos el on sale/on auction a false
                        if(extradatajson.highestbidder.parse::<u128>().unwrap() == 0 as u128){
                            extradatajson.adressbidder=token_owner_id.as_ref().unwrap().to_string();
                        }
                        extradatajson.on_sale=false;
                        extradatajson.on_auction=false;
                        // se convierte el Json a un String plano
                    let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
                    let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
                    originaltoken.extra = Some(finalextrajson);
                    //remplazamos la metadata
                    self.tokens
                        .token_metadata_by_id
                        .as_mut()
                        .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
            //el costo/subastaminima no se modifica hasta que lo ponga a la venta/subasta
            //lo transferimos al nuevo onwer
            //
            self.tokens
            .internal_transfer_unguarded(&token_id, &token_owner_id.as_ref().unwrap().to_string(), &extradatajson.adressbidder.to_string());
            //insertar nuevo token a Hashmap
            let mut _map =self.tokenHM.clone();
            _map.insert(token_id.clone(),info);
            self.tokenHM=_map.clone();
            self.n_token_on_auction-=1;
        return false ;
    }

   
     
    //Genera un nuevo registro o actualiza uno existente
    pub fn inserthash(& mut self,token_id : TokenId ,info:Vec<String>) -> usize {
        //insertar nuevo token a Hashmap
        let mut _map =self.tokenHM.clone();
        _map.insert(token_id.clone(),info);
        self.tokenHM=_map.clone();
        return  self.tokenHM.len();
        
    } 
    //harcodea 2000 registros al hashmap
    pub fn fillhash(& mut self ) -> usize {
        //insertar nuevo token a Hashmap
        let mut _map =self.tokenHM.clone();
        let ends= _map.len().to_string().parse::<u64>().unwrap();
        for x in 0..55 {
             _map.insert(x.to_string(),vec!["S".to_string(),"dokxo.testnet".to_string(),"dokxo.testnet".to_string(),"2".to_string(),"1639897851".to_string()]);
        }
        self.tokenHM=_map.clone();
        return  self.tokenHM.len();
        
    } 
    //obtiene un registro del hashmap por key
    pub fn gethash(& self,tokens:String   )   {

        let mut _map = self.tokenHM.clone();
        
       let x = _map.get(&tokens);
       log!("value {:?}", x);
 
    }
    //elimina un registro por key 
    pub fn clearhash(& mut self,_token:String     )   {

        let mut _map = self.tokenHM.clone();
   
            _map.remove(&_token);
            self.tokenHM=_map.clone();

       
       log!("value {:?}",_map.len());
 
    } 
    //genera un nuevo hashmap
    pub fn resethash(& mut self  )   {
        let mut _map = self.tokenHM.clone();
        self.tokenHM=HashMap::new();
    } 
    
    #[payable]
    pub fn extraer_token(&mut self, token_id: TokenId){
         

        // Verificar  si existe:
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
            //recuperar el token
            let mut  originaltoken = self
              .tokens
              .token_metadata_by_id.as_ref()
              .and_then(|by_id| by_id.get(&token_id))
              .unwrap();
              //recuperar el owner del token
            let token_owner_id = self.tokens.owner_by_id.get(&token_id);
         /*    assert_eq!(
                env::signer_account_id() != token_owner_id.as_ref().unwrap().to_string(),
                false,
                "no eres el dueño del token "
            );  */
            let Contractaccount: AccountId = token_owner_id.as_ref().unwrap().clone();
            let  account: ValidAccountId = Contractaccount.clone().try_into().unwrap(); 
            let msj: Option<String> = Some("withdraw succesfully,enjoy it! :)".to_string());
            //let msj2: String = Some("withdraw succesfully,enjoy it! :)".to_string());
            //let apro: Option<u64> = Some(apro);
             //   self.tokens.nft_approve(token_id.clone(),account.clone(),msj.clone());
            self.tokens.nft_transfer_call(account, token_id, None,msj, "".to_string());
           log!("transfer done");
    }
    #[payable]
    pub fn aproved_token(&mut self, token_id: TokenId){
        let token_owner_id = self.tokens.owner_by_id.get(&token_id);    
        let Contractaccount: AccountId = token_owner_id.as_ref().unwrap().clone();
            let  account: ValidAccountId = Contractaccount.clone().try_into().unwrap(); 
            let msj: Option<String> = Some("withdraw succesfully,enjoy it! :)".to_string());
            let apro: Option<u64> = Some(0);
            self.tokens.nft_approve(token_id.clone(),account.clone(),msj.clone());
    }
      /// 
    pub fn quitar_del_market_place(&mut self, token_id: TokenId,info:Vec<String>) -> TokenMetadata {
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
         let mut originaltoken = self
         .tokens
         .token_metadata_by_id.as_ref()
         .and_then(|by_id| by_id.get(&token_id))
         .unwrap();
      
     //se  reemplaza los ' por \" en un string plano                "'", "\""
     let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
     //el string plano se convierte a JSon
     let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
     //Se modifica el json
     
     
     extradatajson.on_sale =  false ;
 
     // se convierte el Json a un String plano
     let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
     let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
    
     originaltoken.extra = Some(finalextrajson);
     //remplazamos la metadata
     self.tokens
         .token_metadata_by_id
         .as_mut()
         .and_then(|by_id| by_id.insert(&token_id, &originaltoken));

         //insertar nuevo token a Hashmap
         let mut _map =self.tokenHM.clone();
         _map.insert(token_id.clone(),info);
         self.tokenHM=_map.clone();

        //cambiar el numero de nfts disponibles
        self.n_token_on_sale += 1;
        //retornar la metadata
        originaltoken
    }


    pub fn storage_byte_cost() -> Balance {
        env::storage_byte_cost()
    }

    pub fn get_on_sale_toks(&self) -> u64 {
        self.n_token_on_sale
        //recorrer
    }
    //obtiene paginacion de los tokens en venta
    pub fn get_pagination_onsale(&self,tokens:u64) ->  Vec<u64> {
        let mut vectIDs = vec![];
        let mut _tokfound =0;
        let mut _map =self.tokenHM.clone();
      //  let ends= _map.len().to_string().parse::<u64>().unwrap();
        let mut i=0;
        let mut status =|x : Vec<_> | { i+=1; x[0] =="S"  };
        let mut toksfilted: Vec<u64> = _map.iter()
                .filter(|st| status(st.1.clone()))
                .map(|p| p.0.clone().parse::<u64>().unwrap() )
                .collect() ;
                toksfilted.sort();
           // log!("{:?}",toksfilted);
          vectIDs.push(0);
        for x in 0..toksfilted.clone().len() { 
          
                 _tokfound+=1;
                if _tokfound== tokens {   
                    vectIDs.push( toksfilted[x].clone() );  
                    _tokfound=0;  
                    }
            
           
            
            if _tokfound == tokens {break; }            
        }
        vectIDs
        //return toksfilted[0..25].to_vec();
    }
    //obtiene paginacion de los tokens en subasta
    pub fn get_pagination_onauction(&self,tokens:u64) ->  Vec<u64> {
        let mut vectIDs = vec![];
        let mut _tokfound =0;
        let mut _map =self.tokenHM.clone();
      //  let ends= _map.len().to_string().parse::<u64>().unwrap();
        let mut i=0;
        let mut status =|x : Vec<_> | { i+=1; x[0] =="A"  };
        let mut toksfilted: Vec<u64> = _map.iter()
                .filter(|st| status(st.1.clone()))
                .map(|p| p.0.clone().parse::<u64>().unwrap() )
                .collect() ;
                toksfilted.sort();
           // log!("{:?}",toksfilted);
          vectIDs.push(0);
        for x in 0..toksfilted.clone().len() { 
          
                 _tokfound+=1;
                if _tokfound== tokens {   
                    vectIDs.push( toksfilted[x].clone() );  
                    _tokfound=0;  
                    }
            
           
            
            if _tokfound == tokens {break; }            
        }
        vectIDs
    }

    
    pub fn get_on_auction_toks(&self) -> u64 {
        self.n_token_on_auction
    }
    pub fn get_on_auction_toksV2(&self) -> u64 {
        let mut _realonsale =0;

        let limit = self.get_on_total_toks();
        log!("tokans total {}",&limit);
        for x in 0..(limit-1) { 
                          
                let mut token =self.get_token(x.to_string().clone());
                if token.on_auction{       
                    _realonsale += 1;
                }
                if x== (limit-1) {
                    break;
                }
       }
        _realonsale
    }
    pub fn get_on_total_toks(&self) -> u64 {
        self.n_total_tokens
    }
    pub fn get_token(&self, token_id: TokenId) -> Meta {
        
        let mut metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();
        let token = Meta {
            token_id : token_id.to_string(),
            owner_id : owner_id.to_string(),
            title : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            media : metadata.media.as_ref().unwrap().to_string(),
            culture : extradatajson.culture,
            country : extradatajson.country,
            creator : extradatajson.creator,
            price : extradatajson.price,
            on_sale: extradatajson.on_sale, // sale status
            on_auction: extradatajson.on_auction, //auction status
            adressbidder: extradatajson.adressbidder,
            highestbidder: extradatajson.highestbidder,
            lowestbidder:extradatajson.lowestbidder,
            expires_at: extradatajson.expires_at,
            starts_at: extradatajson.starts_at,
        };
        token
    }
    
 
    //solo obtiene los tokenid que esten en onsale y retorna un vec de META
    pub fn obtener_pagina_on_sale(& self,tokens: u64,_start_index: u64, _minprice:u64,_maxprice:u64,_date:u64) -> Vec<Meta>  {
        //insertar nuevo token a Hashmap
        let mut _map =self.tokenHM.clone();
        let mut vectIDs = vec![];
        let mut vectMEta = vec![];
        let ends= _map.len().to_string().parse::<u64>();
        let mut _tokfound =0;
        let mut i=0;
        let mut toksfilted:Vec<u64> = vec![];
        //comienza el filtrado segun los parametros
        //(tokens:N , _start_index:N,status:"S" ,account:"tes", price:N ,date:N)

            // (25,0, "null",0,0) ->Gallery
            if _minprice==0 && _maxprice==0 && _date==0 {

                let mut status =|p:u64 ,x : Vec<_> | { i+=1; x[0] =="S"  };
                toksfilted = _map.iter()
                     .filter(|st| status(st.0.clone().parse::<u64>().unwrap() ,st.1.clone()))
                     .map(|p| p.0.clone().parse::<u64>().unwrap() )
                     .collect() ;
                     toksfilted.sort();

            } // (25,0,"S","null",2,0) ->Gallery y price
            else if _minprice>=0 &&   _date==0 {
                let mut status =|tokeni:u64 ,vect2 : Vec<String> | 
                { i+=1; vect2[0] =="S" && vect2[3].parse::<u64>().unwrap() >=_minprice  && vect2[3].parse::<u64>().unwrap() <=_maxprice };
                toksfilted = _map.iter()
                     .filter(|st| status(st.0.clone().parse::<u64>().unwrap() ,st.1.clone()))
                     .map(|p| p.0.clone().parse::<u64>().unwrap() )
                     .collect();
                     toksfilted.sort();
                  if toksfilted.is_empty() {
                     log!("No se encontraron tokens entre {} y {}",_minprice,_maxprice);
                     return  vectMEta
                  }   
                      
            }// (25,0,"S","null",0,2143234) ->Gallery y date
            else if _minprice==0 && _date>0 {

            }// (25,0,"null",2,2132433543) ->Gallery , price y date
            else if _minprice>0 && _date>0 {

            }


        
         
        
           
            // (25,0,"S","null",2,2132433543) ->Gallery , price y date
        // (25,0,"U","d.testnet",0,0) -> Gallery's artis
            // (25,0,"U","d.testnet",2,0) -> Gallery's artis y price
            // (25,0,"U","d.testnet",2,2234235443) -> Gallery's artis, price y date
        // (25,0,"A","null",0,0) ->Gallery auction
           // (25,0,"A","null",2,0) ->Gallery y price
            // (25,0,"A","null",2,2132433543) ->Gallery , price y date

         
      //  let ends= _map.len().to_string().parse::<u64>().unwrap();
        
      
        
        //este ciclo recupera los primeros
        for x in _start_index..ends.unwrap()  {
              _tokfound+=1;
                if _tokfound > tokens  {break;}      
            let tok = toksfilted[x as usize];
            //log!("{}",tok.clone());

                     vectIDs.push(tok );
                
        }  

        let endmeta = vectIDs.len().to_string().parse::<u64>().unwrap();
        //en este ciclo recupera los tokens que encontramos anteriormente
          for x in 0..endmeta { 
            let tokenid =  vectIDs[x as usize];
            let  token =self.get_token(tokenid.to_string());
     
            vectMEta.push(token  );
                    
        }  

        
    return vectMEta ;   
    }

    //solo obtiene los tokenid que esten en onsale y retorna un vec de META
    pub fn obtener_pagina_on_sale_V2(& self,tokens: u64,_start_index: u64, _minprice:u64,_maxprice:u64,_mindate:u64,_maxdate:u64) -> Vec<Meta>  {
        //insertar nuevo token a Hashmap
        let mut _map =self.tokenHM.clone();
        let mut vectIDs = vec![];
        let mut vectMEta = vec![];
        let ends= _map.len().to_string().parse::<u64>();
        let mut _tokfound =0;
        let mut i=0;
        let mut toksfilted:Vec<u64> = vec![];
        //comienza el filtrado segun los parametros
        //(tokens:N , _start_index:N,status:"S" ,account:"tes", price:N ,date:N)
            // (25,0, "null",0,0) ->Gallery
            if _minprice==0 && _maxprice==0 && _mindate==0 && _maxdate==0{
                let mut status =|p:u64 ,x : Vec<_> | { i+=1; x[0] =="S"  };
                toksfilted = _map.iter()
                     .filter(|st| status(st.0.clone().parse::<u64>().unwrap() ,st.1.clone()))
                     .map(|p| p.0.clone().parse::<u64>().unwrap() )
                     .collect() ;
                     toksfilted.sort();
                if toksfilted.is_empty() {
                        log!("No hay tokens aun");
                        return  vectMEta
                     }  
            } // (25,0, 2,6,0,0) ->Gallery y price between range
            else if _minprice >=0 && _maxprice>=0 && _mindate==0 && _maxdate==0 {
                let mut status =|tokeni:u64 ,vect2 : Vec<String> |
                { i+=1; vect2[0] =="S" && vect2[3].parse::<u64>().unwrap() >=_minprice  && vect2[3].parse::<u64>().unwrap() <=_maxprice };
                toksfilted = _map.iter()
                     .filter(|st| status(st.0.clone().parse::<u64>().unwrap() ,st.1.clone()))
                     .map(|p| p.0.clone().parse::<u64>().unwrap() )
                     .collect();
                     toksfilted.sort();
                  if toksfilted.is_empty() {
                     log!("No se encontraron tokens entre {} y {}",_minprice,_maxprice);
                     return  vectMEta
                  }  
            }// (25,0, 4,0,0,0) ->Gallery and less than min price
            else if _minprice >=0 && _maxprice==0 && _mindate==0 && _maxdate==0 {
                let mut status =|tokeni:u64 ,vect2 : Vec<String> |
                { i+=1; vect2[0] =="S" && vect2[3].parse::<u64>().unwrap() <=_minprice   };
                toksfilted = _map.iter()
                     .filter(|st| status(st.0.clone().parse::<u64>().unwrap() ,st.1.clone()))
                     .map(|p| p.0.clone().parse::<u64>().unwrap() )
                     .collect();
                     toksfilted.sort();
                  if toksfilted.is_empty() {
                     log!("No se encontraron tokens entre {} y {}",_minprice,_maxprice);
                     return  vectMEta
                  }  
            }
            // (25,0, 0,12,0,0) ->Gallery and more than max price
            else if _minprice ==0 && _maxprice>0 && _mindate==0 && _maxdate==0 {
                let mut status =|tokeni:u64 ,vect2 : Vec<String> |
                { i+=1; vect2[0] =="S" && vect2[3].parse::<u64>().unwrap() >=_maxprice  };
                toksfilted = _map.iter()
                     .filter(|st| status(st.0.clone().parse::<u64>().unwrap() ,st.1.clone()))
                     .map(|p| p.0.clone().parse::<u64>().unwrap() )
                     .collect();
                     toksfilted.sort();
                  if toksfilted.is_empty() {
                     log!("No se encontraron tokens entre {} y {}",_minprice,_maxprice);
                     return  vectMEta
                  }  
            }
            // (25,0, 0,0,yesterday,today) ->Gallery and  range between 2 date
            else if _minprice ==0 && _maxprice==0 && _mindate>=0 && _maxdate>=0{
                let mut status =|tokeni:u64 ,vect2 : Vec<String> |
                { i+=1; vect2[0] =="S" && vect2[4].parse::<u64>().unwrap() >=_mindate  && vect2[4].parse::<u64>().unwrap() <=_maxdate };
                toksfilted = _map.iter()
                     .filter(|st| status(st.0.clone().parse::<u64>().unwrap() ,st.1.clone()))
                     .map(|p| p.0.clone().parse::<u64>().unwrap() )
                     .collect();
                     toksfilted.sort();
                  if toksfilted.is_empty() {
                     log!("No se encontraron tokens entre la fechas {} y {}",_mindate,_maxdate);
                     return  vectMEta
                  }  
            }
            // (25,0, 0,0,yesterday,today) ->Gallery and  less than mindate
            else if _minprice ==0 && _maxprice==0 && _mindate>=0 && _maxdate==0{
                let mut status =|tokeni:u64 ,vect2 : Vec<String> |
                { i+=1; vect2[0] =="S" && vect2[4].parse::<u64>().unwrap() <=_mindate  };
                toksfilted = _map.iter()
                     .filter(|st| status(st.0.clone().parse::<u64>().unwrap() ,st.1.clone()))
                     .map(|p| p.0.clone().parse::<u64>().unwrap() )
                     .collect();
                     toksfilted.sort();
                  if toksfilted.is_empty() {
                     log!("No se encontraron tokens entre la fechas {} y {}",_mindate,_maxdate);
                     return  vectMEta
                  }  
            }
             // (25,0, 0,0,yesterday,today) ->Gallery and  more  than maxdate
             else if _minprice ==0 && _maxprice==0 && _mindate==0 && _maxdate>=0{
                let mut status =|tokeni:u64 ,vect2 : Vec<String> |
                { i+=1; vect2[0] =="S" && vect2[4].parse::<u64>().unwrap() >=_maxdate  };
                toksfilted = _map.iter()
                     .filter(|st| status(st.0.clone().parse::<u64>().unwrap() ,st.1.clone()))
                     .map(|p| p.0.clone().parse::<u64>().unwrap() )
                     .collect();
                     toksfilted.sort();
                  if toksfilted.is_empty() {
                     log!("No se encontraron tokens entre la fechas {} y {}",_mindate,_maxdate);
                     return  vectMEta
                  }  
            }
            // (25,0,"S","null",2,2132433543) ->Gallery , price y date
        // (25,0,"U","d.testnet",0,0) -> Gallery's artis
            // (25,0,"U","d.testnet",2,0) -> Gallery's artis y price
            // (25,0,"U","d.testnet",2,2234235443) -> Gallery's artis, price y date
        // (25,0,"A","null",0,0) ->Gallery auction
           // (25,0,"A","null",2,0) ->Gallery y price
            // (25,0,"A","null",2,2132433543) ->Gallery , price y date
      //  let ends= _map.len().to_string().parse::<u64>().unwrap();
        //este ciclo recupera los primeros
        for x in _start_index..ends.unwrap()  {
              _tokfound+=1;
                if _tokfound > tokens  {break;}      
            let tok = toksfilted[x as usize];
            //log!("{}",tok.clone());
                     vectIDs.push(tok );
        }  
        let endmeta = vectIDs.len().to_string().parse::<u64>().unwrap();
        //en este ciclo recupera los tokens que encontramos anteriormente
          for x in 0..endmeta {
            let tokenid =  vectIDs[x as usize];
            let  token =self.get_token(tokenid.to_string());
            vectMEta.push(token  );
        }  
    return vectMEta ;  
    }


    //solo obtiene los tokenid que esten en onsale y retorna un vec de META
    pub fn obtener_pagina_on_auction(&   self,tokens: u64,_start_index: u64 ) -> Vec<Meta> {
        //insertar nuevo token a Hashmap
        let mut _map =self.tokenHM.clone();
        let mut vectIDs = vec![];
        let mut vectMEta = vec![];
        let ends= _map.len().to_string().parse::<u64>();
        
        //este ciclo recupera los primeros
        for x in _start_index..ends.unwrap()  {
            
            if vectIDs.len() >= tokens as usize { log!("max {} ",&x); break ;}
        
            let tok = _map.get(&x.to_string() ).unwrap();
            if tok[0] == "onauction".to_string()  {
                
                    vectIDs.push(x.to_string().parse::<u64>().unwrap() );
                
            }       
            
        }

        let endmeta = vectIDs.len().to_string().parse::<u64>().unwrap();
        //en este ciclo recupera los tokens que encontramos anteriormente
          for x in 0..endmeta { 
            let tokenid =  vectIDs[x as usize];
            let mut token =self.get_token(tokenid.to_string());
            vectMEta.push(token  );
                    
        }  

        
    return vectMEta ;   
    }
    pub fn obtener_pagina_by_owner(&self,account: ValidAccountId) -> Vec<Meta>  {
         //insertar nuevo token a Hashmap
         let mut _map =self.tokenHM.clone();
         let mut vectIDs = vec![];
         let mut vectMEta = vec![];
         let ends= _map.len().to_string().parse::<u64>();
         
         //este ciclo recupera los primeros
         for x in 0..ends.unwrap()  {
            let tok = _map.get(&x.to_string() ).unwrap();
             if tok[1] == account.to_string()  {
                  vectIDs.push(x.to_string().parse::<u64>().unwrap() );
             }                  
         }
 
         let endmeta = vectIDs.len().to_string().parse::<u64>().unwrap();
         //en este ciclo recupera los tokens que encontramos anteriormente
           for x in 0..endmeta { 
             let tokenid =  vectIDs[x as usize];
             let mut token =self.get_token(tokenid.to_string());
             vectMEta.push(token);
                     
         }  
 
         
     return vectMEta ; 
         
    }
  

    pub fn tokens_of(&self, account_id: ValidAccountId,from_index: U128,limit: u64,) -> Vec<Token> {
        return self
            .tokens
            .nft_tokens_for_owner(account_id, Some(from_index), Some(limit));
    }
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
        Self {
           
            tokens:old_state.tokens,
            metadata: old_state.metadata,
            n_total_tokens:old_state.n_total_tokens,
            n_token_on_sale: old_state.n_token_on_sale,
            n_token_on_auction:old_state.n_token_on_auction,
            tokenHM:old_state.tokenHM, 
        }
    }
   
 
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


