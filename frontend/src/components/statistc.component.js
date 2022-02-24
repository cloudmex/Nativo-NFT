import React from "react";
import PropTypes from "prop-types";
import nativoLogo from "../assets/img/nativologocrop.png"
import arteNativo from '../assets/img/arte nativo.png'
function LightStatisicC(props) {
  return (
    <section className="text-gray-600 body-font bg-gray-100">
      <div className="container px-5 lg:py-20 py:12 mx-auto ">
        <div className="grid lg:grid-cols-2 grid-cols-1 w-full mb-20 place-items-center text-center">
          <div className="w-full">
            <img className="lg:px-32 px-4 mb-4" src={nativoLogo}/>
          </div>
          <div className="w-full grid-cols-1 lg:px-10 px-4">
            <h1 className="lg:text-3xl text-xl font-medium text-gray-900 w-full mb-4">Acerca de nosotros</h1>
            <p className="lg:text-xl text-base w-full">Nativo NFT es un mercado de NFT's montado sobre NEAR protocol. Actualmente se encuentra en beta pública</p>
          </div>
        </div>
        <div className="grid lg:grid-cols-2 grid-cols-1 w-full mb-20 place-items-center text-center">
          <div className="w-full grid-cols-1 lg:px-10 px-4">
            <h1 className="lg:text-3xl text-xl font-medium text-gray-900 w-full mb-4">Nativo DAO</h1>
            <p className="lg:text-xl text-base w-full">Nuestro roadmap tiene contemplado el lanzamiento de un DAO para gobernanza el cual pueda utilizar los tokens de gobernanza $NTV para el manejo del protocolo de Nativo NFT.
En un primer momento este DAO estará controlado por miembros del equipo fundador, y desde donde se podrán realizar actualizaciones al smart contract.
</p>
          </div>
          <div className="w-full">
            <img className="lg:px-32 px-4 mb-4" src={arteNativo}/>
          </div>
        </div>
      </div>
    </section>
  );
}

LightStatisicC.defaultProps = {
  theme: "indigo",
};

LightStatisicC.propTypes = {
  theme: PropTypes.string.isRequired,
};

export default LightStatisicC;
