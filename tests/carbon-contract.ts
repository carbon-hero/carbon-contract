import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CarbonEngine } from '../target/types/carbon_engine'

describe("carbon-engine", () => {
  // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    const wallet = provider.wallet as anchor.Wallet;
    anchor.setProvider(provider);
    const program = anchor.workspace.CarbonEngine as Program<CarbonEngine>;
  
    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );

    const testNftTitle = "Beta";
    const testNftSymbol = "BETA";
    const testNftUri = "https://raw.githubusercontent.com/Coding-and-Crypto/Rust-Solana-Tutorial/master/nfts/mint-nft/assets/example.json"
    
    it("Mint!", async () => {
      
        // Derive the mint address and the associated token
        const mintKeypair = anchor.web3.Keypair.generate();
        const tokenAddress = await anchor.utils.token.associatedAddress({
            mint: mintKeypair.publicKey,
            owner: wallet.publicKey,
        });
        console.log(`New token: ${mintKeypair.publicKey}`);

        const [metadataAddress] = (anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                TOKEN_METADATA_PROGRAM_ID.toBuffer(),
                mintKeypair.publicKey.toBuffer(),
            ],
            TOKEN_METADATA_PROGRAM_ID
        ));
        console.log(`metadata: ${metadataAddress}`);

        await program.methods.mint(
            testNftTitle, testNftSymbol, testNftUri
        )
        .accounts({
            metadata: metadataAddress,
            mint: mintKeypair.publicKey,
            tokenAccount: tokenAddress,
            mintAuthority: wallet.publicKey,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        })
        .signers([mintKeypair])
        .rpc();
    });
});
