 // SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract MarketPlace is ERC721Enumerable {
    using Counters for Counters.Counter;
    //representa un entero que lleva la cuenta de los tokensNFT
    Counters.Counter public _tokenIds;

    //matenera el dueño del contrato o quien lo mino
    address public minero;

    //es el numero de tokens en venta
    uint256 public nTokenOnSale;
    // es la representacion de un token
    struct tokenData {
        uint256 price;
        string data;
        bool onSale;
        uint256 tokenID;
        address creator;
    }

    //contiene todas las representacion de cada token
    mapping(uint256 => tokenData) public tokensData;

    constructor() ERC721("MarketPlace", "MPE") {
        minero = msg.sender;
    }

    /**
    mina nuevos tokens
	@param receptor {address} es a quien mandaremos el  token 
	@param datos    {string} es un json con todos los datos del token
	@param price    {uint256} the price in weis
    @return {uint256} equivale a el tokenid recien creado
	 */
    function minar(
        address receptor,
        string memory datos,
        uint256 price
    ) public returns (uint256) {
        //obtener el tokenid actual
        uint256 newItemId = _tokenIds.current();

        //aumentar en 1 el conteo de tokens
        _tokenIds.increment();

        //minar el token
        _mint(receptor, newItemId);

        //agregar los datos a nuestro mapping
        tokensData[newItemId].data = datos;
        tokensData[newItemId].price = price;
        tokensData[newItemId].onSale = true;
        tokensData[newItemId].tokenID = newItemId;
        tokensData[newItemId].creator = msg.sender;

        //agregar a la venta el nuevo token
        nTokenOnSale++;
        return newItemId;
    }

    /**
    * Función para transferir un token NFT del propietario a otro address
    @param to {address} representa a quien vamos a transferir nuestro NFT
    @param tokenId {uint256} este valor indica el id que vamos a transferir 
    @return {addresss}  esta función nos retorna el address del nuevo propietario del NFT
     */

    function transferirNft(address to, uint256 tokenId)
        public
        returns (address)
    {
        require(_exists(tokenId), "El token que intentas transferir no existe");
        require(msg.sender == ownerOf(tokenId), "Este token no te pertenece");

        //transferir el token NFT si el token id existe y es nuestro
        safeTransferFrom(msg.sender, to, tokenId);
        //retornar el nuevo owner del token NFT
        return to;
    }

    /** 
    * Función para eliminar un token NFT en tu address
    @param tokenId {uint256} representa el id del token NFT a eliminar
    @return {success} si el proceso fue llevado con exito la funcion retorna un true
     */
    function quemarNft(uint256 tokenId) public returns (bool) {
        require(_exists(tokenId), "El token que intentas quemar no existe");
        require(msg.sender == ownerOf(tokenId), "Este token no te pertenece");

        //Eliminamos el token NFT si el token id existe y es nuestro
        _burn(tokenId);

        return true;
    }

    /**
     * pone a la venta un token y si le das el precio lo cambia
     *@param tokenid {uint256} representa el identificador del tokenid
     *@param price  {uint256} representa el precio en wei al que se vendera
     *
     */
    function revender(uint256 tokenid, uint256 price) public {
        require(
            ownerOf(tokenid) == msg.sender,
            "solo el dueno del token puede venderlo"
        );
        require(price > 0, "no hay nfts gratis");
        tokensData[tokenid].onSale = true;
        tokensData[tokenid].price = price;
        nTokenOnSale++;
    }

    /**
     *cambia el estado del token para que este no pueda ser visualizado o vendido
     *@param tokenid {uint256} tokenid representa el token al cual afectar
     */
    function quitarDelMarketPlace(uint256 tokenid) public {
        //este require solo permite que el owner del tokenid pueda usar la funcion
        require(
            ownerOf(tokenid) == msg.sender,
            "solo el dueno del token puede venderlo"
        );
        // impide que accedamos a una posicion inexistente
        require(
            tokenid < _tokenIds.current(),
            "ese token todavia no ha sido creado"
        );
        tokensData[tokenid].onSale = false;
        nTokenOnSale--;
    }

    /**
     *regresa todos los nft disponibles
     *@return tokenData[] contiene todos los tokens disponibles
     */
    function obtenerNfts() public view returns (tokenData[] memory) {
        //es el numero de tokens
        uint256 nTokens = _tokenIds.current();
        //es un arreglo temporal con todos los nfts
        tokenData[] memory onSaleTokensData = new tokenData[](nTokens);
        //sacamos los nft del mapping y los almacenamos en el arreglo
        for (uint256 index = 0; index < nTokens; index++) {
            onSaleTokensData[index] = tokensData[index];
        }

        return onSaleTokensData;
    }

    /**
     *trata a tokensData como un arreglo divido entre nTokenP
     * el inicio de una pagina es el numero de pagina por el numero de tokens por pagina
     * @param nTokenP uint256 es el numero de tokens que tiene una pagina
     * @param nPagina contiene que pagina vamos a consultar
     *@return tokenData[] contiene los tokens de una pagina
     */
    function obtenerPaginav1(uint256 nTokenP, uint256 nPagina)
        public
        view
        returns (tokenData[] memory)
    {
        //es el incio del ciclo o el inicio de  la pagina
        uint256 inicioPagina = nPagina * nTokenP;
        //es un arreglo temporal con todos los nfts de la pagina
        tokenData[] memory onSaleTokensData = new tokenData[](nTokenP);
        //sacamos los nft y los almacenamos en el arreglo
        for (
            uint256 index = inicioPagina;
            index < nTokenP * (nPagina + 1);
            index++
        ) {
            //agregar tokens al arreglo
            onSaleTokensData[index - inicioPagina] = tokensData[index];
        }

        return onSaleTokensData;
    }

    /**
     *nos regresa los tokens que estan en venta hasta que se llene el arreglo o termine el ciclo
     * espera a llenar el contador hasta que es el incio de la pagina que solicito
     * @param nTokenP uint256 es el numero de tokens que tiene una pagina
     * @param nPagina contiene que pagina vamos a consultar
     *@return tokenData[] contiene los tokens de una pagina
     */
    function obtenerPaginav2(uint256 nTokenP, uint256 nPagina)
        public
        view
        returns (tokenData[] memory)
    {
        //es el numero de tokens
        uint256 nTokens = _tokenIds.current();
        //es la posicion del el ultimo token agregado a onsaletokendata
        uint256 lastToken = 0;
        //cuenta cuantos tokens en venta van
        uint256 ostok = 0;
        //a partir de cuantos tokens comienza
        //es un arreglo temporal con todos los nfts de la pagina
        tokenData[] memory onSaleTokensData = new tokenData[](nTokenP);
        //sacamos los nft y los almacenamos en el arreglo
        for (uint256 index = 0; index < nTokens; index++) {
            //si esta en venta
            if (tokensData[index].onSale) {
                //aumentamos el numero de tokens que estan a la venta
                ostok++;
                //si el numero de tokens es mayor o igual al incio de la pagina comenzamos a agregar tokens al arreglo
                if (ostok >= nTokenP * nPagina) {
                    onSaleTokensData[lastToken] = tokensData[index];
                    lastToken++;
                }
                //si el numero de tokens llega al fin de la pagina cancelamos el for
                if (ostok == nTokenP * (nPagina + 1)) break;
            }
            //agregar tokens al arreglo
        }

        return onSaleTokensData;
    }
    event  buyed ( address tokenon ,  uint  payout,  address tokencre ,  uint  reg, address mpowner ,  uint  gains    ); 

    /**
     *con esta funcion cualquiera puede comprar el nft que desea
     *@param tokenId {uint256} tokenid representa el tokenid a comprar
     */
    function comprarNft(uint256 tokenId) public payable {
        //este require se asegura que se mando el suficiente ether para comprar el token
        require(msg.value == tokensData[tokenId].price, "fondos insuficientes");
        //se impide la venta si no esta en venta el token
        require(
            tokensData[tokenId].onSale,
            "no se encuentra a la venta ese token"
        );
        //es el dueño del token
        address payable tokenOwner = payable(ownerOf(tokenId));
        //es el creador del token
        address payable tokenCreator = payable(tokensData[tokenId].creator);
        

        //mandamos el token al comprador
        _transfer(tokenOwner, msg.sender, tokenId);
        
        //calculo de regalias ,ganancias y pago al vendedor
        uint256 regal=msg.value * 10 / 100;
        uint256 gains=msg.value* 3 / 100;
        uint256 payout=msg.value*  87 / 100;
        //le transferimos el dinero al  vendedor
        tokenOwner.transfer(payout);
        //le transferimos las regalias al creator
        tokenCreator.transfer(regal);
        //le transferimos el comision al dueño del MarketPlace
        payable(minero).transfer(gains);
        //poner el token en pausa
        tokensData[tokenId].onSale = false;
   emit  buyed( tokenOwner ,  payout,  tokenCreator ,  regal, minero , gains   );
        nTokenOnSale--;
    }

    /**
     * @param owner {address}  representa la dirección al cual consultar los tokens
     * @return tokenData[] arreglo con todos los datos de cada token
     */
    function tokensOf(address owner) public view returns (tokenData[] memory) {
        //numero de tokens
        uint256 nTokens = balanceOf(owner);
        if (nTokens == 0) {
            return (new tokenData[](nTokens));
        }
        //arreglo con los tokens
        tokenData[] memory userTokens = new tokenData[](nTokens);

        for (uint256 i = 0; i < nTokens; i++) {
            //obtener los datos de cada uno de los tokens
            userTokens[i] = tokensData[tokenOfOwnerByIndex(owner, i)];
        }
        return (userTokens);
    }

    function tokensOfPaginav1(
        address owner,
        uint256 nTok_porpagina,
        uint256 pag
    ) public view returns (tokenData[] memory) {
        uint256 nUserTokens = balanceOf(owner);
        //calcular el inicio de la pagina
        uint256 inicioPagina = pag * nTok_porpagina;
        //si el fin es mayor que el numero de tokens
        uint256 finPagina = inicioPagina + nTok_porpagina;
        if (finPagina > nUserTokens) {
            finPagina = nUserTokens; // 4
        }
        //arreglo con los tokens del usuario
        tokenData[] memory userTokens = new tokenData[](nTok_porpagina);
        for (uint256 i = inicioPagina; i < finPagina; i++) {
            userTokens[i - inicioPagina] = tokensData[
                tokenOfOwnerByIndex(owner, i)
            ];
        }
        return userTokens;
    }
//metodo para la obtencion de la metadata de un token en especidifco
 function getItemInfo(uint256 _token)public view returns(tokenData[] memory){
      tokenData[] memory getone = new tokenData[](1);
        getone[0]=tokensData[ _token] ;
      return getone;
       
    }
//metodo para la obtencion de un rango de tokensData
  function obtenerNftsbyrango(uint256 _Mintoken,uint256 _Maxtoken) public view returns (tokenData[] memory) {
        //obetenemos el numero acutal de tokens
       
        require(_tokenIds.current()>0,"al menos debe haber un token");
        


    //es el numero de tokens
        uint256 nTokens = _tokenIds.current();
        //es un arreglo temporal con todos los nfts
         uint256 arr = _Maxtoken - _Mintoken;
        tokenData[] memory onSaleTokensData = new tokenData[](arr);
        //sacamos los nft del mapping y los almacenamos en el arreglo
        for (uint256 index = 0; index < nTokens; index++) {
            if(index >= _Mintoken && index <_Maxtoken ){
                 onSaleTokensData[index] = tokensData[index];
            }
             if (index == _Maxtoken  ) break;
           
        }

        return onSaleTokensData;
    }

}
