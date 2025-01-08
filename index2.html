<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Daily FaceScan Airdrop & Sign-In Demo</title>
  
  <!-- 1) SweetAlert for Alerts -->
  <script src="https://unpkg.com/sweetalert/dist/sweetalert.min.js"></script>
  
  <!-- 2) Web3Auth Sign-In with Solana (SIWS) -->
  <script src="https://unpkg.com/@web3auth/sign-in-with-solana@0.0.1/dist/index.umd.js"></script>
  
  <style>
    body {
      font-family: Arial, sans-serif;
      text-align: center;
      margin: 20px;
      background-color: #f0f2f5;
    }
    button {
      font-size: 16px;
      padding: 10px 20px;
      margin: 10px;
      cursor: pointer;
      border: none;
      border-radius: 5px;
      background-color: #0364ff;
      color: #fff;
      transition: background-color 0.3s;
    }
    button:hover {
      background-color: #024ecf;
    }
    #status, #siws-status {
      margin-top: 20px;
      font-size: 16px;
      white-space: pre-wrap;
      background-color: #fff;
      padding: 15px;
      border-radius: 8px;
      box-shadow: 0px 4px 6px rgba(0,0,0,0.1);
      max-width: 600px;
      margin-left: auto;
      margin-right: auto;
    }
    .hidden {
      display: none;
    }
  </style>
</head>

