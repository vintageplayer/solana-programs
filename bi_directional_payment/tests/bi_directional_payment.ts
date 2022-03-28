import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BiDirectionalPayment } from "../target/types/bi_directional_payment";

describe("bi_directional_payment", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.BiDirectionalPayment as Program<BiDirectionalPayment>;
  
  const user1 = anchor.web3.Keypair.generate();
  const user2 = anchor.web3.Keypair.generate();
  const channel = anchor.web3.Keypair.generate();

  it("Creates a Channel!", async () => {
    // Add your test here.
    const balance1 = 10;
    const balance2 = 10;
    const expires_at = parseInt(Date.now()/1000);
    const challenge_period = 3600000;
    const users = [user1.publicKey, user2.publicKey];
    const balances = [balance1, balance2];

    const tx = await program.rpc.createChannel(
      users,
      balances,
      expires_at,
      challenge_period,
      {
        accounts: {
          channel: channel.publicKey,
          user: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [channel],
      }
    );
    console.log("Your transaction signature", tx);
    const channelAccount = await program.account.channel.fetch(channel.publicKey);
    console.log(channelAccount);
  });

  it("Can Challenge Exit!", async () => {
    // Add your test here.
    const balance1 = 12;
    const balance2 = 8;
    const balances = [balance1, balance2];
    const nonce = 2;
    const tx = await program.rpc.challengeExit(
      balances,
      nonce,
      {
        accounts: {
          channel: channel.publicKey,
          user1: user1.publicKey,
          user2: user2.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [user1, user2],
      }
    );
    console.log("Your transaction signature", tx);
    const channelAccount = await program.account.channel.fetch(channel.publicKey);
    console.log(channelAccount);
  });


  it("Can Withdraw Funds!", async () => {
    // Add your test here.
    const tx = await program.rpc.withdraw(
      {
        accounts: {
          channel: channel.publicKey,
          user: user1.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [user1],
      }
    );
    console.log("Your transaction signature", tx);
    const channelAccount = await program.account.channel.fetch(channel.publicKey);
    console.log(channelAccount);
  });

});
