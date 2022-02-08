/**
 * Archivo para la configuración del proyecto de truffle.
 */

const path = require("path");
// Importación del manejador de wallets
const HDWalletProvider = require("@truffle/hdwallet-provider");
// const infuraKey = "fj4jll3k.....";

//Obtención del mnemonico correspondiente a la seed phrase de nuestra wallet
const fs = require("fs");
const mnemonic = fs.readFileSync(".secret").toString().trim();

//Validación para permitir solo frases de 12 palabras
if (!mnemonic || mnemonic.split(" ").length !== 12) {
  throw new Error(
    "No se pudo encontrar un mnemonico aceptable en el archivo .secret"
  );
}

module.exports = {
  /**
   * Las redes defininen como conectarnos al cliente de aurora.
   * Ejemplo:
   * $ truffle test --network <network-name>
   */

  networks: {
    // Se incluye la red de aurora en la configuración de truffle
    auroraTestnet: {
      provider: () =>
        new HDWalletProvider(mnemonic, "https://testnet.aurora.dev"),
      network_id: 0x4e454153,
      gas: 10000000,
    },
    aurora: {
      provider: () =>
        new HDWalletProvider(mnemonic, "https://mainnet.aurora.dev"),
      network_id: 0x4e454152,
      gas: 10000000,
    },
  },

  // Set default mocha options here, use special reporters etc.
  mocha: {
    // timeout: 100000
  },
  contracts_build_directory: path.join(
    __dirname,
    ".././frontend/src/contracts"
  ),

  // Configure your compilers
  compilers: {
    solc: {
      version: "0.8.1", // Fetch exact version from solc-bin (default: truffle's version)
      // docker: true,        // Use "0.5.1" you've installed locally with docker (default: false)
      // settings: {          // See the solidity docs for advice about optimization and evmVersion
      //  optimizer: {
      //    enabled: false,
      //    runs: 200
      //  },
      //  evmVersion: "byzantium"
      // }
    },
  },
};
