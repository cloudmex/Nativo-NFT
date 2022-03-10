import React from "react";
import PropTypes from "prop-types";
import nativoLogo from "../assets/img/nativologocrop.png"
import arteNativo from '../assets/img/arte nativo.png'
function LightStatisicC(props) {
  return (
    <div>
      <section className="text-gray-600 body-font bg-white">
        <div className="container px-5 lg:py-10 py:12 mx-auto mb-10 lg:mb-0 ">
          <div className="grid lg:grid-cols-2 grid-cols-1 w-full  place-items-center text-center">
            <div className="w-full">
              <img className="p-[50px]  w-[300px] lg:w-[500px] h-[220px]  lg:h-[350px]  m-auto" src={nativoLogo} />
            </div>
            <div className="w-full grid-cols-1 lg:px-10 px-4">
              <h1 className="lg:text-3xl text-2xl font-medium text-gray-900 w-full mb-4">Acerca de nosotros</h1>
              <p className="lg:text-xl text-base w-full text-justify">Nativo NFT es un mercado de NFT's montado sobre NEAR protocol. Actualmente se encuentra en beta pública</p>
            </div>
          </div>
          </div>
      </section>
      <section className="text-gray-600 body-font bg-gray-100">
        <div className="grid lg:grid-cols-2 grid-cols-1 w-full place-items-center text-center place-content-around">
          <div className="w-full grid-cols-1 lg:px-10 px-4 ">
            <h1 className="lg:text-3xl text-2xl font-medium text-gray-900 w-full mb-4 mt-10 lg:mt-2">Nativo DAO... Muy pronto</h1>
            <p className="lg:text-xl text-base w-full text-justify">Nuestro roadmap tiene contemplado el lanzamiento de un DAO para gobernanza el cual pueda utilizar los tokens de gobernanza $NTV para el manejo del protocolo de Nativo NFT.
              En un primer momento este DAO estará controlado por miembros del equipo fundador, y desde donde se podrán realizar actualizaciones al smart contract...
            </p>
          </div>
          <div className="w-full">
            <img className="p-[50px] lg:px-32 w-[300px] lg:w-[560px] h-[300px]  lg:h-[450px]  m-auto" src={arteNativo} />
          </div>
        </div>
      </section>
      </div>
  );
}

LightStatisicC.defaultProps = {
  theme: "indigo",
};

LightStatisicC.propTypes = {
  theme: PropTypes.string.isRequired,
};

export default LightStatisicC;
