`💻 Tecnologías utilizadas`
==========

para el desarrollo del smart contract se utilizo el siguiente stack de tecnologías:
1. truffle framework 
2. node.js versión 12 
3. Solidity
4. MetaMask (extensión del navegador y app móvil) 
5. red de [aurora]

`👨🏻‍💻 instalación local` 
=================

1. para instalar el proyecto de forma local primero tenemos que asegurarnos de tener node js instalado en nuestro equipo, para eso es recomendado usar la herramienta de [nvm] (node version manager) e instalamos la versión 12

```bash
nvm install 12.22.1
```
2. instalamos de forma global el framework de truffle
```bash
npm install -g truffle
```
3. instalamos las dependencias del archivo `package.json` dentro del directorio `/blockchain`
```bash
npm install 
```

`👩🏻‍💻 despliegue del smart contract en la red de` [aurora]
================

una vez instalado de forma local el proyecto es necesario desplegar el contrato en la red de [aurora], para esto primero tendremos que configurar el proyecto para usar nuestra address de la wallet de metamask asi como el seed phrase para esto configuraremos dos archivos:

1. configuramos el `truffle-config.js` dentro del directorio `/blockchain` y cambiamos el address por el propio 

![picture](https://github.com/cristian-cloudmex/NFT-culturas-latinas/blob/master/assets/cambiar_address.png?raw=true)

2. configuramos el archivo `.secret` dentro del directorio `/blockchain` y escribimos nuestra seed phrase (frase de 12 palabras) como texto plano 

![picture](https://github.com/cristian-cloudmex/NFT-culturas-latinas/blob/master/assets/seed_phrase.png?raw=true)

con todo configurado pasamos a ejecutar el siguiente `script` estando dentro del directorio `/blockchain` para desplegar el contrato
```bash
npm run deploy:aurora
```
este comando desplegará el smart contract en la red de [aurora] para que podamos interactuar posteriormente con él.

![picture](https://github.com/cristian-cloudmex/NFT-culturas-latinas/blob/master/assets/despliegue.png?raw=true)

`interactuar con el smart contract`
================

para interactuar con el smart contract desplegado es necesario hacer una instancia del contrato desde la consola de truffle para eso usaremos los siguientes comandos:

```bash
truffle console --network auroraTestnet
```

```bash
const cvt = await MarketPlace.deployed()
```

![picture](https://github.com/cristian-cloudmex/NFT-culturas-latinas/blob/master/assets/truffle.png?raw=true)

`📃 Métodos del smart contract`
===============
para ejecutar estos metodos primero debemos tener una `instancia del contrato corriendo en la red de aurora` 

Método para minar un nuevo token nft 
------------------

```bash
await cvt.minar("address", "data", precio)
```

Método para transferir un token nft 
-----------------
```bash
await cvt.transferirNft("address", tokenid)
```

Método para listar los token nft a la venta
-----------------
```bash
await cvt.obtenerNfts()
```

Método para comprar un token nft
----------------
```bash
await cvt.comprarNft(tokenid)
```

Método para listar mis tokens nft adquiridos
---------------
```bash
await cvt.tokensOf("address")
```

Método para quitar de la venta uno de mis token nft 
---------------
```bash
await cvt.quitarDelMarketPlace(tokenid)
```

Método para revender un token nft
---------------
```bash
await cvt.revender(tokenid, precio)
```

`arbol de archivos`
================

```bash
├── readme.md                                   #este archivo
├── contracts                                   #directorio con los smart contract en solidity
│   ├── marketplace.sol
│   └── migrations.sol
├── migrations                                  #migraciones del contrato
│   ├── 1_initial_migration.js
│   └── 2_market_place.js
├── package-lock.json
├── package.json                                #archivo de dependencias del proyecto
├── test                                        #directorio de los test del smart contract
│   └── market_place.js
├── truffle-config.js                           # archivo de configuración de truffle
├── utils
│   └── exception.js
└── yarn.lock
```

[nvm]:  https://github.com/nvm-sh/nvm
[aurora]: https://aurora.dev/