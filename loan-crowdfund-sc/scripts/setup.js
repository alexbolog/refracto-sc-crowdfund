const BigNumber = require("bignumber.js");
const { signAndSendTx, getSmartContract } = require("./common");
const {
  OptionalValue,
  U8Value,
  AddressValue,
  Address,
  TokenIdentifierValue,
  BooleanValue,
  TokenTransfer,
} = require("@multiversx/sdk-core/out");

const setup = async () => {
  const contract = await getSmartContract();
  console.log("Initialized contract");

  console.log("Issuing token");
  await issueToken(contract);
  console.log("Token Issued");
};

const issueToken = async (contract) => {
  const tokenDisplayName = "REFRACTOLoanShare";
  const tokenTicker = "REFRACTO";

  const interaction = contract.methods
    .issueAndSetRoles([tokenDisplayName, tokenTicker])
    .withValue(new BigNumber(0.05).shiftedBy(18))
    .withGasLimit(60_000_000);
  await signAndSendTx(interaction);
};

setup().then(() => console.log("Done"));
