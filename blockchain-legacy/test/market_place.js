const MarketPlace = artifacts.require("MarketPlace");
let tryCatch = require("../utils/exception").tryCatch;
let errTypes = require("../utils/exception").errTypes;

contract("MarketPlace", async function (accounts) {
  /**
   * este test comprobara que quien mino el contrato sea equivalente a la primera direccion
   */
  it("la primera cuenta deberia ser el minero del contrato", async function () {
    const instance = await MarketPlace.deployed();
    const minero = await instance.minero();
    assert.isTrue(minero == accounts[0], "otra persona desplego el sm");
  });

  /**
   * se crea un nuevo token y se revisa que la transacción fue correcta
   */
  it("deberia poder crearse un nuevo token", async function () {
    const instance = await MarketPlace.deployed();
    const receipt = await instance.minar(accounts[1], "datos", 1000);
    return assert.isTrue(
      receipt.receipt.status,
      "el minado del nuevo token no se efectuo correctamente"
    );
  });

  /**
   * se revisa que al comprar un token se transfiera y tambien que el dueño reciba los fondos
   */
  it("el balance del primer usuario debe aumentar una vez que venda ", async function () {
    const instance = await MarketPlace.deployed();
    let etherOnWei = web3.utils.toWei("1", "ether");

    //esto creara el token con el tokenid igual a 1
    const receipt = await instance.minar(accounts[0], "datos", etherOnWei);

    //obtener el balance del usuario que acaba de minar
    const user1Balance = await web3.eth.getBalance(accounts[0]);
    //el segundo usuario comprara el nft del primer usuario por un ether
    await instance.comprarNft(1, {
      from: accounts[1],
      value: etherOnWei,
    });

    const user1Ntokens = (await instance.balanceOf(accounts[0])).toNumber();

    const user1BalanceAfter = await web3.eth.getBalance(accounts[0]);

    assert.isTrue(
      user1Ntokens === 0,
      "no se transferio correctamente el token"
    );
    assert.isTrue(
      user1BalanceAfter - user1Balance == etherOnWei,
      "no se le mando correctamente el pago"
    );
  });

  /**
   * se asegura de que el tokenid le pertenece a la cuenta que acaba de comprar el token
   */
  it("el dueño del segundo token deberia ser la segunda cuenta", async () => {
    const instance = await MarketPlace.deployed();
    const owner = await instance.ownerOf(1);

    assert.isTrue(
      accounts[1] == owner,
      "no se le transfirio el token a la seunda cuenta"
    );
  });

  /**
   * se comprueba que el smart contract tenga dos tokens
   */
  it("deberia tener dos tokens el contrato", async function () {
    const instance = await MarketPlace.deployed();
    const ntotalTokens = await instance.totalSupply();
    return assert.isTrue(
      ntotalTokens.toNumber() == 2,
      "no se creo correctamente algun token"
    );
  });

  it("el token comprado deberia tener la propiedad de en venta en falso", async () => {
    const instance = await MarketPlace.deployed();
    const onsale = (await instance.tokensData(1)).onSale;

    assert.isTrue(!onsale, "no se le asigno  falso a la propiedad onSale");
  });

  it("no deberia ser posible comprar el toknen ", async () => {
    const instance = await MarketPlace.deployed();
    let etherOnWei = web3.utils.toWei("1", "ether");

    //el primer usuario no deberia poder comprar este token
    await tryCatch(
      instance.comprarNft(1, {
        from: accounts[1],
        value: etherOnWei,
      }),
      errTypes.revert
    );
  });

  it("el primer usuario deberia ser dueño del tokenid 1 ", async () => {
    const instance = await MarketPlace.deployed();

    await instance.transferirNft(accounts[0], 1, { from: accounts[1] });
    const tokenId1owner = await instance.ownerOf(1);
    assert.equal(
      accounts[0],
      tokenId1owner,
      "a pesar de transferirle el token al usuario 1 el token este no es dueño de "
    );
  });

  it("el token 1 deberia estar disponible para la venta", async () => {
    const instance = await MarketPlace.deployed();
    const onsaleBefore = (await instance.tokensData(1)).onSale;
    await instance.revender(1, 0, { from: accounts[0] });
    const onsale = (await instance.tokensData(1)).onSale;
    assert.isTrue(!onsaleBefore, "el token no deberia estar a la venta");
    assert.isTrue(onsale, "revender no cambio el estado del token");
  });
});
