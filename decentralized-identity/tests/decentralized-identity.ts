const assert = require('assert');
const anchor = require('@project-serum/anchor');
const { PublicKey, SystemProgram } = anchor.web3;

describe("Decentralized Identity: ", function() {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.DecentralizedIdentity;

  it("username is created & mapped", async function() {
    // const user_account  = anchor.web3.Keypair.generate();
    const [accountPDA, accountBump] = await PublicKey.findProgramAddress([anchor.utils.bytes.utf8.encode("testuser")], program.programId);
    // console.log('User Account: ',user_account.publicKey);
    console.log('PDA: ', accountPDA);
    console.log('Provider Wallet: ', provider.wallet.publicKey);

    await program.rpc.claimUsername("testuser", accountBump, {
      accounts: {
        authority: provider.wallet.publicKey,
        user: accountPDA,
        systemProgram: SystemProgram.programId,
      },
      signers: [provider.wallet]
    });
    const account = await program.account.userAccount.fetch(user_account.publicKey);
    console.log('Data: ', account);
    assert.ok(account.username === 'testuser');
    assert.ok(account.bump === accountBump);
  });
});