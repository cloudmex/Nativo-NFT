import React from "react";
import {
  fromWEItoEth,
  getContract,
  getSelectedAccount,
  syncNets,
} from "../utils/blockchain_interaction";
import { currencys } from "../utils/constraint";
import { getNearContract, fromYoctoToNear } from "../utils/near_interaction";


function LightEcommerceA() {
  const [Landing, setLanding] = React.useState({
    theme: "yellow",
    currency: currencys[parseInt(localStorage.getItem("blockchain"))],
    tokens: [],
    page: parseInt( window.localStorage.getItem("page")),
    blockchain: localStorage.getItem("blockchain"),
    tokensPerPage: 10,
    tokensPerPageNear: 12,
  });

  async function getPage(pag) {
    let toks;
    if (Landing.blockchain == "0") {
      toks = await getContract()
        .methods.obtenerPaginav2(Landing.tokensPerPage, 2)
        .call();

      //filtrar tokens
      let copytoks = toks.filter((tok) => tok.onSale);

      console.log(toks);
      //convertir los precios de wei a eth
      copytoks = copytoks.map((tok) => {
        return { ...tok, price: fromWEItoEth(tok.price) };
      });

      setLanding({
        ...Landing,
        tokens: copytoks,
        page: pag,
      });
    } else {
      //instanciar contracto
      let contract = await getNearContract();
      let numberOfToks = pag * Landing.tokensPerPage;
      //obtener cuantos tokens estan a la venta
      let onSaleToks = await contract.get_on_sale_toks();
      //obtener tokens a la venta
      toks = await contract.obtener_pagina_v2({
        from_index: pag,
        limit: Landing.tokensPerPage,
      });

      //convertir los datos al formato esperado por la vista
      toks = toks.map((tok) => {
        return {
          tokenID: tok.token_id,
          price: fromYoctoToNear(tok.metadata.price),
          data: JSON.stringify({
            title: tok.metadata.title,
            image: tok.metadata.media,
          }),
        };
      });
      console.log(toks);
      setLanding({
        ...Landing,
        tokens: toks,
        page: pag,
      });
    }
  }

  React.useEffect(() => {
    (async () => {
      let toks, onSaleToks;
      let arr=[];
      
      if (Landing.blockchain == "0") {
        //primero nos aseguramos de que la red de nuestro combo sea igual a la que esta en metamask
          await syncNets();
          //obtener cuantos tokens tiene el contrato
          let totalSupply = await getContract().methods.totalSupply().call();
          //obtener el numero de tokens a la venta
          onSaleToks = await getContract().methods.nTokenOnSale.call().call();

            //indices del arreglo para la paginacion :::0*10=0 1*10=10  1*10=10 2*10=20
          for(let i =Landing.page*10; i<(parseInt(Landing.page)+1)*Landing.tokensPerPage ; i++) {
            console.log("ini",Landing.page*10,"actual",i,"fin",(parseInt(Landing.page)+1)*Landing.tokensPerPage)
            //obtiene la informacion de x token
            let infoe  = await getContract().methods.getItemInfo(i).call();
            //Valida si está a la venta
             if(infoe[0].onSale){
                  //agrega el token al arreglo para mostrar
                  arr.push(infoe[0]);
                  }
                 
           //Concadena el token encontrado con los tokens que ya se mostraron
             setLanding({
              ...Landing,
              tokens: arr,
              nPages: Math.ceil(onSaleToks / Landing.tokensPerPage),
            });  

          }
           
     
      } else {
        //instanciar contracto
        let contract = await getNearContract();
        console.log("Page",Landing.page)
        //obtener tokens a la venta
        toks = await contract.obtener_pagina_v2({
          from_index: Landing.page,
          limit: Landing.tokensPerPageNear,
        });
        //obtener cuantos tokens estan a la venta
        onSaleToks = await contract.get_on_sale_toks();

        //convertir los datos al formato esperado por la vista
        toks = toks.map((tok) => {
          return {
            tokenID: tok.token_id,
            price: fromYoctoToNear(tok.metadata.price),
            data: JSON.stringify({
              title: tok.metadata.title,
              image: tok.metadata.media,
            }),
          };
        });

        console.log("toks",toks);
        console.log("onsale",onSaleToks);
        console.log(Math.ceil(onSaleToks /Landing.tokensPerPageNear))
        setLanding({
          ...Landing,
          tokens: toks,
          nPages: Math.ceil(onSaleToks /Landing.tokensPerPageNear),
        });
      }
    })();
  }, []);
  return (
    <section className="text-gray-600 body-font">
      <div className="container px-5 py-24 mx-auto">
        {/* Arroja un mensaje si no hay tokens disponibles en venta*/}
        {!Landing.tokens.length > 0 ? (
          <p className="lg:w-2/3 mx-auto leading-relaxed text-base">
            Actualmente no hay tokens NFT disponibles.
          </p>
        ) : null}
        <div className="flex flex-wrap -m-4">
          {Landing.tokens &&
            Landing.tokens.map((token, key) => {
              //a nuestro datos le aplicamos al funcion stringify por lo cual necesitamos pasarlo
              const tokenData = JSON.parse(token.data);
              return (
                <div className="lg:w-1/3 md:w-1/2 px-3 w my-" key={key}>
                 {tokenData.image ?
                  <a href={"/detail/" + token.tokenID}>
                    <div className="token">
                    <div className="block relative h-48 overflow-hidden">
                      <img
                        alt="ecommerce"
                        className="imgaa object-cover object-center w-full h-full block"
                        src={`https://ipfs.io/ipfs/${tokenData.image}`}
                      />
                    </div>
                    <div className="mt-4">
                      <h2 className="ml-1 text-gray-900 title-font text-lg font-medium">
                        {tokenData.title}
                      </h2>
                      <p className="mt-1 mb-4 ml-2">
                        {Landing.blockchain==0 &&
                            fromWEItoEth(token.price) + " " + Landing.currency}

                        {Landing.blockchain!=0 &&
                              token.price + " " + Landing.currency}
                      </p>
                    </div>
                    </div>
                  </a>
                  :
                  <img 
                     src={"https://media.giphy.com/media/tA4R6biK5nlBVXeR7w/giphy.gif"} 
                     className="object-cover object-center w-full h-full block" />

                  }
                </div>
              );
            })}
        </div>
        <div className="bg-white px-4 py-3 flex items-center justify-center border-t border-gray-200 sm:px-6 mt-16">
          <nav
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
                    await getPage(index);
                    window.localStorage.setItem("page",index);
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
    </section>
  );
}

export default LightEcommerceA;
