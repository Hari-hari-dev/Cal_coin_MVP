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
    input {
      width: 80%;
      padding: 10px;
      margin: 10px 0;
      border-radius: 5px;
      border: 1px solid #ccc;
    }
    .hidden {
      display: none;
    }
  </style>
</head>
<body>
  <h1>Daily FaceScan Airdrop & Sign-In Demo</h1>
  
  <!-- Wallet Connection and SIWS -->
  <div>
    <button id="connectWalletBtn">Connect Phantom Wallet</button>
    <button id="siwsBtn">Sign-In with Solana</button>
  </div>

  <!-- Display SIWS Status -->
  <div id="siws-status" class="hidden">
    <h3>Sign-In Details</h3>
    <p><strong>Public Key:</strong> <span id="siws-publicKey"></span></p>
    <p><strong>Signature:</strong> <span id="siws-signature"></span></p>
    <button id="verifySiwsBtn">Verify Signature</button>
  </div>

  <!-- Airdrop Interaction -->
  <div>
    <button id="initBtn">Initialize Airdrop</button>
    <button id="claimBtn">Claim Airdrop</button>
    <button id="debugBtn">Debug Airdrop</button>
  </div>

  <!-- Display Airdrop Status -->
  <div id="status">Not connected</div>

  <!-- Main JavaScript Module -->
  <script type="module">
    /*********************************************************
     * 1) Import Libraries
     *********************************************************/
    import * as anchor from 'https://esm.sh/@project-serum/anchor@0.26.0';
    import {
      Connection,
      PublicKey,
      Keypair
    } from 'https://esm.sh/@solana/web3.js@1.98.0';

    // Check that Anchor and SIWS loaded
    console.log("Anchor loaded:", typeof anchor !== "undefined");
    console.log("SIWS loaded:", typeof window.SIWS !== "undefined");

    /*********************************************************
     * 2) Program + IDL Info
     *********************************************************/
    // 2a) Network (devnet or localhost)
    const NETWORK_URL = "https://api.devnet.solana.com";

    // 2b) Program ID from your `declare_id!`
    const PROGRAM_ID = new PublicKey("ADM5ikM5LS1ptrwFqXNyZDYazDzThSJknLNpJyw1x6c");

    // 2c) IDL matching your `lib.rs`
    const IDL = {
      "version": "0.1.0",
      "name": "daily_facescan",
      "instructions": [
        {
          "name": "initialize",
          "accounts": [
            {"name": "airdrop","isMut": true,"isSigner": true},
            {"name": "mint","isMut": true,"isSigner": true},
            {"name": "mintAuthority","isMut": false,"isSigner": false},
            {"name": "authority","isMut": true,"isSigner": true},
            {"name": "systemProgram","isMut": false,"isSigner": false},
            {"name": "tokenProgram","isMut": false,"isSigner": false},
            {"name": "rent","isMut": false,"isSigner": false}
          ],
          "args": []
        },
        {
          "name": "claim",
          "accounts": [
            {"name": "airdrop","isMut": false,"isSigner": false},
            {"name": "payer","isMut": true,"isSigner": true},
            {"name": "mintAuthority","isMut": false,"isSigner": false},
            {"name": "ticket","isMut": true,"isSigner": false},
            {"name": "mint","isMut": true,"isSigner": false},
            {"name": "recipientTokenAccount","isMut": true,"isSigner": false},
            {"name": "systemProgram","isMut": false,"isSigner": false},
            {"name": "tokenProgram","isMut": false,"isSigner": false},
            {"name": "associatedTokenProgram","isMut": false,"isSigner": false},
            {"name": "rent","isMut": false,"isSigner": false}
          ],
          "args": []
        }
      ],
      "accounts": [
        {
          "name": "Airdrop",
          "type": {
            "kind": "struct",
            "fields": [
              {"name": "gatekeeperNetwork","type": "publicKey"},
              {"name": "mint","type": "publicKey"},
              {"name": "dailyAmount","type": "u64"},
              {"name": "lastClaimTimestamp","type": "i64"},
              {"name": "owners","type": {"array": ["publicKey",6]}},
              {"name": "ownersCount","type": "u8"},
              {"name": "initialized","type": "bool"}
            ]
          }
        },
        {
          "name": "Ticket",
          "type": {"kind": "struct","fields": []}
        }
      ],
      "errors": [
        {"code":6000,"name":"InvalidPass","msg":"Invalid gating or pass check not satisfied"},
        {"code":6001,"name":"Unauthorized","msg":"Not an authorized owner"},
        {"code":6002,"name":"OwnersFull","msg":"Owners array is full"},
        {"code":6003,"name":"AlreadyOwner","msg":"Pubkey is already an owner"},
        {"code":6004,"name":"OwnerNotFound","msg":"Owner not found in the array"},
        {"code":6005,"name":"CannotRemoveSelf","msg":"Cannot remove yourself"},
        {"code":6006,"name":"InvalidPubkey","msg":"Could not parse gatekeeper network as valid"},
        {"code":6007,"name":"AlreadyInitialized","msg":"Already initialized"}
      ]
    };

    /*********************************************************
     * 3) Global Variables
     *********************************************************/
    let program;       // Anchor Program
    let provider;      // Anchor Provider
    let airdropKP;     // Keypair for the Airdrop account
    let mintKP;        // Keypair for the SPL Mint
    let mintAuthPda;   // PDA for the Mint Authority

    // SIWS variables
    let siwsMessage;   

    /*********************************************************
     * 4) Connect Phantom
     *********************************************************/
    async function connectWallet() {
      if (!window.solana) {
        alert("Phantom not found. Please install the wallet extension!");
        return;
      }
      try {
        const resp = await window.solana.connect();
        const userPubkey = resp.publicKey.toBase58();
        document.getElementById('status').textContent = "Connected to Phantom: " + userPubkey;
        console.log("Phantom connected as:", userPubkey);
      } catch (err) {
        console.error("Phantom connect error:", err);
        alert("Phantom connect error: " + err.message);
      }
    }

    /*********************************************************
     * 5) Setup Anchor
     *********************************************************/
    function setupAnchor() {
      if (!anchor) {
        alert("Anchor not loaded. Check script references.");
        return;
      }
      // Connect to devnet or localnet
      const connection = new anchor.web3.Connection(NETWORK_URL, "processed");

      // Construct a wallet interface from Phantom
      const wallet = {
        publicKey: window.solana.publicKey,
        signTransaction: (tx) => window.solana.signTransaction(tx),
        signAllTransactions: (txs) => window.solana.signAllTransactions(txs)
      };

      // AnchorProvider
      provider = new anchor.AnchorProvider(connection, wallet, {
        preflightCommitment: "processed"
      });

      // Program with IDL & ProgramID
      program = new anchor.Program(IDL, PROGRAM_ID, provider);
    }

    /*********************************************************
     * 6) Initialize Airdrop
     *********************************************************/
    async function onInitialize() {
      // Check Phantom
      if (!window.solana || !window.solana.publicKey) {
        alert("Please connect Phantom first!");
        return;
      }
      if (!program) {
        setupAnchor();
      }

      // Generate Keypairs
      airdropKP = anchor.web3.Keypair.generate();
      mintKP = anchor.web3.Keypair.generate();

      try {
        // Derive the Mint Authority PDA
        const [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
          [
            airdropKP.publicKey.toBytes(),
            new TextEncoder().encode("mint_authority")
          ],
          PROGRAM_ID
        );
        mintAuthPda = pda;

        // Call "initialize"
        await program.methods.initialize()
          .accounts({
            airdrop: airdropKP.publicKey,
            mint: mintKP.publicKey,
            mintAuthority: mintAuthPda,
            authority: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY
          })
          .signers([airdropKP, mintKP])
          .rpc();

        document.getElementById("status").textContent =
          "Initialize success!\nAirdropPubkey: " + airdropKP.publicKey.toBase58() +
          "\nMintPubkey: " + mintKP.publicKey.toBase58();

        console.log("Initialized with airdrop:", airdropKP.publicKey.toBase58());
      } catch (err) {
        console.error("Initialize failed:", err);
        alert("Initialize error: " + err.message);
      }
    }

    /*********************************************************
     * 7) Claim Airdrop
     *********************************************************/
    async function onClaim() {
      if (!program) {
        setupAnchor();
      }
      if (!airdropKP) {
        alert("No known airdropPubkey. Did you call Initialize first?");
        return;
      }
      try {
        // Derive the "ticket" PDA
        const [ticketPda] = await anchor.web3.PublicKey.findProgramAddress(
          [
            airdropKP.publicKey.toBytes(),
            provider.wallet.publicKey.toBytes(),
            new TextEncoder().encode("ticket")
          ],
          PROGRAM_ID
        );

        // Associated token account for the minted SPL
        const recipientAta = await anchor.utils.token.associatedAddress({
          mint: mintKP.publicKey,
          owner: provider.wallet.publicKey
        });

        // Call "claim"
        await program.methods.claim()
          .accounts({
            airdrop: airdropKP.publicKey,
            payer: provider.wallet.publicKey,
            mintAuthority: mintAuthPda,
            ticket: ticketPda,
            mint: mintKP.publicKey,
            recipientTokenAccount: recipientAta,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY
          })
          .rpc();

        document.getElementById("status").textContent =
          "Claim successful! Check your token account on devnet explorers.";
        console.log("Claim successful!");
      } catch (err) {
        console.error("Claim failed:", err);
        alert("Claim error: " + err.message);
      }
    }

    /*********************************************************
     * 8) Debug: Fetch Airdrop State
     *********************************************************/
    async function onDebug() {
      if (!program) {
        setupAnchor();
      }
      if (!airdropKP) {
        alert("No known airdropPubkey. Did you call Initialize first?");
        return;
      }
      try {
        const airdropState = await program.account.airdrop.fetch(airdropKP.publicKey);
        console.log("Airdrop State:", airdropState);
        document.getElementById("status").textContent =
          "Airdrop state =>\n" + JSON.stringify(airdropState, null, 2);
      } catch (err) {
        console.error("Debug error:", err);
        alert("Debug fetch error: " + err.message);
      }
    }

    /*********************************************************
     * 9) Sign-In with Solana (SIWS)
     *********************************************************/
    async function signInWithSolana() {
      if (!window.solana || !window.solana.publicKey) {
        alert("Connect Phantom first!");
        return;
      }

      // Prepare an SIWS message
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
      const preparedMessage = siwsMessage.prepareMessage();
      const encodedMessage = new TextEncoder().encode(preparedMessage);

      try {
        // Request signature from Phantom
        const signedMessage = await window.solana.request({
          method: "signMessage",
          params: {
            message: encodedMessage,
            display: "text",
          },
        });

        // Show details
        document.getElementById("siws-status").classList.remove("hidden");
        document.getElementById("siws-publicKey").textContent = signedMessage.publicKey.toString();
        document.getElementById("siws-signature").textContent = signedMessage.signature;
        
        console.log("Signed Message =>", signedMessage);
      } catch (err) {
        console.error("SIWS sign error:", err);
        alert("SIWS sign error: " + err.message);
      }
    }

    /*********************************************************
     * 10) Verify SIWS Signature
     *********************************************************/
    async function verifySignature() {
      if (!siwsMessage) {
        alert("No SIWS message found. Sign In first!");
        return;
      }
      const sigStr = document.getElementById("siws-signature").textContent;
      const pubStr = document.getElementById("siws-publicKey").textContent;

      const signature = { t: "sip99", s: sigStr };
      const payload = siwsMessage.payload;
      payload.address = pubStr;

      const resp = await siwsMessage.verify({ payload, signature });
      if (resp.success) {
        swal("Success", "Signature Verified", "success");
      } else {
        swal("Error", resp.error.type, "error");
      }
    }

    /*********************************************************
     * 11) Event Listeners
     *********************************************************/
    window.addEventListener("load", () => {
      console.log("Page loaded");
    });

    document.getElementById("connectWalletBtn").onclick = connectWallet;
    document.getElementById("initBtn").onclick = onInitialize;
    document.getElementById("claimBtn").onclick = onClaim;
    document.getElementById("debugBtn").onclick = onDebug;

    document.getElementById("siwsBtn").onclick = signInWithSolana;
    document.getElementById("verifySiwsBtn").onclick = verifySignature;

  </script>
</body>
</html>