<body>
  <h1>Daily FaceScan Airdrop & Sign-In Demo</h1>

  <!-- Buttons: Connect + SIWS -->
  <div>
    <button id="connectWalletBtn">Connect Phantom Wallet</button>
    <button id="siwsBtn">Sign-In with Solana</button>
  </div>

  <!-- SIWS Panel -->
  <div id="siws-status" class="hidden">
    <h3>Sign-In Details</h3>
    <p><strong>Public Key:</strong> <span id="siws-publicKey"></span></p>
    <p><strong>Signature:</strong> <span id="siws-signature"></span></p>
    <button id="verifySiwsBtn">Verify Signature</button>
  </div>

  <!-- Airdrop Buttons -->
  <div>
    <button id="initBtn">Initialize Airdrop</button>
    <button id="claimBtn">Claim Airdrop</button>
    <button id="debugBtn">Debug Airdrop</button>
  </div>

  <!-- Status Panel -->
  <div id="status">Not connected</div>

  <!-- Main JS -->
  <script type="module">
    import * as anchor from "https://esm.sh/@project-serum/anchor@0.26.0";
    import { Connection, PublicKey } from "https://esm.sh/@solana/web3.js@1.98.0";

    // Debug
    console.log("Anchor loaded:", typeof anchor !== "undefined");
    console.log("SIWS loaded:", typeof window.SIWS !== "undefined");

    // Devnet or local
    const NETWORK_URL = "https://api.devnet.solana.com";
    // Program ID from your Rust
    const PROGRAM_ID = new PublicKey("8fJGcyaRCuQuTj12YLvLtQHiG9aQZiMqjG9iGQ9TBqyg");

    // The IDL snippet must match your Rust code's IDL fields
    const IDL = { /* same as your daily_facescan IDL JSON */ };

    let program, provider;
    let airdropKP, mintKP, mintAuthPda;
    let siwsMessage;

    async function connectWallet() {
      if (!window.solana) {
        alert("Phantom wallet not detected!");
        return;
      }
      try {
        await window.solana.connect();
        const pubkey = window.solana.publicKey.toBase58();
        document.getElementById("status").textContent = "Connected: " + pubkey;
      } catch (err) {
        alert("Connect error: " + err.message);
      }
    }

    function setupAnchor() {
      if (typeof anchor === "undefined") {
        alert("Anchor not loaded.");
        return;
      }
      const connection = new anchor.web3.Connection(NETWORK_URL, "processed");
      const wallet = {
        publicKey: window.solana.publicKey,
        signTransaction: (tx) => window.solana.signTransaction(tx),
        signAllTransactions: (txs) => window.solana.signAllTransactions(txs),
      };
      provider = new anchor.AnchorProvider(connection, wallet, {
        preflightCommitment: "processed",
      });
      program = new anchor.Program(IDL, PROGRAM_ID, provider);
    }

    async function onInitialize() {
      if (!program) setupAnchor();
      if (!window.solana.publicKey) {
        alert("Connect Phantom first!");
        return;
      }
      try {
        airdropKP = anchor.web3.Keypair.generate();
        mintKP = anchor.web3.Keypair.generate();

        // Derive mintAuthority PDA
        const [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
          [airdropKP.publicKey.toBytes(), new TextEncoder().encode("mint_authority")],
          PROGRAM_ID
        );
        mintAuthPda = pda;

        await program.methods.initialize()
          .accounts({
            airdrop: airdropKP.publicKey,
            mint: mintKP.publicKey,
            mintAuthority: mintAuthPda,
            authority: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          })
          .signers([airdropKP, mintKP])
          .rpc();

        document.getElementById("status").textContent = 
          "Init success!\nAirdropPubkey: " + airdropKP.publicKey.toBase58() + 
          "\nMintPubkey: " + mintKP.publicKey.toBase58();
      } catch (err) {
        alert("Init error: " + err.message);
      }
    }

    async function onClaim() {
      if (!program) setupAnchor();
      if (!airdropKP) {
        alert("Initialize first, no airdrop key?");
        return;
      }
      try {
        const [ticketPda, ticketBump] = await anchor.web3.PublicKey.findProgramAddress(
          [
            airdropKP.publicKey.toBytes(),
            provider.wallet.publicKey.toBytes(),
            new TextEncoder().encode("ticket")
          ],
          PROGRAM_ID
        );
        const recipientAta = await anchor.utils.token.associatedAddress({
          mint: mintKP.publicKey,
          owner: provider.wallet.publicKey
        });

        await program.methods.claim()
          .accounts({
            airdrop: airdropKP.publicKey,
            payer: provider.wallet.publicKey,
            mintAuthority: mintAuthPda,
            ticket: ticketPda,
            mint: mintKP.publicKey,
            recipientTokenAccount: recipientAta,
            recipient: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            associatedTokenProgram: anchor.utils.token.ASSOCIATED_TOKEN_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          })
          .rpc();

        document.getElementById("status").textContent = 
          "Claim successful! Check your token account.";
      } catch (err) {
        alert("Claim error: " + err.message);
      }
    }

    async function onDebug() {
      if (!program) setupAnchor();
      if (!airdropKP) {
        alert("Initialize first, no airdrop key?");
        return;
      }
      try {
        const state = await program.account.airdrop.fetch(airdropKP.publicKey);
        document.getElementById("status").textContent = 
          "Airdrop state:\n" + JSON.stringify(state, null, 2);
      } catch (err) {
        alert("Debug error: " + err.message);
      }
    }

    // SIWS
    async function signInWithSolana() {
      if (!window.solana || !window.solana.publicKey) {
        alert("Connect Phantom first!");
        return;
      }
      const header = new window.SIWS.Header();
      header.t = "sip99";

      const payload = new window.SIWS.Payload();
      payload.domain = window.location.host;
      payload.address = window.solana.publicKey.toBase58();
      payload.uri = window.location.origin;
      payload.statement = "Sign in with Solana to the app.";
      payload.version = "1";
      payload.chainId = "1";

      siwsMessage = new window.SIWS.SIWSMessage({ header, payload });
      const prepared = siwsMessage.prepareMessage();
      const encoded = new TextEncoder().encode(prepared);

      try {
        const sig = await window.solana.request({
          method: "signMessage",
          params: { message: encoded, display: "text" },
        });
        document.getElementById("siws-status").classList.remove("hidden");
        document.getElementById("siws-publicKey").textContent = sig.publicKey.toString();
        document.getElementById("siws-signature").textContent = sig.signature;
      } catch (err) {
        alert("Sign-In error: " + err.message);
      }
    }
    async function verifySignature() {
      if (!siwsMessage) {
        alert("No SIWS message to verify.");
        return;
      }
      const sigStr = document.getElementById("siws-signature").textContent;
      const pubk  = document.getElementById("siws-publicKey").textContent;

      const signature = { t: "sip99", s: sigStr };
      const payload   = siwsMessage.payload;
      payload.address = pubk;

      const resp = await siwsMessage.verify({ payload, signature });
      if (resp.success) {
        swal("Success", "Signature Verified", "success");
      } else {
        swal("Error", resp.error.type, "error");
      }
    }

    // Event Listeners
    document.getElementById("connectWalletBtn").onclick = connectWallet;
    document.getElementById("siwsBtn").onclick = signInWithSolana;
    document.getElementById("verifySiwsBtn").onclick = verifySignature;

    document.getElementById("initBtn").onclick = onInitialize;
    document.getElementById("claimBtn").onclick = onClaim;
    document.getElementById("debugBtn").onclick = onDebug;
  </script>
</body>
</html>
