import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import { useFormik } from "formik";
import * as Yup from "yup";
import { ApolloClient, InMemoryCache, gql } from '@apollo/client'
import { acceptedFormats, currencys } from "../utils/constraint";
import Modal from "../components/modal.component";
import {
  addNetwork,
  fromETHtoWei,
  getContract,
  getSelectedAccount,
  syncNets,
  syncNetworks,
} from "../utils/blockchain_interaction";
import {
  estimateGas,
  fromNearToEth,
  fromNearToYocto,
  fromYoctoToNear,
  getNearAccount,
  getNearContract,
  storage_byte_cost,
} from "../utils/near_interaction";
import { Reader, uploadFile } from '../utils/fleek';
import Swal from 'sweetalert2'

function LightHeroE(props) {
  //este estado contiene toda la info de el componente
  const [mint, setmint] = React.useState({
    file: undefined,
    blockchain: localStorage.getItem("blockchain"),
  });

  const [combo, setcombo] = useState(true);
  const [title, setTitle] = useState("")
  const [desc, setDesc] = useState("")
  //guarda el estado de el modal
  const [modal, setModal] = React.useState({
    show: false,
    title: "cargando",
    message: "hola como estas",
    loading: true,
    disabled: true,
  });

  const [actualDate, setactualDate] = useState("");
  let collectionData
  const APIURL = 'https://api.thegraph.com/subgraphs/name/luisdaniel2166/nativo3'

  //guardara todos los valores del formulario
  const pru = (parseInt(Math.random() * 100000) + 1);

  const formik = useFormik({
    initialValues: {
      titleCol: "",
      descriptionCol: "",
    },
    //validaciones
    validationSchema: Yup.object({
      title: Yup.string()
        .max(30, "Menos de 30 caracteres")
        .required("Requerido")
        .min(5, "el titulo debe longitud mayor a 5"),

      description: Yup.string()
        .max(300, "Menos de 50 caracteres")
        .required("Requerido")
        .min(30, "la descripción minimo es de 30 caracteres"),

      price: Yup.number()
        .required("Requerido")
        .positive("el precio debe ser positivo")
        .moreThan(0, "no existen nft gratis")
        .min(0.000000000000000001, "el precio minimo es un wei"),

      culture: Yup.string().required(
        "Escribe el nombre de la cultura pertenenciente "
      ),

      country: Yup.string().required(
        "Escribe el nombre del pais pertenenciente "
      ),

      image: Yup.string().required("Requerido"),
    }),
    onSubmit: async (values) => {
      console.log('Entra')
      //evitar que el usuario pueda volver a hacer click hasta que termine el minado
      setmint({ ...mint, onSubmitDisabled: true });
      let account;
      if (mint.blockchain == "0") {
        //primero nos aseguramos de que la red de nuestro combo sea igual a la que esta en metamask
        await syncNets();

        //la cuenta a la cual mandaremos el token
        account = await getSelectedAccount();
        //console.log(account);
      }

      //cargamos el modal
      setModal({
        ...modal,
        show: true,
        title: "cargando",
        loading: true,
        disabled: true,
      });

      console.log(JSON.stringify(values))
      const fecha = values.date.split('-')
      let dateSTR = fecha[1] + '-' + fecha[2] + '-' + fecha[0]
      console.log(dateSTR)
      const date = new Date(dateSTR)
      date.setDate(date.getDate())
      date.setHours(values.hrs)
      date.setMinutes(values.min)
      if (date < Date.now()) {
        alert("La fecha y hora para la subasta debe de ser mayor a la fecha y hora actual")
        window.location.reload();
        return
      }
      let token;
      if (mint.blockchain == "0") {
        //los datos de la transacccion
        token = await getContract()
          .methods.minar(
            account,
            JSON.stringify(values),
            fromETHtoWei(values.price)
          )
          .send({ from: account })
          .catch((err) => {
            return err;
          });
      } else {
        let contract = await getNearContract();
        const data = await contract.account.connection.provider.block({
          finality: "final",
        });
        const dateActual = (data.header.timestamp) / 1000000;
        const owner = await getNearAccount()
        let payloadCol = {
          contr: "dev-1643397318707-12565509757416",
          addressowner: owner,
          title: values.titleCol,
          descrip: values.descriptionCol,
        }

        let colResult = contract.Add_user_collection(
          payloadCol
        )
        //console.log(contract);
        //console.log(payload);
        //console.log(fromYoctoToNear("5700000000000000000000"));

        // alert(payload);
        // let tokenresult = contract.minar(
        //   payload,
        //   300000000000000, // attached GAS (optional)
        //   amount
        // );


      }
      //if de error
      if (!token.status)
        setModal({
          ...modal,
          show: true,
          loading: false,
          title: "error",
          message: "intentalo de nuevo",
          change: setModal,
          disabled: false,
        });

      else
        setModal({
          ...modal,
          show: true,
          title: "Exito",
          message: "el nuevo token se ha minado correctamente",
          loading: false,
          change: setModal,
          disabled: false,
        });

      setmint({ ...mint, onSubmitDisabled: false });
    },
  });

  async function saveCollection() {
    console.log("Hola");
    let contract = await getNearContract();
    const owner = await getNearAccount()
    let payloadCol = {
      contr: "dev-1643659132538-80320824962807",
      addressowner: owner,
      title: title,
      descrip: desc,
    }
    console.log(desc);
    let colResult = await contract.Add_user_collection(
      payloadCol
    )
    Swal.fire({
      title: 'Colección creada',
      text: 'Tu colección ha sido creada',
      icon: 'success',
    }).then(function() {
      window.location.href = "/minar"
    })
  }

  /**
   * hace que cuando se toque la imagen se cambien el valor de touch de formik
   */
  function imageClick() {
    formik.setFieldTouched("image");
  }
  /**
   * cada vez que el usuario cambia de archivo se ejecuta esta funcion
   *
   */
  function imageChange(e) {
    const { file, reader } = Reader(e);

    if (file) {
      //asignar imagen de preview
      setmint({ ...mint, file: URL.createObjectURL(e.target.files[0]) });

      //una vez que cargue el arhcivo lo mandamos a ipfs
      //una vez que cargue el arhcivo lo mandamos a ipfs

      //una vez que cargue
      reader.onloadend = function () {
        //subimos la imagen a ipfs
        uploadFile(file.name, reader.result).then(({ hash }) => {
          // //console.log(result);
          //console.log(`https://ipfs.fleek.co/ipfs/${hash}`);
          formik.setFieldValue("image", hash);
        })

      };
    }
    /*  //si selecciono un archivo, evita que nos de error si el usuario decide cancelar la carga
     if (e.target.files[0]) {
       //asignar imagen de preview
       setmint({ ...mint, file: URL.createObjectURL(e.target.files[0]) });
 
       //una vez que cargue el arhcivo lo mandamos a ipfs
       const reader = new FileReader();
       reader.readAsArrayBuffer(e.target.files[0]);
 
       //una vez que cargue
       reader.onloadend = async function () {
         //subimos la imagen a ipfs
         window.ipfs.add(reader.result).then(async (result) => {
           console.log(result);
           console.log(`https://ipfs.io/ipfs/${result.path}`);
 
           //agregamos el cid de ipfs  en el campo image
           formik.setFieldValue("image", result.path);
         });
       };
     } */
  }
  const format = (v) => {
    return v < 10 ? "0" + v : v;
  }
  const fechaActual = async () => {
    let contract = await getNearContract();
    const data = await contract.account.connection.provider.block({
      finality: "final",
    });
    const dateActual = new Date((data.header.timestamp) / 1000000);
    const fs = format(dateActual.getFullYear()) + "-" + (format(dateActual.getMonth() + 1)) + "-" + format(dateActual.getDate());
    //console.log(fs)
    setactualDate(fs)
  }

  return (
    <section className="text-gray-600 body-font">
      <form
        onSubmit={formik.handleSubmit}
        className="container mx-auto flex px-5 py-24 md:flex-row flex-col items-center"
      >

        <div className="lg:flex-grow md:w-1/2 lg:pl-24 md:pl-16 flex flex-col md:items-start md:text-left items-center text-center">
          <h1 className=" w-full title-font sm:text-4xl text-3xl mb-12 font-medium text-gray-900 text-center">
            Nueva Colección
          </h1>
          <div className="flex w-full md:justify-start justify-center items-end">
            <div className="relative mr-4 lg:w-full xl:w-1/2 w-3/4">
              {/* <select onChange={e=>{
                setcombo(e.target.value == "A la venta");
              }}>
                <option>A la venta</option>
                <option>En subasta</option>
              </select> */}


              <div className="flex justify-between ">
                <label
                  htmlFor="titleCol"
                  className="leading-7 text-sm text-gray-600"
                >
                  Título de la colección
                </label>
                {formik.touched.titleCol && formik.errors.titleCol ? (
                  <div className="leading-7 text-sm text-red-600">
                    {formik.errors.titleCol}
                  </div>
                ) : null}
              </div>

              <input
                type="text"
                id="titleCol"
                name="titleCol"
                {...formik.getFieldProps("titleCol")}
                value={title}
                onChange={e => { setTitle(e.target.value) }}
                className={`  w-full bg-gray-100 bg-opacity-50 rounded   focus:bg-transparent  text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out `}
              />

              <div className="flex justify-between ">
                <label
                  htmlFor="descriptionCol"
                  className="leading-7 text-sm text-gray-600"
                >
                  Descripción de la colección
                </label>
                {formik.touched.descriptionCol && formik.errors.descriptionCol ? (
                  <div className="leading-7 text-sm text-red-600">
                    {formik.errors.descriptionCol}
                  </div>
                ) : null}
              </div>
              <input
                type="text"
                id="titleCol"
                name="titleCol"
                {...formik.getFieldProps("titleCol")}
                value={desc}
                onChange={e => { setDesc(e.target.value) }}
                className={`  w-full bg-gray-100 bg-opacity-50 rounded   focus:bg-transparent  text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out `}
              />



              <div className="relative flex py-5 items-center">
                <div className="flex-grow border-t border-gray-200"></div>

                <div className="flex-grow border-t border-gray-200"></div>
              </div>




              <button
                onClick={() => saveCollection()}
                className={` mt-12 w-full text-white bg-${props.theme}-500 border-0 py-2 px-6 focus:outline-none hover:bg-${props.theme}-600 rounded text-lg`}
              >
                {combo ? "Crear colección" : "Subastar"}
              </button>
            </div>
          </div>
        </div>
      </form>
      <Modal {...modal} />
    </section>
  );
}

LightHeroE.defaultProps = {
  theme: "yellow",
};

LightHeroE.propTypes = {
  theme: PropTypes.string.isRequired,
};

export default LightHeroE;
