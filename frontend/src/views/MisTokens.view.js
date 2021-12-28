import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import Tooltip from '@mui/material/Tooltip';
//Importamos metodos de interacción con el smartcontract
import {
  fromWEItoEth,
  getContract,
  getSelectedAccount,
  syncNets,
} from "../utils/blockchain_interaction";

import { useHistory } from "react-router";
import ModalSubasta from '../components/modalSubasta.component'
import Modal from "../components/modalRevender.component";
import { currencys } from "../utils/constraint";
import {
  getNearAccount,
  getNearContract,
  fromYoctoToNear,
  fromNearToYocto,
} from "../utils/near_interaction";
import Swal from 'sweetalert2';

function MisTokens(props) {
  //Hooks para el manejo de estados
  const [nfts, setNfts] = useState({
    nfts: [],
    page:  parseInt( window.localStorage.getItem("Mypage")),
    tokensPerPage: 9,
    tokensPerPageNear: 15,

    blockchain: localStorage.getItem("blockchain"),
    currency: currencys[parseInt(localStorage.getItem("blockchain"))],
  }); //state de los token nft
  const [modal, setModal] = useState({
    //state para la ventana modal
    show: false,
  });

  const [modalSub, setModalSub] = useState({
    //state para la ventana modal
    show: false,
  });

  const history = useHistory();

  const sendtonearwallet = async (e)=>{   
    Swal.fire({
        title: `¡ATENCIÓN!`,
        icon: 'info',
        text: 'Esta acción creará una transferencia nula que generará un error, pero los tokens serán agregados al Near Wallet una vez finalizada. (Solo se debe realizar una vez)',
        showDenyButton: true,
        confirmButtonText: 'Continuar',
        confirmButtonColor: '#f89c0c',
        denyButtonText: `Cancelar`,
        iconColor: '#f89c0c',
        backdrop: 'rgba(0, 0, 0,0.5)'
    }).then(async (result) => {
        if (result.isConfirmed) {
          //console.log(e);
          let contract = await getNearContract();
          let account = await getNearAccount();
      
           
          try {
              let res = await contract.nft_transfer_call(
                {"receiver_id":""+account,"token_id":""+e,"approval_id":null,"memo":"","msg":""},
                300000000000000,
                1,
          );
          } catch (error) {
            //console.error();
          }
        } else if (result.isDenied) {

        }
    }) 
  }

  async function getPage(pag) {
    if (nfts.blockchain == "0") {
      //esta funcion nos regresa todos los tokens por que solidity no permite arreglos
      //dinamicos en memory
      let toks = await getContract()
        .methods.tokensOfPaginav1(nfts.owner, nfts.tokensPerPage, pag)
        .call();

      //asignamos y filtramos todos los tokens que estan a  la venta

      setNfts({
        ...nfts,
        nfts: toks.filter((tok) => tok.onSale),
        page: pag,
      });
    } else {
      let contract = await getNearContract();
      let account = await getNearAccount();
      //console.log("pag",pag,"nfts.tokensPerPageNear",nfts.tokensPerPageNear)
      let payload = {
        account_id: account.toString(),
        from_index: ""+pag.toString(),
        limit: nfts.tokensPerPageNear.toString(),
      };

      let nftsArr = await contract.nft_tokens_for_owner(payload);

      //convertir los datos al formato esperado por la vista
      nftsArr = nftsArr.map((tok) => {
        return {
          tokenID: tok.token_id,
          price: fromYoctoToNear(tok.metadata.price),
          onSale: tok.metadata.on_sale,
          data: JSON.stringify({
            title: tok.metadata.title,
            image: tok.metadata.media,
          }),
        };
      });

      setNfts({
        ...nfts,
        nfts: nftsArr,
        page: pag,
      });
    }
  }

  //Hook para el manejo de efectos
  useEffect(() => {
    (async () => {
      window.localStorage.setItem("Mypage",0);
     
      if (nfts.blockchain == "0") {
        //Comparamos la red en el combo de metamask con la red de aurora
        await syncNets();
        let account = await getSelectedAccount();
        //obtenemos el listado de nfts
        let nftsArr = await getContract()
          .methods.tokensOfPaginav1(account, nfts.tokensPerPage, nfts.page)
          .call();
        let balance = await getContract().methods.balanceOf(account).call();
        //console.log(nftsArr);

        //filtrar tokens
        let copytoks = nftsArr.filter((tok) => tok.price > 0);

        //convertir los precios de wei a eth
        copytoks = copytoks.map((tok) => {
          return { ...tok, price: fromWEItoEth(tok.price) };
        });

        //Actualizamos el estado el componente con una propiedad que almacena los tokens nft
        setNfts({
          ...nfts,
          nfts: copytoks,
          nPages: Math.ceil(balance / nfts.tokensPerPage)+1,
          owner: account,
        });
      } else {
        
        let contract = await getNearContract();
        let account = await getNearAccount();
        //console.log(account);
        let payload = {
          account : account,
          //from_index: nfts.page , 
          //limit: nfts.tokensPerPageNear,
        };

        let nftsArr = await contract.obtener_pagina_by_owner(payload);
        let balance = await contract.nft_supply_for_owner({
          account_id: account,
        });
        //console.log("extras:",nftsArr  );
        //console.log("balance",balance);
 
        //convertir los datos al formato esperado por la vista
        nftsArr = nftsArr.map((tok) => {
          //console.log("X->",  tok  )
         
          return {
            tokenID: tok.token_id,
            price:  fromYoctoToNear(tok.price),
            onSale: tok.on_sale,// tok.metadata.on_sale,
            onAuction: tok.on_auction,
            data: JSON.stringify({
              title: tok.title,//"2sdfeds",// tok.metadata.title,
              image:tok.media,//"vvvvvvvvvvvvvv",//tok.metadata.media,
            }),
          };
        });
        //Actualizamos el estado el componente con una propiedad que almacena los tokens nft
        setNfts({
          ...nfts,
          nfts: nftsArr.slice(nfts.page*nfts.tokensPerPageNear,(nfts.page+1)*nfts.tokensPerPageNear),
          nPages: Math.ceil(nftsArr.length / nfts.tokensPerPageNear),
          owner: account,
        });
      }
    })();
  }, []);

  /**
   * Función que cambia a "no disponible" un token nft que esta a la venta siempre que se sea el owner
   * @param tokenId representa el token id del nft a quitar del marketplace
   * @return void
   */
  async function quitarDelMarketplace(tokenId) {
    setNfts({ ...nfts, disabled: true });
    let quitar;
    if (nfts.blockchain == "0") {
      await syncNets();

      let account = await getSelectedAccount();
      quitar = await getContract()
        .methods.quitarDelMarketPlace(tokenId)
        .send({
          from: account,
        })
        .catch((err) => {
          return err;
        });
    } else {
      let contract = await getNearContract();
      let payload = {
        token_id: tokenId,
      };
      let amount = fromNearToYocto(0);
      //console.log(amount);
      //console.log(payload);
      quitar = await contract.quitar_del_market_place(
        payload,
        300000000000000, // attached GAS (optional)
        amount
      );
    }

    //console.log(quitar);
    //recargar la pantalla si la transacción se ejecuto correctamente
    if (quitar.title || quitar.status) {
      history.go(0);
    }

    setNfts({ ...nfts, disabled: false });
  }

  return (
    <>
      <section className="text-gray-600 body-font">
        <div className="container px-5 py-24 mx-auto">
          <div className="flex flex-col text-center w-full mb-20">
            <h1 className="sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900">
              Mis piezas de arte NFT
            </h1>
            <p className="lg:w-2/3 mx-auto leading-relaxed text-base">
              En esta sección aparecen los token nfts que has creado o
              adquirido.
            </p>
            <div className="flex justify-end">
              <button
                className={`sp-7 mt-2 text-white bg-${props.theme}-500 border-0 py-2 px-5 focus:outline-none hover:bg-${props.theme}-600 rounded text-md`}
                onClick={() => {
                  sendtonearwallet(nfts.tokenID)
                }}
                
              >
                Mostrar tokens en NEAR Wallet
              </button>
            </div>
            {/* Arroj un mensaje si no hay tokens en mi pertenencia*/}
            {nfts.nfts.length > 0 ? null : (
              <p className="lg:w-2/3 mx-auto leading-relaxed text-base">
                Actualmente no tienes tokens en tu pertenencia.
              </p>
            )}
          </div>
          <div className="flex flex-wrap -m-4">
            {/* Hacemos un map del array de nft dentro del state */}
            {nfts?.nfts &&
              nfts.nfts.map((nft, key) => {
                //obtenemos la data del token nft
                const nftData = JSON.parse(nft.data);
                //console.log("Aquiiii",nft);
                return (
                  //devolvemos un card por cada token nft del usuario
                  <div className="lg:w-1/3 sm:w-1/2 p-4 sp-6" key={key}>
                    
                    <div className="flex relative">
                      <img
                        alt="gallery"
                        className="absolute inset-0 w-full h-full object-cover object-center"
                        src={`https://ipfs.io/ipfs/${nftData.image}`}
                      />
                      <div className="px-8 py-10 relative z-10 w-full border-4 border-gray-200 bg-white opacity-0 hover:opacity-100 ">
                        <h1 className="title-font text-lg font-medium text-gray-900 mb-3">
                          {nftData.title}
                        </h1>
                        <p className="leading-relaxed">{nftData.description }</p>
                        {/* Etiqueta de token en venta */}
                        <div
                          className={`flex border-l-4 border-${props.theme}-500 py-2 px-2 my-2 bg-gray-50`}
                        >
                          <span className="text-gray-500">En venta</span>
                          <span className="ml-auto text-gray-900">
                            <span
                              className={`inline-flex items-center justify-center px-2 py-1  text-xs font-bold leading-none ${
                                nft.onSale
                                  ? "text-green-100 bg-green-500"
                                  : "text-red-100 bg-red-500"
                              } rounded-full`}
                            >
                              {nft.onSale ? "Disponible" : "No disponible"}
                            </span>
                          </span>
                        </div>
                         
                        <p className="leading-relaxed">{nftData.description }</p>
                        {/* Etiqueta de token en subasta */}
                        {/* <div
                          className={`flex border-l-4 border-${props.theme}-500 py-2 px-2 my-2 bg-gray-50`}
                        >
                          <span className="text-gray-500">OnAuction</span>
                          <span className="ml-auto text-gray-900">
                            <span
                              className={`inline-flex items-center justify-center px-2 py-1  text-xs font-bold leading-none ${
                                nft.onAuction
                                  ? "text-green-100 bg-green-500"
                                  : "text-red-100 bg-red-500"
                              } rounded-full`}
                            >
                              {nft.onAuction ? "Disponible" : "No disponible"}
                            </span>
                          </span>
                        </div> */}
                        <br></br>
                        <h2
                          className={`tracking-widest text-sm title-font font-medium text-${props.theme}-500 mb-1`}
                        >{`Token id: ${nft.tokenID}  `}</h2>
                        <h2
                          className={`tracking-widest text-sm title-font font-medium text-${props.theme}-500 mb-1`}
                        >{`Adquirido en $${nft.price} ${nfts.currency}`}</h2>
                        {/* Mostramos la opción de revender o quitar del marketplace */}
                        {nft.onSale ? (<>      <button
                            className={` mt-12 w-full text-white bg-${props.theme}-500 border-0 py-2 px-6 focus:outline-none hover:bg-${props.theme}-600 rounded text-lg`}
                            disabled={nfts.disabled}
                            onClick={async () => {
                              await quitarDelMarketplace(nft.tokenID);
                            }}
                          >
                            Quitar a la venta
                          </button>
                          </>
                    
                        ) : (
                          <>
                          {!nft.onAuction && <>  <button
                            className={` mt-12 w-full text-white bg-${props.theme}-500 border-0 py-2 px-6 focus:outline-none hover:bg-${props.theme}-600 rounded text-lg`}
                            onClick={() => {
                              setModal({
                                ...modal,
                                show: true,
                                tokenId: nft.tokenID,
                                title: "Revender nft",
                                currency: nfts.currency,
                                blockchain: nfts.blockchain,
                                message:
                                  "Ingresa el costo al cual quieres poner a la venta este NFT.",
                                buttonName: "Cancelar",
                                change: setModal,
                              });
                             
                            }}
                          >
                            Poner en venta
                          </button>
                          {/* <button
                            className={` mt-2 w-full text-white bg-${props.theme}-500 border-0 py-2 px-6 focus:outline-none hover:bg-${props.theme}-600 rounded text-lg`}
                            onClick={() => {
                              setModalSub({
                                ...modalSub,
                                show: true,
                                tokenId: nft.tokenID,
                                title: "Subastar NFT",
                                currency: nfts.currency,
                                blockchain: nfts.blockchain,
                                message:
                                  "Ingresa el monto base al que quieres subastar este NFT junto a su fecha y hora de finalizacion.",
                                buttonName: "Cancelar",
                                change: setModalSub,
                              });
                             
                            }}
                          >
                            Poner en subasta
                          </button> */}
                         
                      
                          </>}
                        
                        </>

                        
                        )}
                      </div>
                    </div>
                  </div>
                );
              })}
          </div>

          <div className="bg-white px-4 py-3 flex items-center justify-center border-t border-gray-200 sm:px-6 mt-16">
            <nav
              className="relative z-0 inline-flex rounded-md shadow-sm -space-x-px"
              aria-label="Pagination"
            >
              {[...Array(nfts?.nPages)].map((page, index) => {
                return (
                  <a
                    href="#"
                    className={`bg-white ${
                      nfts.page == index
                        ? "bg-yellow-100 border-yellow-500 text-yellow-600 hover:bg-yellow-200"
                        : "border-gray-300 text-gray-500 hover:bg-gray-50"
                    }  relative inline-flex items-center px-4 py-2 text-sm font-medium`}
                    key={index}
                    onClick={async () => {
                      window.localStorage.setItem("Mypage",index);
                      window.location.reload();
                    }}
                  >
                    {index + 1}
                  </a>
                );
              })}
            </nav>
          </div>
        </div>

        {/* Mandamos a llamar al modal con el state como props*/}
        <ModalSubasta {...modalSub} />
        <Modal {...modal} />
      </section>
    </>
  );
}

MisTokens.propTypes = {
  theme: PropTypes.string,
};

MisTokens.defaultProps = {
  theme: "yellow",
};

export default MisTokens;
