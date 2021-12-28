import React from "react";
import PropTypes from "prop-types";

function LightStatisicC(props) {
  return (
    <section className="text-gray-600 body-font">
      <div className="container px-5 py-24 mx-auto ">
        <div className="flex flex-col text-center w-full mb-20">
          <h1 className="sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900">
            Información acerca de las culturas nativas en América Latina
          </h1>
        </div>
        <div className="flex justify-center -m-4 text-center sp-1">
          <div className="flex p-4 md:w-1/4 sm:w-1/2 w-full sp">
            <div className="border-2 border-gray-200 px-4 py-6 rounded-lg sp-3">
              <h2 className="title-font font-medium text-3xl text-gray-900">
                522
              </h2>
              <p className="leading-relaxed">Número actual de culturas nativas en América Latina.</p>
            </div>
          </div>
          <div className="flex p-4 md:w-1/4 sm:w-1/2 w-full sp">
            <div className="border-2 border-gray-200 px-4 py-6 rounded-lg sp-3">
              <h2 className="title-font font-medium text-3xl text-gray-900">
                8 %
              </h2>
              <p className="leading-relaxed">Porcentaje de la población que forma parte de alguna cultura nativa.</p>
            </div>
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
