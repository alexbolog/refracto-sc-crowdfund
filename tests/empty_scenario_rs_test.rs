use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/lib");

    blockchain.register_contract("file:output/loan-crowdfund-sc.wasm", loan_crowdfund_sc::ContractBuilder);
    blockchain
}

#[test]
fn lib_rs() {
    world().run("scenarios/lib.scen.json");
}
