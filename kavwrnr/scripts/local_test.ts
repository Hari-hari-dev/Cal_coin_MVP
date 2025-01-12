// scripts/local_test.ts
import * as anchor from "@project-serum/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import fs from "fs";


// Instead of ESM import for the JSON, use `require` to avoid import assertion issues:
const jsonString = fs.readFileSync("../target/idl/cal_coin.json", "utf-8");
const idl = JSON.parse(jsonString);
// The program ID from Anchor.toml / your Rust code
const programID = new PublicKey("11111111111111111111111111111111");

async function main() {
  // 1) Local provider
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  anchor.setProvider(provider);

  // 2) We assume your wallet is a Keypair from provider
  //    i.e. (provider.wallet as anchor.Wallet).payer is a Keypair
  const authorityKeypair = (provider.wallet as anchor.Wallet).payer;
  const authorityPubkey = authorityKeypair.publicKey;

  // 3) Load the program
  const program = new anchor.Program(idl, programID, provider);

  // 4) Derive a user PDA
  const [userPda] = await PublicKey.findProgramAddress(
    [Buffer.from("user"), authorityPubkey.toBuffer()],
    programID
  );

  // 5) Derive mint + authority PDAs (if your Rust uses these seeds)
  const [mintPda] = await PublicKey.findProgramAddress(
    [Buffer.from("my_mint")],
    programID
  );
  const [authPda] = await PublicKey.findProgramAddress(
    [Buffer.from("my_auth")],
    programID
  );

  // 6) Create user
  console.log("Creating user at", userPda.toBase58());
  const dummyNetwork = new PublicKey("11111111111111111111111111111112");
  await program.methods
    .createUser(dummyNetwork) 
    .accounts({
      user: userPda,
      authority: authorityPubkey,
      systemProgram: SystemProgram.programId
    })
    .rpc();
  console.log("User created successfully!");

  // 7) Derive ATA
  const callerAta = await anchor.utils.token.associatedAddress({
    mint: mintPda,
    owner: authorityPubkey
  });
  console.log("callerAta:", callerAta.toBase58());

  // 8) Call `claim`
  // If your Rust code references a `pass` param, either pass a dummy address or remove it from `.accounts(...)`.
  console.log("Claiming tokens...");
  await program.methods
    .claim()
    .accounts({
      user: userPda,
      // pass: SystemProgram.programId, // if gating is commented out
      caller: authorityPubkey,
      mint: mintPda,
      myAuth: authPda,
      callerAta: callerAta,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY
    })
    .rpc();

  console.log("Claim complete!");
}

main().catch((err) => {
  console.error("Error in local_test.ts:", err);
});
