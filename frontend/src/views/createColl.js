import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import { useFormik } from "formik";
import * as Yup from "yup";
import { ApolloClient, InMemoryCache, gql } from '@apollo/client'
import { acceptedFormats, currencys } from "../utils/constraint";
import Modal from "../components/modal.component";
import icon from "../assets/img/iconoColeccion.png"
import banner from "../assets/img/portadaColeccion.jpg"
import ImageUploader from 'react-images-upload';
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
import { Reader2, uploadFile2 } from '../utils/fleek2';
import Swal from 'sweetalert2'

function LightHeroE(props) {
  //este estado contiene toda la info de el componente
  const [mint, setmint] = React.useState({
    fileIcon: undefined,
    fileBanner: undefined,
    blockchain: localStorage.getItem("blockchain"),
  });

  function onDrop(picture) {
    console.log(picture[0])
    imageChangeIcon(picture)
  }

  
  const [combo, setcombo] = useState(true);
  const [title, setTitle] = useState("")
  const [desc, setDesc] = useState("")
  const [mediaIcon, setMediaIcon] = useState("")
  const [mediaBanner, setMediaBanner] = useState("")
  const [txtBttnIcon, setTxtBttnIcon] = useState("Seleccionar imagen")
  const [txtBttnBanner, setTxtBttnBanner] = useState("Seleccionar imagen")
  
  
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
  const APIURL = 'https://api.thegraph.com/subgraphs/name/luisdaniel2166/nativotest'

  //guardara todos los valores del formulario
  const pru = (parseInt(Math.random() * 100000) + 1);

  useEffect(() => {
    const valores = window.location.search;
    const values = new URLSearchParams(valores)
    if(values.has('transactionHashes')){
      window.location.href ="/create"
    }
  },[])

  const formik = useFormik({
    initialValues: {
      titleCol: "",
      descriptionCol: "",
    },
    //validaciones
    validationSchema: Yup.object({
      // title: Yup.string()
      //   .max(30, "Menos de 30 caracteres")
      //   .required("Requerido")
      //   .min(5, "el titulo debe longitud mayor a 5"),

      // description: Yup.string()
      //   .max(300, "Menos de 50 caracteres")
      //   .required("Requerido")
      //   .min(30, "la descripción minimo es de 30 caracteres"),

      // price: Yup.number()
      //   .required("Requerido")
      //   .positive("el precio debe ser positivo")
      //   .moreThan(0, "no existen nft gratis")
      //   .min(0.000000000000000001, "el precio minimo es un wei"),

      // culture: Yup.string().required(
      //   "Escribe el nombre de la cultura pertenenciente "
      // ),

      // country: Yup.string().required(
      //   "Escribe el nombre del pais pertenenciente "
      // ),

      // image: Yup.string().required("Requerido"),
    }),
    // onSubmit: async (values) => {
    //   // console.log('Entra')
    //   //evitar que el usuario pueda volver a hacer click hasta que termine el minado
    //   setmint({ ...mint, onSubmitDisabled: true });
    //   let account;
    //   if (mint.blockchain == "0") {
    //     //primero nos aseguramos de que la red de nuestro combo sea igual a la que esta en metamask
    //     await syncNets();

    //     //la cuenta a la cual mandaremos el token
    //     account = await getSelectedAccount();
    //     //console.log(account);
    //   }

    //   //cargamos el modal

    //   // console.log(JSON.stringify(values))
    //   const fecha = values.date.split('-')
    //   let dateSTR = fecha[1] + '-' + fecha[2] + '-' + fecha[0]
    //   // console.log(dateSTR)
    //   const date = new Date(dateSTR)
    //   date.setDate(date.getDate())
    //   date.setHours(values.hrs)
    //   date.setMinutes(values.min)
    //   if (date < Date.now()) {
    //     alert("La fecha y hora para la subasta debe de ser mayor a la fecha y hora actual")
    //     window.location.reload();
    //     return
    //   }
    //   let token;
    //   if (mint.blockchain == "0") {
    //     //los datos de la transacccion
    //     token = await getContract()
    //       .methods.minar(
    //         account,
    //         JSON.stringify(values),
    //         fromETHtoWei(values.price)
    //       )
    //       .send({ from: account })
    //       .catch((err) => {
    //         return err;
    //       });
    //   } else {
    //     let contract = await getNearContract();
    //     const data = await contract.account.connection.provider.block({
    //       finality: "final",
    //     });
    //     const dateActual = (data.header.timestamp) / 1000000;
    //     const owner = await getNearAccount()
    //     let payloadCol = {
    //       contr: "dev-1644523323613-61099606761670",
    //       addressowner: owner,
    //       title: values.titleCol,
    //       descrip: values.descriptionCol,
    //     }

    //     let colResult = contract.Add_user_collection(
    //       payloadCol
    //     )
    //     //console.log(contract);
    //     //console.log(payload);
    //     //console.log(fromYoctoToNear("5700000000000000000000"));

    //     // alert(payload);
    //     // let tokenresult = contract.minar(
    //     //   payload,
    //     //   300000000000000, // attached GAS (optional)
    //     //   amount
    //     // );


    //   }
    //   //if de error
    //   if (!token.status)
    //     setModal({
    //       ...modal,
    //       show: true,
    //       loading: false,
    //       title: "error",
    //       message: "intentalo de nuevo",
    //       change: setModal,
    //       disabled: false,
    //     });

    //   else
    //     setModal({
    //       ...modal,
    //       show: true,
    //       title: "Exito",
    //       message: "el nuevo token se ha minado correctamente",
    //       loading: false,
    //       change: setModal,
    //       disabled: false,
    //     });

    //   setmint({ ...mint, onSubmitDisabled: false });
    // },
  });

  async function saveCollection() {
    // console.log("Hola");
    let contract = await getNearContract();
    const owner = await getNearAccount()
    let payloadCol = {
      address_contract: "dev-1645632654382-28045928413066",
      address_collection_owner: owner,
      title: title.replace(/,/gi," "),
      descrip: desc.replace(/,/gi," "),
      mediaicon: mediaIcon,
      mediabanner: mediaBanner,
    }
    console.log(payloadCol);
    // console.log(desc);
    if (title == "" || desc == "" || mediaIcon == "" || mediaBanner == "") {
      Swal.fire({
        title: 'Datos incompletos',
        text: 'Para crear una colección es necesario que rellenes todos los campos, verifica que hayas rellenado todos los datos',
        icon: 'error',
        confirmButtonColor: '#E79211'
      })
      return
    } else if (title.length < 10) {
      Swal.fire({
        title: 'Titulo de la colección muy corto',
        text: 'El titulo de la coleccion debe de tener minimo 10 caracteres',
        icon: 'error',
        confirmButtonColor: '#E79211'
      })
      return
    } else if (desc.length < 30) {
      Swal.fire({
        title: 'Descripción de la colección muy corta',
        text: 'La descripción de la colección debe de tener minimo 30 caracteres',
        icon: 'error',
        confirmButtonColor: '#E79211'
      })
      return
    }
    let amount = fromNearToYocto(0.1);
    let colResult = await contract.add_user_collection(
      payloadCol,
      300000000000000,
      amount,
    )
    Swal.fire({
      title: 'Colección creada',
      text: 'Tu colección ha sido creada',
      icon: 'success',
    }).then(function () {
      window.location.href = "/create"
    })
  }

  /**
   * hace que cuando se toque la imagen se cambien el valor de touch de formik
   */
  function imageClickIcon() {
    formik.setFieldTouched("imageIcon");
  }
  function imageClickBanner() {
    formik.setFieldTouched("imageBanner");
  }
  /**
   * cada vez que el usuario cambia de archivo se ejecuta esta funcion
   *
   */
  function imageChangeIcon(picture) {
    setTxtBttnIcon("Cargando")
    let data = picture.pop()
    const { file, reader } = Reader2(data);
    console.log(data)
    if (file) {
      //asignar imagen de preview

      setmint({ ...mint, fileIcon: URL.createObjectURL(data) });

      //una vez que cargue el arhcivo lo mandamos a ipfs
      //una vez que cargue el arhcivo lo mandamos a ipfs

      //una vez que cargue
      reader.onloadend = function () {
        //subimos la imagen a ipfs
        uploadFile2(file.name, reader.result).then(({ hash }) => {
          // //console.log(result);
          //console.log(`https://ipfs.fleek.co/ipfs/${hash}`);
          formik.setFieldValue("image", hash);
          setMediaIcon(hash)
          setTxtBttnIcon("Cambiar imagen")
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



  function imageChangeBanner(picture) {
    setTxtBttnBanner("Cargando")
    let data = picture.pop()
    const { file, reader } = Reader2(data);
    if (file) {
      //asignar imagen de preview
      setmint({ ...mint, fileBanner: URL.createObjectURL(data) });

      //una vez que cargue el arhcivo lo mandamos a ipfs
      //una vez que cargue el arhcivo lo mandamos a ipfs
      //una vez que cargue
      reader.onloadend = function () {
        //subimos la imagen a ipfs
        console.log(this)
        uploadFile2(file.name, reader.result).then(({ hash }) => {
          // //console.log(result);
          //console.log(`https://ipfs.fleek.co/ipfs/${hash}`);
          formik.setFieldValue("image", hash);
          setMediaBanner(hash)
          setTxtBttnBanner("Cambiar imagen")
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
    <>
      <div className=" mx-auto text-gray-600 body-font flex flex-col mt-10">
        <div className="">
          <h1 className=" w-full title-font sm:text-4xl text-3xl mb-6 font-medium text-gray-900 text-center">
            Nueva Colección
          </h1>
          <div className="items-center px-6 xl:px-96">
            <div className="flex flex-col items-center bg-slate-200 bg-opacity-70 rounded-2xl border-4 border-slate-400 mb-4">
              <div className="w-full px-6 mb-6">
                <div className="flex justify-between">
                  <label
                    htmlFor="titleCol"
                    className="leading-7 text-sm text-gray-600 font-semibold"
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
                  placeholder="Min. 10 Caracteres"
                  onChange={e => { setTitle(e.target.value) }}
                  className={`  w-full bg-white  rounded   focus:bg-opacity-60  text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out `}
                />

                <div className="flex justify-between ">
                  <label
                    htmlFor="descriptionCol"
                    className="leading-7 text-sm text-gray-600 font-semibold"
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
                  placeholder="Min. 30 Caracteres"
                  value={desc}
                  onChange={e => { setDesc(e.target.value) }}
                  className={`  w-full bg-white  rounded   focus:bg-opacity-60  text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out `}
                />

              </div>
              <div className="w-full px-6 mb-6">
                <div className="flex flex-col lg:flex-row items-center text-center">
                  <div className="lg:w-1/5 w-full">
                    <label className="font-semibold">Icono de la colección</label>
                  </div>
                  <div className="lg:w-4/5 w-full">
                    <ImageUploader
                      withIcon={false}
                      buttonText={txtBttnIcon}
                      onChange={imageChangeIcon}
                      imgExtension={['.jpg', '.gif', '.png', '.gif', '.jpeg']}
                      maxFileSize={5242880}
                      singleImage={true}
                      withLabel={true}
                      label="Maximo 5mb, formatos aceptados .jpg, .gif, .png"
                      fileSizeError="El tamaño no puede superar los 5mb"
                      fileTypeError="Tipo de archivo no soportado"
                    />
                  </div>

                </div>
                <div className="flex flex-col lg:flex-row items-center text-center">
                  <div className="lg:w-1/5 w-full">
                    <label className="font-semibold">Portada de la colección</label>
                  </div>
                  <div className="lg:w-4/5 w-full">
                    <ImageUploader
                      withIcon={false}
                      buttonText={txtBttnBanner}
                      onChange={imageChangeBanner}
                      imgExtension={['.jpg', '.gif', '.png', '.gif', '.jpeg']}
                      maxFileSize={10485760}
                      singleImage={true}
                      withLabel={true}
                      label="Maximo 10mb, formatos aceptados .jpg, .gif, .png"
                      fileSizeError="El tamaño no puede superar los 10mb"
                      fileTypeError="Tipo de archivo no soportado"
                    />
                  </div>
                </div>
              </div>
            </div>

          </div>
          <div className="lg:w-full w-full px-6 mb-6 lg:mb-0 text-center">
            <p className="font-semibold">Vista previa de la colección en la parte inferior</p>
            <button
              onClick={() => saveCollection()}
              className={` mt-4 mb-4 text-white bg-${props.theme}-500 border-0 py-2 lg:px-6 px-2 focus:outline-none hover:bg-${props.theme}-600 rounded text-lg`}
            >
              {combo ? "Crear colección" : "Subastar"}
            </button>
            <p className="font-semibold">Tu imagen puede durar un poco de tiempo en mostrarse</p>
          </div>
        </div>
        <div className="">
          <div className={`container px-5 pt-6 mx-auto flex flex-wrap flex-col text-center items-center `}>
            <img
              className="object-cover h-96 w-full rounded-3xl  z-0 opacity-80 brightness-[.75] blur-sm"
              src={mediaBanner == "" ? icon : `https://ipfs.io/ipfs/${mediaBanner}`}
            />
            <img
              className="object-cover h-48 w-48 rounded-3xl border-solid border-4 border-slate-700 z-10 -mt-96"
              src={mediaIcon == "" ? banner : `https://ipfs.io/ipfs/${mediaIcon}`}
            />
            <div className="z-10 -mt-120 w-full text-white">
              <div className="bg-white lg:mx-20 mx-5 text-black mt-4 pt-2 rounded-t-2xl bg-opacity-80">
                <h1 className="lg:text-5xl text-3xl font-bold pb-4 opacity-100 stroke-gray-700">{title == "" ? "Titulo de la coleccion" : title}</h1>
                <p className="lg:text-xl text-base px-2 pb-3 stroke-gray-700">{desc == "" ? "Escribe la descripcion de tu coleccion" : desc}</p>
                <div className="grid grid-cols-2 divide-x pb-3 mx-auto stroke-gray-700">
                  <div>
                    <p className="lg:text-xl text-base pb-1 lg:text-right text-center lg:mr-5 ml-1"><b>Creador:</b><br/>Tu cuenta</p>
                  </div>
                  <div>
                    <p className="lg:text-xl text-base pb-1 lg:text-left text-center lg:ml-5 mr-1"><b>Contrato:</b><br/>Este es tu contrato</p>
                  </div>
                </div>
              </div>
              <div className="grid grid-cols-3 divide-x gap-1 bg-yellow-400 rounded-b-2xl text-white lg:mx-20 mx-5 mx-auto text-center">
                <div className="pl-5">
                  <p className="lg:text-lg text-base pb-1"><b>No. de tokens:</b></p>
                  <p className="lg:text-base text-sm pb-1">0</p>
                </div>
                <div>
                  <p className="lg:text-lg text-base pb-1"><b>No. de ventas:</b></p>
                  <p className="lg:text-base text-sm pb-1">0</p>
                </div>
                <div className="pr-5">
                  <p className="lg:text-lg text-base pb-1"><b>Vol. de venta:</b></p>
                  <p className="lg:text-base text-sm pb-1">0 {currencys[parseInt(localStorage.getItem("blockchain"))]}</p>
                </div>
              </div>
            </div>

          </div>
        </div>
      </div>
    </>
  );
}

LightHeroE.defaultProps = {
  theme: "yellow",
};

LightHeroE.propTypes = {
  theme: PropTypes.string.isRequired,
};

export default LightHeroE;
