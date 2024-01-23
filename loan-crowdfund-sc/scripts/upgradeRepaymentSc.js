const { upgrade, getSmartContract, signAndSendTx } = require("./common");

const MAX_PROJECT_ID = 176;

const upgradeRepaymentSc = async (sc, projectId) => {
  const interaction = sc.methods
    .upgradeRepaymentSc([projectId])
    .withGasLimit(100_000_000);

  await signAndSendTx(interaction);
};

(async () => {
  const sc = await getSmartContract();

  for (let i = 133; i < MAX_PROJECT_ID; i++) {
    await upgradeRepaymentSc(sc, i);
  }
})();
