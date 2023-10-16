require("dotenv").config();
const { promises } = require("fs");
const {
  ApiNetworkProvider,
  ProxyNetworkProvider,
} = require("@multiversx/sdk-network-providers");
const { parseUserKey, UserSigner } = require("@multiversx/sdk-wallet");
const {
  Account,
  ResultsParser,
  AbiRegistry,
  SmartContract,
  Address,
  TransactionWatcher,
  Code,
  CodeMetadata,
  TokenIdentifierValue,
  AddressValue,
  StringValue,
  BooleanValue,
  U32Value,
  U64Value,
  U8Value,
} = require("@multiversx/sdk-core");
const config = require("./config.json");

// load pem content and account
// sign and send tx
// automatic load network and pem files
// provide deploy, upgrade functions
const loadNetworkConfig = () => {
  const workingEnv = process.env.ENVIRONMENT;
  const chainId = process.env[`NETWORKS_${workingEnv}_CHAIN`];
  const gateway = process.env[`NETWORKS_${workingEnv}_GATEWAY`];
  const api = process.env[`NETWORKS_${workingEnv}_API`];
  const pem = process.env[`PEM_${workingEnv}`];

  return {
    chain: chainId,
    gateway,
    api,
    pem,
  };
};

const networkCfg = loadNetworkConfig();

const getPemAndAccount = async () => {
  const apiProvider = new ApiNetworkProvider(networkCfg.api);
  const pemContent = await loadPemContent(networkCfg.pem);
  const account = await loadUserAccount(apiProvider, pemContent);
  return {
    pem: pemContent,
    account,
  };
};

const loadPemContent = async (path) => {
  let buffer = await promises.readFile(path);
  return buffer.toString();
};

const loadUserAccount = async (apiProvider, walletPemContents) => {
  const userKey = parseUserKey(walletPemContents);
  const address = userKey.generatePublicKey().toAddress();

  const account = new Account(address);
  const apiAccount = await apiProvider.getAccount(address);
  account.update(apiAccount);
  return account;
};

const Parser = new ResultsParser();

const getSmartContract = async (address) => {
  const scAddress = address ?? config.address[process.env.ENVIRONMENT];
  const abiJson = await promises.readFile(process.env.SC_ABI_FILE_PATH, {
    encoding: "utf8",
  });
  const abiObj = JSON.parse(abiJson);
  const abiRegistry = AbiRegistry.create(abiObj);
  return new SmartContract({
    address: new Address(scAddress),
    abi: abiRegistry,
  });
};

const signAndSendExplicit = async (tx, walletPemContents) => {
  const provider = getProxyProvider();
  const signer = prepareUserSigner(walletPemContents);
  const serializedTransaction = tx.serializeForSigning();
  const signature = await signer.sign(serializedTransaction);
  tx.applySignature(signature);
  await provider.sendTransaction(tx);
  console.log(`Transaction sent. Tx hash: ${tx.getHash().toString()}`);
  const watcher = new TransactionWatcher(provider);
  const transactionOnNetwork = await watcher.awaitCompleted(tx);
  return transactionOnNetwork;
};

const prepareUserSigner = (walletPemContents) => {
  return UserSigner.fromPem(walletPemContents);
};

const getProxyProvider = () => {
  return new ProxyNetworkProvider(networkCfg.gateway);
};

const createCodeMetadata = (payable, payableBySc) => {
  return new CodeMetadata(true, true, payable, payableBySc);
};

const loadWasm = async () => {
  let buffer = await promises.readFile(process.env.SC_WASM_FILE_PATH);
  let code = Code.fromBuffer(buffer);
  return code;
};

