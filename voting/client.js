const anchor = require("@project-serum/anchor");

// Configure the local cluster.
// anchor.setProvider(anchor.Provider.local());

// console.log(anchor.Provider.local());
// console.log(anchor.Provider.local("http://127.0.0.1:8899"));

const provider = anchor.Provider.local("http://127.0.0.1:8899")
anchor.setProvider(provider);

const base_account = anchor.web3.Keypair.generate();
// const voteAccount = anchor.web3.Keypair.generate();
// const updateAccount = anchor.web3.Keypair.generate();


async function main() {
  // #region main
  // Read the generated IDL.
  const idl = JSON.parse(
    require("fs").readFileSync("./target/idl/voting.json", "utf8")
  );

  // Address of the deployed program.
  const programId = new anchor.web3.PublicKey("YCGYEbSaZ9NHAMu3hk6hgEuSHtxd81d1EBhbUYhGDB5");

  // Generate the program client from IDL.
  const program = new anchor.Program(idl, programId);
  await console.log("Base Account: ",base_account.publicKey.toString())

  // Execute the RPC.
  await program.rpc.initialize(
    ["proposal_1", "proposal_2", "proposal_3", "proposal_4"],
    {
      accounts: {
        accountWhitelist: base_account.publicKey,
        user: base_account.publicKey,
        proposalVotes: base_account.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      }
    }
    );
  // #endregion main
}

console.log("Running client.");
main().then(() => console.log("Success"));