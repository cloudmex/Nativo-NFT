import React from "react";
import {
  fromWEItoEth,
  getContract,
  getSelectedAccount,
  syncNets,
} from "../utils/blockchain_interaction";
import { currencys } from "../utils/constraint";
import { getNearContract, fromYoctoToNear, getNearAccount } from "../utils/near_interaction";
import { useParams, useHistory } from "react-router-dom";

import filtroimg from '../assets/landingSlider/img/filtro.png'
import countrys from '../utils/countrysList'
import loading from '../assets/landingSlider/img/loader.gif'
import Pagination from '@mui/material/Pagination';
import { Account } from "near-api-js";
import { ApolloClient, InMemoryCache, gql } from '@apollo/client'

function LightEcommerceA() {
  const [Landing, setLanding] = React.useState({
    theme: "yellow",
    currency: currencys[parseInt(localStorage.getItem("blockchain"))],
    tokens: [],
    page: parseInt(window.localStorage.getItem("page")),
    pag: window.localStorage.getItem("pagSale"),
    blockchain: localStorage.getItem("blockchain"),
    tokensPerPage: 10,
    tokensPerPageNear: 9,
  });
  const [esconder, setesconder] = React.useState(true);
  const [counter, setcounter] = React.useState();
  const [load, setload] = React.useState(false);
  const [pagsale, setpagsale] = React.useState(0);
  const [pagCount, setpagCount] = React.useState("");
  const [chunksale, setchunksale] = React.useState(0);
  const [page, setpage] = React.useState(1);
  const [ini, setini] = React.useState(true);
  const [trigger, settrigger] = React.useState(true);
  const [filtro, setfiltro] = React.useState({
    culture: "null",
    country: "null",
    type: "null",
    date: "null",
    price: "null",
  });

  const APIURL = 'https://api.thegraph.com/subgraphs/name/luisdaniel2166/nativotest'

  const handleChangePage = (e, value) => {
    // console.log(value)
    setpage(value)
    window.scroll(0, 0)
    settrigger(!trigger)
  }

  const modificarFiltro = (v) => {
    setfiltro(c => ({ ...c, ...v }))
  }

  const { data } = useParams();

  var tokData
  var colData
  const { tokenid: owner } = useParams();
  React.useEffect(() => {
    // console.log("esto ---> ",owner);

    setload(c => true);
    (async () => {
      let toks, onSaleToks;
      let arr = [];

      if (Landing.blockchain == "0") {
        //primero nos aseguramos de que la red de nuestro combo sea igual a la que esta en metamask
        await syncNets();
        //obtener cuantos tokens tiene el contrato
        let totalSupply = await getContract().methods.totalSupply().call();
        //obtener el numero de tokens a la venta
        onSaleToks = await getContract().methods.nTokenOnSale.call().call();

        //indices del arreglo para la paginacion :::0*10=0 1*10=10  1*10=10 2*10=20
        for (let i = Landing.page * 10; i < (parseInt(Landing.page) + 1) * Landing.tokensPerPage; i++) {
          //console.log("ini",Landing.page*10,"actual",i,"fin",(parseInt(Landing.page)+1)*Landing.tokensPerPage)
          //obtiene la informacion de x token
          let infoe = await getContract().methods.getItemInfo(i).call();
          //Valida si está a la venta
          if (infoe[0].onSale) {
            //agrega el token al arreglo para mostrar
            arr.push(infoe[0]);
          }

          //Concadena el token encontrado con los tokens que ya se mostraron
          setLanding({
            ...Landing,
            tokens: arr,
            nPages: Math.ceil(arr.length / Landing.tokensPerPage),
          });

        }


      } else {
        window.contr = await getNearContract();

        //instanciar contracto
        let contract = await getNearContract();
        let account = await getNearAccount();
        //console.log("Page",Landing.page)
        //obtener tokens a la venta
        // //console.log("Paasdsadfsdfdge",Landing.page*30,"edfew" ,Landing.tokensPerPageNear*(Landing.page+1))
        // let pag = await contract.get_ids_onsale({
        //    tokens: Landing.tokensPerPageNear})
        //  window.localStorage.setItem('pagSale',pag)

        // let payload = {
        //   account : (owner.toString().toLowerCase()+".testnet").toString(),
        //   //from_index: nfts.page, 
        //   //limit: nfts.tokensPerPageNear,
        // };
        // console.log("payload ",payload);
        // toks = await contract.obtener_pagina_by_creator(payload);
        let info = data.split(":");
        const queryData = `
          query($contract: String, $collectionID: String){
            collections(where: {collectionID: $collectionID}) {
              id
              owner
              title
              tokenCount
              description
              contract
              mediaIcon
              mediaBanner
              saleCount
              saleVolume
              collectionID
            }
            tokens(where: {collectionID: $collectionID}) {
              id
              collection
              collectionID
              contract
              tokenId
              owner_id
              title
              description
              media
              creator
              price
              status
              adressbidder
              highestbidder
              lowestbidder
              expires_at
              starts_at
              extra
            }
          }
        `
        //Declaramos el cliente
        const client = new ApolloClient({
          uri: APIURL,
          cache: new InMemoryCache(),
        })

        await client
          .query({
            query: gql(queryData),
            variables: {
              collectionID: data
            },
          })
          .then((data) => {
            console.log("collections data: ", data.data.collections)
            // console.log("tokens data: ",data.data.tokens)
            tokData = data.data.tokens
            colData = data.data.collections[0]
          })
          .catch((err) => {
            console.log('Error ferching data: ', err)
          })
        // console.log(tokData)

        // var pag = await contract.get_pagination_creator_filters({
        //   account : (owner.toString().toLowerCase()).toString(),
        //   tokens: Landing.tokensPerPageNear,
        //   //_start_index: Landing.page,
        //   _start_index: pagsale,
        //   _minprice: 0,
        //   _maxprice: 0,
        //   _mindate: 0,
        //   _maxdate: 0,
        // })
        // let pagi= pag.toString()
        // setpagCount(pagi)
        // console.log(pagi)
        // console.log(pagCount)
        // window.localStorage.setItem("pagPerf",parseInt(pagi.split(",")[0].split("-")[1]))
        // window.localStorage.setItem("pagCPerf",parseInt(pagi.split(",")[0].split("-")[0]))
        // console.log(chunksale)
        // console.log(pagsale)
        // console.log(page)
        // toks = await contract.obtener_pagina_creator({
        //   account : (owner.toString().toLowerCase()).toString(),
        //   chunk: (ini ? parseInt(window.localStorage.getItem("pagCPerf")): chunksale),
        //   tokens: Landing.tokensPerPageNear,
        //   //_start_index: Landing.page,
        //   _start_index: (ini ? parseInt(window.localStorage.getItem("pagPerf")): pagsale),
        //   _minprice: 0,
        //   _maxprice: 0,
        //   _mindate: 0,
        //   _maxdate: 0,
        // });
        // console.log("toks ",toks);
        // let pagNumArr = pag
        // //obtener cuantos tokens estan a la venta
        // if(ini){
        //   window.localStorage.removeItem("pagCPerf")
        //   window.localStorage.removeItem("pagPPerf")
        //   setini(!ini)
        // }

        //convertir los datos al formato esperado por la vista
        let tok = tokData.map((tok) => {
          return {
            title: tok.title,
            tokenId: tok.tokenId,
            media: tok.media,
            price: tok.price,
            owner: tok.owner_id
          };
        });
        console.log(tok)

        //console.log("toks",toks);
        //console.log("onsale",onSaleToks);
        //console.log(Math.ceil(onSaleToks /Landing.tokensPerPageNear))
        let numpage = parseInt(tok.length / Landing.tokensPerPageNear)
        if (tok.length % Landing.tokensPerPageNear > 0) {
          numpage++
        }
        await setLanding({
          ...Landing,
          tokens: tok.slice(Landing.tokensPerPageNear * (page - 1), Landing.tokensPerPageNear * page),
          nPages: numpage,
          titleCol: colData.title,
          ownerCol: colData.owner,
          mediaCol: colData.mediaIcon,
          bannerCol: colData.mediaBanner,
          descriptionCol: colData.description,
          contract: colData.contract,
          tokenCount: colData.tokenCount,
          saleCount: colData.saleCount,
          saleVolume: fromYoctoToNear(colData.saleVolume),
          colID: colData.collectionID
        });
      }

    })();
  }, [trigger]);

  return (
    <section className="text-gray-600 body-font">
      <div className={`container px-5 pt-6 mx-auto flex flex-wrap flex-col text-center items-center `}>
        <img
          className="object-cover h-96 w-full rounded-3xl  z-0 opacity-80 brightness-[.75] blur-sm"
          src={`https://ipfs.io/ipfs/${Landing.bannerCol}`}
        />
        <img
          className="object-cover h-48 w-48 rounded-3xl border-solid border-4 border-slate-700 z-10 -mt-96"
          src={`https://ipfs.io/ipfs/${Landing.mediaCol}`}
        />
        <div className="z-10 -mt-120 w-full text-white">
          <div className="bg-white lg:mx-20 mx-5 text-black mt-4 pt-2 rounded-t-2xl bg-opacity-80">
            <h1 className="lg:text-5xl text-3xl font-bold pb-4 opacity-100 stroke-gray-700">{Landing.titleCol}</h1>
            <p className="lg:text-xl text-base px-2 pb-3 stroke-gray-700">{Landing.descriptionCol == "" ? "Esta coleccion no tiene una descripcion" : Landing.descriptionCol}</p>
            <div className="grid grid-cols-2 divide-x pb-3 mx-auto stroke-gray-700">
              <div>
                <p className="lg:text-xl text-base pb-1 lg:text-right text-center lg:mr-5 ml-1"><b>Creador:</b><br/>{Landing.ownerCol}</p>
              </div>
              <div>
                <p className="lg:text-xl text-base pb-1 lg:text-left text-center lg:ml-5 mr-1"><b>Contrato:</b><br/>{Landing.contract}</p>
              </div>
            </div>
          </div>
          <div className="grid grid-cols-3 divide-x gap-1 bg-yellow-400 rounded-b-2xl text-white lg:mx-20 mx-5 mx-auto text-center">
            <div className="pl-5">
              <p className="lg:text-lg text-base pb-1"><b>No. de tokens:</b></p>
              <p className="lg:text-base text-sm pb-1">{Landing.tokenCount}</p>
            </div>
            <div>
              <p className="lg:text-lg text-base pb-1"><b>No. de ventas:</b></p>
              <p className="lg:text-base text-sm pb-1">{Landing.saleCount}</p>
            </div>
            <div className="pr-5">
              <p className="lg:text-lg text-base pb-1"><b>Vol. de venta:</b></p>
              <p className="lg:text-base text-sm pb-1">{Landing.saleVolume} {Landing.currency}</p>
            </div>
          </div>
        </div>

      </div>
      <div className="bg-white px-4 py-3 flex items-center justify-center border-b border-gray-200 sm:px-6 mt-1">
        <Pagination count={Landing.nPages} page={page} onChange={handleChangePage} color="warning" theme="light" />
      </div>
      {/* <div className={"container px-5 mx-auto flex flex-wrap items-center "+(
        esconder? "" : "py-2"
      )}>
        <div className="fs-1 flex items-center" onClick={e=>{
            setesconder(v=> !v);
          }}>
          <img src={filtroimg} className="logg mr-1"/>
          <b>Filtro</b>
        </div>
      </div>
      <div className={"container py-5 px-5  mx-auto flex flex-wrap items-center "+(
        esconder ? "" : "esconder"
      )} >
        <b>Tipo: </b>
      <select className="ml-2 p-2 lg:w-2/12 bg-s1 ">
          <option >
            Todos los tokens
          </option>
          <option >
            Tokens en venta
          </option>
          <option >
            Tokens en subasta
          </option>
        </select>
        <b className="ml-2" >Fecha:</b>
        <select className="p-2 lg:w-2/12 ml-2 bg-s1">
          <option >
            Todos los tokens
          </option>
          <option >
            Nuevos tokens
          </option>
          <option >
            Tokens Antiguos
          </option>
        </select>
        <b className="ml-2" >Precio:</b>
        <select className="p-2 lg:w-2/12 ml-2 bg-s1">
          <option >
            Todos los tokens
          </option>
          <option >
            Asendente
          </option>
          <option >
            Desendente
          </option>
        </select>
        <b className="ml-2" >País:</b>
        <select className="p-2 lg:w-2/12 ml-2 bg-s1" onChange={e=>{
            modificarFiltro({country: (e.target.value == "Todos los tokens" ? "null" : e.target.value)});
          }}>
          <option >
            Todos los tokens
          </option>
          {
            countrys.map(c=>(
              <option >
                {c}
              </option>
            ))
          }
        </select>
      </div> */}
      <div className="container px-5 py-3 mx-auto ">

        {/* Arroja un mensaje si no hay tokens disponibles en venta*/}

        <div className={"flex flex-wrap" + (load ? " justify-center" : "")}>

          {/* 
          {
            load ?
            <img src={loading} style={{width:"50px"}}/>
            : */}
          {
            Landing.tokens.length > 0 ?
              Landing.tokens.map((element, key) => {
                //a nuestro datos le aplicamos al funcion stringify por lo cual necesitamos pasarlo
                //const tokenData = JSON.parse(token.data);
                return (
                  <div className="lg:w-1/3 md:w-1/2 px-3 w my-" key={key}>
                    <a
                      href={"/detail/" + element.tokenId + ":" + Landing.colID}
                    >
                      <div className="token bg-[#f7f4f0]">
                        <div className="block relative h-48 rounded overflow-hidden">

                          <img
                            alt="ecommerce"
                            className="imgaa object-cover object-center w-full h-full block"
                            src={`https://ipfs.io/ipfs/${element.media}`}
                          />



                        </div>
                        <div className="mt-4">
                          <h2 className="ml-1 text-gray-900 title-font text-lg font-medium">
                            {element.title}
                          </h2>
                          <p className="mt-1 mb-4 ml-2">
                            <b>TokenID:</b> {element.tokenId + "\n"}
                            <br />
                            <b>Dueño:</b> {element.owner + "\n"}
                            <br />
                            {Landing.blockchain != 0 &&
                              fromYoctoToNear(element.price) + " " + Landing.currency}
                            <br />
                            {/* {Landing.blockchain==0 &&
                            fromWEItoEth(token.price) + " " + Landing.currency}

                        {Landing.blockchain!=0 &&
                              fromYoctoToNear(token.price) + " " + Landing.currency} */}
                          </p>
                        </div>
                      </div>
                    </a>
                  </div>
                );
              })
              :
              <div class="container mx-auto flex  my- md:flex-row flex-col  justify-center h-96 items-center text-3xl">
                <div class="flex flex-col justify-center">
                  <h1 class="text-center">Aún no hay NFTs en esta colección</h1>
                </div>
              </div>
          }
        </div>
        <div className="bg-white px-4 py-3 flex items-center justify-center border-t border-gray-200 sm:px-6 mt-16">
          <Pagination count={Landing.nPages} page={page} onChange={handleChangePage} color="warning" theme="light" />
          {/* <nav
            className="relative z-0 inline-flex rounded-md shadow-sm -space-x-px"
            aria-label="Pagination"
          > 
            {Landing?.page != 0 && (
              <a
                href="#"
                className="relative inline-flex items-center px-2 py-2 rounded-l-md  border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50"
              >
                <span className="sr-only">Previous</span>
                <svg
                  className="h-5 w-5"
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                  aria-hidden="true"
                >
                  <path
                    fillRule="evenodd"
                    d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                    clipRule="evenodd"
                  />
                </svg>
              </a>
            )}
            {[...Array(Landing?.nPages)].map((page, index) => {
              return (
                <a
                  
                  className={`bg-white ${
                    Landing.page == index
                      ? "bg-yellow-100 border-yellow-500 text-yellow-600 hover:bg-yellow-200"
                      : "border-gray-300 text-gray-500 hover:bg-gray-50"
                  }  relative inline-flex items-center px-4 py-2 text-sm font-medium`}
                  key={index}
                  onClick={async () => {
                  //  await getPage(index);
                    if(index == 0){
                      window.localStorage.setItem("page",0)
                    }
                    else{
                      window.localStorage.setItem("page",parseInt(Landing.pag.split(",")[index])+1);  
                    }
                    setcounter(Landing.tokens[Landing.tokens.length-1].tokenID +1)

                    window.location.reload();
                  }}
                >
                  {index + 1}
                </a>
              );
            })}
          </nav> */}
        </div>
      </div>
    </section>
  );
}

export default LightEcommerceA;