const deploy = async () => {
  let contract = new SmartContract();
  let { pem, account } = await getPemAndAccount();
  let code = await loadWasm();
  let codeMetadata = createCodeMetadata(
    config.deploymentArgs.payable,
    config.deploymentArgs.payableBySc
  );

  const transaction = contract.deploy({
    deployer: account.address,
    code: code,
    codeMetadata: codeMetadata,
    initArguments: buildDeployArgs(),
    gasLimit: config.deploymentArgs.gasLimit,
    chainID: networkCfg.chain,
  });
  transaction.setNonce(account.getNonceThenIncrement());

  console.log(`Deploying contract on ${process.env.ENVIRONMENT}...`);
  const txResult = await signAndSendExplicit(transaction, pem);
  const deployedAddress = deploymentTransactionResultHandler(txResult);

  if (deployedAddress !== "") {
    config.address[process.env.ENVIRONMENT] = deployedAddress;
    await promises.writeFile("./config.json", JSON.stringify(config, null, 2));
  }
  console.log(`Deployment completed. Contract address: ${deployedAddress}`);
  return deployedAddress;
};

const upgrade = async (scAddress) => {
  let address = scAddress ?? config.address[process.env.ENVIRONMENT];
  if (!address) {
    console.log("Contract address not found. Please deploy first.");
    return;
  }
  let contract = await getSmartContract(address);
  let { pem, account } = await getPemAndAccount();
  let code = await loadWasm();
  let codeMetadata = createCodeMetadata(
    config.deploymentArgs.payable,
    config.deploymentArgs.payableBySc
  );

  const transaction = contract.upgrade({
    caller: account.address,
    code: code,
    codeMetadata: codeMetadata,
    initArguments: buildDeployArgs(),
    gasLimit: config.deploymentArgs.gasLimit,
    chainID: networkCfg.chain,
  });
  transaction.setNonce(account.getNonceThenIncrement());

  console.log(`Upgrading contract on ${process.env.ENVIRONMENT}...`);
  const txResult = await signAndSendExplicit(transaction, pem);
  // const deployedAddress = deploymentTransactionResultHandler(txResult);

  // if (deployedAddress !== "") {
  //   config.address[process.env.ENVIRONMENT] = deployedAddress;
  //   await promises.writeFile("./config.json", JSON.stringify(config, null, 2));
  // }
  console.log(`Upgrade completed. Contract address: ${deployedAddress}`);
  return deployedAddress;
};

const deploymentTransactionResultHandler = (transactionResult) => {
  if (transactionResult.status.status !== "success") {
    console.log("Transaction failed", transactionResult);
    return "";
  } else {
    console.log(
      "Deployment successful. Contract address: ",
      transactionResult.logs.events[0].address.value
    );
    return transactionResult.logs.events[0].address.value;
  }
};

const buildDeployArgs = () => {
  const args = [];
  config.deploymentArgs[process.env.ENVIRONMENT].forEach((arg) => {
    switch (arg.type) {
      case "TokenIdentifier":
        args.push(new TokenIdentifierValue(arg.value));
        break;
      case "ManagedAddress":
        args.push(new AddressValue(new Address(arg.value)));
        break;
      case "ManagedBuffer":
        args.push(new StringValue(arg.value));
        break;
      case "bool":
        args.push(new BooleanValue(arg.value === "true" ? true : false));
        break;
      case "u8":
        args.push(new U8Value(arg.value));
        break;
      case "u32":
        args.push(new U32Value(arg.value));
        break;
      case "u64":
        args.push(new U64Value(arg.value));
        break;
    }
  });
  return args;
};

const signAndSendTx = async (txInteraction) => {
  let { pem, account } = await getPemAndAccount();
  let tx = txInteraction
    .withChainID(networkCfg.chain)
    .withSender(account.address)
    .buildTransaction();

  tx.setNonce(account.getNonceThenIncrement());
  let txResult = await signAndSendExplicit(tx, pem);

  return (
    !txResult.status.isFailed() &&
    !txResult.status.isInvalid() &&
    !txResult.status.isPending()
  );
};

module.exports = {
  loadNetworkConfig,
  getPemAndAccount,
  getSmartContract,
  resultsParser: Parser,
  signAndSendTx,
  getProxyProvider,
  signAndSendExplicit,

  deploy,
  upgrade,
};
