import * as anchor from "@project-serum/anchor";
import * as serum from "@project-serum/serum";
import { Program } from "@project-serum/anchor";
import * as serumComm from "@project-serum/common";
import { Staking } from "../target/types/staking";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@project-serum/serum/lib/token-instructions";
import { expect } from "chai";

describe("staking_yield", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Staking as Program<StakingYield>;

  let mint: PublicKey = null;
  let tokenAcc: PublicKey = null;
  let stake: Keypair = null;
  let stakeVault: PublicKey = null;

  async function getTokenAccount(provider, addr) {
    return await serumComm.getTokenAccount(provider, addr);
  }

  async function createMint(provider: anchor.Provider, authority?: PublicKey) {
    if (authority === undefined) {
      authority = provider.wallet.publicKey;
    }

    // Create mint account
    const mint = anchor.web3.Keypair.generate();
    const instructions = [
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: provider.wallet.publicKey,
        newAccountPubkey: mint.publicKey,
        space: 82,
        lamports: await provider.connection.getMinimumBalanceForRentExemption(
          82
        ),
        programId: TOKEN_PROGRAM_ID,
      }),
      serum.TokenInstructions.initializeMint({
        mint: mint.publicKey,
        decimals: 0,
        mintAuthority: provider.wallet.publicKey,
      }),
    ];

    const tx = new anchor.web3.Transaction().add(...instructions);

    await provider.send(tx, [mint]);

    return mint.publicKey;
  }

  async function createAssociatedTokenAccount(
    provider: anchor.Provider,
    mint: PublicKey,
    owner: PublicKey
  ) {
    const tokenAcc = anchor.web3.Keypair.generate();
    const tx = new anchor.web3.Transaction().add(
      ...[
        anchor.web3.SystemProgram.createAccount({
          fromPubkey: provider.wallet.publicKey,
          lamports: await provider.connection.getMinimumBalanceForRentExemption(
            165
          ),
          newAccountPubkey: tokenAcc.publicKey,
          programId: TOKEN_PROGRAM_ID,
          space: 165,
        }),
        serum.TokenInstructions.initializeAccount({
          account: tokenAcc.publicKey,
          mint: mint,
          owner, // owner of the mint
        }),
      ]
    );

    await provider.send(tx, [tokenAcc]);

    return tokenAcc.publicKey;
  }

  it("Creates mint", async () => {
    mint = await createMint(anchor.getProvider());
    tokenAcc = await createAssociatedTokenAccount(
      anchor.getProvider(),
      mint,
      anchor.getProvider().wallet.publicKey
    );
  });

  it("Mint tokens to Alice", async () => {
    await program.rpc.mintTo(new anchor.BN(1000), {
      accounts: {
        authority: anchor.getProvider().wallet.publicKey,
        mint: mint,
        to: tokenAcc,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });

    expect(
      Number(
        await (
          await getTokenAccount(anchor.getProvider(), tokenAcc)
        ).amount
      )
    ).to.equal(1000);
  });

  it("Stakes 10 tokens", async () => {
    stakeVault = await createAssociatedTokenAccount(
      anchor.getProvider(),
      mint,
      anchor.getProvider().wallet.publicKey
    );

    stake = anchor.web3.Keypair.generate();
    const end_time = (Date.now() + 10) / 1000; // 1 second

    await program.rpc.createStake(new anchor.BN(10), new anchor.BN(end_time), {
      accounts: {
        authority: anchor.getProvider().wallet.publicKey,
        mint: mint,
        stake: stake.publicKey,
        stakeVault: stakeVault,
        staker: tokenAcc,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [stake],
    });

    expect(
      await (
        await getTokenAccount(anchor.getProvider(), stakeVault)
      ).amount.toNumber()
    ).to.equal(10);
    expect(
      await (
        await program.account.stake.fetch(stake.publicKey)
      ).amount.toNumber()
    ).to.equal(10);
    expect(
      await (
        await program.account.stake.fetch(stake.publicKey)
      ).staker.toString()
    ).to.equal(tokenAcc.toString());
  });

  it("Allows withdrawing tokens with an interest of 10% as 1s", async () => {
    await program.rpc.endStake({
      accounts: {
        authority: anchor.getProvider().wallet.publicKey,
        mint: mint,
        stake: stake.publicKey,
        stakeVault: stakeVault,
        staker: tokenAcc,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });

    // Received 10% interest
    expect(
      await (
        await getTokenAccount(anchor.getProvider(), tokenAcc)
      ).amount.toNumber()
    ).to.equal(1001);
  });
});

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}