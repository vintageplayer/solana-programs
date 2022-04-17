const anchor = require('@project-serum/anchor');
const assert = require('assert');
const { PublicKey, SystemProgram } = anchor.web3;
import { actions, utils, programs, NodeWallet } from "@metaplex/js";

describe("dynamic_nft_marketplace", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.DynamicNftMarketplace;

  // Address of NFTs minted using metaplex v2
  const mint_address = "AqxwEXM3Zf3KuKnHqv6BZCrguz2hkjtAiHmWqUtm5FwR";
  const master_edition = "PyxLurGi8Dd1HtrbVND4w7Mt2WYE16qytJWfMRR1hmr";
  const metadata_address = "BxthkqLLBcbaC57yfPGDPFQHoZahWcFjm37xsKrVYw8z";

  it("Updates Metadata", async () => {
      let current_metadata = await programs.metadata.Metadata.load(
        new Connection(clusterApiUrl("devnet")),
        metadata_address
      );

      console.log(`Current NFT Metadata: ${current_metadata.data.data}`);

      const new_metadata = {
      name: "Updated NFT",
      symbol: "NB",
      description: "Collection of 10 numbers on the blockchain. This is the number 1/10.",
      seller_fee_basis_points: 500,
      image: "0.png",
      attributes: [
          {trait_type: "Layer-1", value: "0"},
          {trait_type: "Layer-2", value: "0"}, 
          {trait_type: "Layer-3", value: "0"},
          {trait_type: "Layer-4", value: "1"}
      ],
      properties: {
          creators: [{address: "6CvtwfYLCDEzagVV99Dwvjrq6fLSBoh6bx7gGEwQDVqn", share: 100}],
          files: [{uri: "0.png", type: "image/png"}]
      },
      collection: {name: "numbers", family: "numbers"}
    };

    const metadataRequest = JSON.stringify(metadata);

    const res = await actions.updateMetadata({
      connection: new Connection(clusterApiUrl("devnet")),
      editionMint: mint_address,
      wallet: provider.wallet,
      new_metadata,
    });

    await sleep(10000);

    updated_metadata = await programs.metadata.Metadata.load(
      new Connection(clusterApiUrl("devnet")),
      metadata_address
    );

    console.log(`Metadata of nft after update: ${currentMetadata.data.data}`);
  });

});

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}