import React, { Fragment, useEffect, useState } from "react";
import PropTypes from "prop-types";
import { Menu, Transition } from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/solid";
import { config, signOut } from "../utils/near_interaction";
import * as nearAPI from "near-api-js";
import { blockchains } from "../utils/constraint";
import nativoLogo from '../assets/img/nativologocrop.png'
import lupa from '../assets/landingSlider/img/lupa1.png'
function LightHeaderB(props) {
  const [state, setState] = useState({
    dropdown:
      blockchains[parseInt(localStorage.getItem("blockchain"))] || "Blockchain",
  });
  const [buscar, setbuscar] = useState("");

  function classNames(...classes) {
    return classes.filter(Boolean).join(" ");
  }

  useEffect(() => {
    if(state.dropdown == 'Blockchain'){
      changeBlockchain(2);
    }
    /*  (async () => {
      const { keyStores, connect, WalletConnection } = nearAPI;
      const keyStore = new keyStores.BrowserLocalStorageKeyStore();

      console.log({ ...config.testnet, keyStore });
      const near = await connect({ ...config.testnet, keyStore });

      const wallet = new WalletConnection(near);
      console.log(wallet.isSignedIn());

      if (!wallet.isSignedIn()) {
        wallet.requestSignIn(
          "dev-1626280160013-8252228", // contract requesting access
          "Latin-Art" // optional
        );
      }

      console.log(wallet.getAccountId());
      console.log(wallet.account());

      const account = await near.account(wallet.getAccountId());
      console.log(await account.getAccountBalance());
      console.log(account);
      console.log(await account.getAccountDetails());
      console.log(await account.state());
    })();*/
  }, []);

  /**
   * esta funcion nos permite cambiar de blockchain
   * @param {int} index representa la posicion dentro del arreglo blockchains
   */

  async function changeBlockchain(index) {
    setState({ dropdown: blockchains[index] });
    localStorage.setItem("blockchain", index);
    window.localStorage.setItem("page",0)
    await signOut();
    window.location.reload();
  }

  async function goNativoV1() {
    window.location.href = 'https://v1.nativonft.app/';
  }

  return (
    <header className="text-gray-600 body-font shadow-sm">
      <div className=" flex flex-wrap p-5 flex-col md:flex-row items-center movil-header">
        <a
          href="/"
          className="movil-logo"
        >
         <img  src={nativoLogo} className="d-inline-block align-top " alt="logo"   width="200px"/>

         
        </a>
        <img src=""/>
        <nav className="movil-nav md:mr-auto md:ml-4 md:py-1 md:pl-4 md:border-l md:border-gray-400	flex flex-wrap items-center text-base justify-center">
          <a href="/galeria" className="mr-5 hover:text-gray-900">
            Galeria
          </a>
          <a href="https://v1.nativonft.app/galeria" className="mr-5 hover:text-gray-900">
            Galeria V1
          </a>
          <a href="/minar" className="mr-5 hover:text-gray-900">
            Minar
          </a>
          <a href="/auctions" className="mr-5 hover:text-gray-900">
            Subastas
          </a>
          <a href="/mis_nfts" className="mr-5 hover:text-gray-900">
            Mis Nfts
          </a>
        </nav>
        
      
        <form className={"flex flex-wrap mr-7"} 
          onSubmit={e=>{
            e.preventDefault();
            window.location.href ="/perfil/"+buscar;
          }}
        >
                  <input type="text"  value={buscar}  maxLength={64} className="p-2 lg:w-12/12 px-3 buscar" placeholder="Usuario"
                    onChange={e=>{
                      
                      const not = "abcdfghijklmnopqrstuvwxyzñ1234567890_-";
                      const tex = e.target.value.toString().toLowerCase();
                      if(not.includes(tex[tex.length -1]) || tex == ""){
                        setbuscar(tex);  
                      }
                    }}  
                  />
                  <button type="submit" className="p-2 lg:w-1/12 px-3 ml-2 bg-s">
                    <img src={lupa} />
                  </button>
                </form> 

        <Menu as="div" className="relative inline-block text-left">
        
          {({ open }) => (
            <>
            
              <div className="flex flex-nowrap ">
               
                <Menu.Button className=" inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-100 focus:ring-yellow-500">
                  {state.dropdown}
                  <ChevronDownIcon
                    className="-mr-1 ml-2 h-5 w-5"
                    aria-hidden="true"
                  />
                </Menu.Button>
              </div>

              
              <Transition
                show={open}
                as={Fragment}
                enter="transition ease-out duration-100"
                enterFrom="transform opacity-0 scale-95"
                enterTo="transform opacity-100 scale-100"
                leave="transition ease-in duration-75"
                leaveFrom="transform opacity-100 scale-100"
                leaveTo="transform opacity-0 scale-95"
              >

                
                <Menu.Items
                  static
                  className="origin-top-right absolute right-0 mt-2 w-56 rounded-md shadow-lg bg-white outline-none"
                >
                  
                  <div className="py-1">

                  

                    <Menu.Item
                      onClick={() => {
                        changeBlockchain(0);
                      }}
                    >
                      {({ active }) => (
                        <a
                          href="#"
                          className={classNames(
                            active
                              ? "bg-gray-100 text-gray-900"
                              : "text-gray-700",
                            "block px-2 py-2 text-sm text-center"
                          )}
                        >
                          {blockchains[0]}
                        </a>
                      )}
                    </Menu.Item>
                    <Menu.Item
                      onClick={() => {
                        goNativoV1();
                      }}
                    >
                      {({ active }) => (
                        <a
                          href="#"
                          className={classNames(
                            active
                              ? "bg-gray-100 text-gray-900"
                              : "text-gray-700",
                            "block px-2 py-2 text-sm text-center"
                          )}
                        >
                          {blockchains[1]}
                        </a>
                      )}
                    </Menu.Item>
                    <Menu.Item
                      onClick={() => {
                        changeBlockchain(2);
                      }}
                    >
                      {({ active }) => (
                        <a
                          href="#"
                          className={classNames(
                            active
                              ? "bg-gray-100 text-gray-900"
                              : "text-gray-700",
                            "block px-2 py-2 text-sm text-center"
                          )}
                        >
                          {blockchains[2]}
                        </a>
                      )}
                    </Menu.Item>
                  </div>
                </Menu.Items>
              </Transition>
            </>
          )}
        </Menu>
      </div>
    </header>
  );
}

LightHeaderB.defaultProps = {
  theme: "indigo",
};

LightHeaderB.propTypes = {
  theme: PropTypes.string.isRequired,
};

export default LightHeaderB;
