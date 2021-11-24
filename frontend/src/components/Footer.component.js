import React from "react";
import PropTypes from "prop-types";
import nearicon from "../icons/near.svg";
import Roadmap from '../assets/img/Roadmap.pdf'
import nativoLogo from '../assets/img/nativologocrop.png'

function LightFooterB(props) {
  return (
    <footer className="text-gray-600 body-font">
      <div className="container px-5 py-24 mx-auto flex md:items-center lg:items-start md:flex-row md:flex-nowrap flex-wrap flex-col">
        <div className="w-64 flex-shrink-0 md:mx-0 mx-auto text-center md:text-left md:mt-0 mt-10">
          <a
            href
            className="flex title-font font-medium items-center md:justify-start justify-center text-gray-900"
          >
            <img  src={nativoLogo} class="d-inline-block align-top" alt="logo"   width="200px"/>
             
          </a>
          <p className="mt-2 text-sm text-gray-500">
            Agradecemos tu visita y preferencia
          </p>
          <div className="flex  items-center justify-between mt-8 ">
            <p>hecho con</p>
            <a href="https://near.org" target="_blank" className="mr-7">
              <img src={nearicon} />
            </a>
          </div>
        </div>
        <div className="flex-grow flex flex-wrap md:pr-20 -mb-10 md:text-left text-center order-first">
          <div className="lg:w-1/4 md:w-1/2 w-full px-4">
            <h2 className="title-font font-medium text-gray-900 tracking-widest text-sm mb-3">
              RECURSOS
            </h2>
            <nav className="list-none mb-10">
            <li>
                <a href="https://drive.google.com/file/d/1IHNp3aHcUDjn8Iws8cObC3qWWVEVcj-7/view" target="_blank" className="text-gray-600 hover:text-gray-800">
                  Mapa de ruta
                </a>
            </li>
              
            </nav>
          </div>

          <div className="lg:w-1/4 md:w-1/2 w-full px-4">
            <h2 className="title-font font-medium text-gray-900 tracking-widest text-sm mb-3">
              COMPAÑIA
            </h2>
            <nav className="list-none mb-10">
              <li>
                <a
                  href="/contacto"
                  className="text-gray-600 hover:text-gray-800"
                >
                  Contacto
                </a>
              </li>
              <li>
                <a
                  href="acercade"
                  className="text-gray-600 hover:text-gray-800"
                >
                  Acerca de
                </a>
              </li>

              <li>
                <a
                  href="/terminos"
                  className="text-gray-600 hover:text-gray-800"
                >
                  Términos (S)
                </a>
              </li>
              <li>
                <a
                  href="/politicas"
                  className="text-gray-600 hover:text-gray-800"
                >
                  Política (P)
                </a>
              </li>
            </nav>
          </div>
        </div>
      </div>
      <div className="bg-gray-100">
        <div className="container mx-auto py-4 px-5 flex flex-wrap flex-col sm:flex-row">
          <p className="text-gray-500 text-sm text-center sm:text-left">
            © 2021 NATIVO NFT-MARKETPLACE
          </p>
          <span className="inline-flex sm:ml-auto sm:mt-0 mt-2 justify-center sm:justify-start">
            <a href="https://github.com/cloudmex/Nativo-NFT" target="_blank" className="text-gray-500">
            <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 24 24"><path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-4.466 19.59c-.405.078-.534-.171-.534-.384v-2.195c0-.747-.262-1.233-.55-1.481 1.782-.198 3.654-.875 3.654-3.947 0-.874-.312-1.588-.823-2.147.082-.202.356-1.016-.079-2.117 0 0-.671-.215-2.198.82-.64-.18-1.324-.267-2.004-.271-.68.003-1.364.091-2.003.269-1.528-1.035-2.2-.82-2.2-.82-.434 1.102-.16 1.915-.077 2.118-.512.56-.824 1.273-.824 2.147 0 3.064 1.867 3.751 3.645 3.954-.229.2-.436.552-.508 1.07-.457.204-1.614.557-2.328-.666 0 0-.423-.768-1.227-.825 0 0-.78-.01-.055.487 0 0 .525.246.889 1.17 0 0 .463 1.428 2.688.944v1.489c0 .211-.129.459-.528.385-3.18-1.057-5.472-4.056-5.472-7.59 0-4.419 3.582-8 8-8s8 3.581 8 8c0 3.533-2.289 6.531-5.466 7.59z"/></svg>
            </a>
            <a href="https://twitter.com/nativonft" target="_blank"  className="ml-3 text-gray-500">
            <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 24 24"><path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-.139 9.237c.209 4.617-3.234 9.765-9.33 9.765-1.854 0-3.579-.543-5.032-1.475 1.742.205 3.48-.278 4.86-1.359-1.437-.027-2.649-.976-3.066-2.28.515.098 1.021.069 1.482-.056-1.579-.317-2.668-1.739-2.633-3.26.442.246.949.394 1.486.411-1.461-.977-1.875-2.907-1.016-4.383 1.619 1.986 4.038 3.293 6.766 3.43-.479-2.053 1.08-4.03 3.199-4.03.943 0 1.797.398 2.395 1.037.748-.147 1.451-.42 2.086-.796-.246.767-.766 1.41-1.443 1.816.664-.08 1.297-.256 1.885-.517-.439.656-.996 1.234-1.639 1.697z"/></svg>
            </a>
            <a href="https://discord.com/invite/7usKw4Dk" target="_blank" className="ml-3 text-gray-500">
            <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 24 24"><path d="M19 24h-14c-2.761 0-5-2.239-5-5v-14c0-2.761 2.239-5 5-5h14c2.762 0 5 2.239 5 5v14c0 2.761-2.238 5-5 5zm-3.288-4.888l-.424-1.48 1.024.952.968.896 1.72 1.52v-14.352c0-.912-.736-1.648-1.64-1.648h-10.72c-.904 0-1.64.736-1.64 1.648v10.816c0 .912.736 1.648 1.64 1.648h9.072zm-1.44-3.664c1.768-.056 2.448-1.216 2.448-1.216 0-2.576-1.152-4.664-1.152-4.664-1.152-.864-2.248-.84-2.248-.84l-.112.128c1.36.416 1.992 1.016 1.992 1.016-.832-.456-1.648-.68-2.408-.768-.576-.064-1.128-.048-1.616.016l-.136.016c-.28.024-.96.128-1.816.504l-.472.232s.664-.632 2.104-1.048l-.08-.096s-1.096-.024-2.248.84c0 0-1.152 2.088-1.152 4.664 0 0 .672 1.16 2.44 1.216l.536-.664c-1.016-.304-1.4-.944-1.4-.944l.224.136.032.024.032.018.009.004.031.018c.2.112.4.2.584.272.328.128.72.256 1.176.344.6.112 1.304.152 2.072.008.376-.064.76-.176 1.16-.344.28-.104.592-.256.92-.472 0 0-.4.656-1.448.952l.528.648zm-3.72-3.736c-.456 0-.816.4-.816.888s.368.888.816.888c.456 0 .816-.4.816-.888.008-.488-.36-.888-.816-.888zm2.92 0c-.456 0-.816.4-.816.888s.368.888.816.888c.456 0 .816-.4.816-.888s-.36-.888-.816-.888z"/></svg>            </a>
 
          </span>
        </div>
      </div>
    </footer>
  );
}

LightFooterB.defaultProps = {
  theme: "indigo",
};

LightFooterB.propTypes = {
  theme: PropTypes.string.isRequired,
};

export default LightFooterB;
