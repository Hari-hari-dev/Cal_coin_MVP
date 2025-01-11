<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8"/>
  <title>Civic Pass Gating Demo</title>

  <!-- 1) SweetAlert for optional popups -->
  <script src="https://unpkg.com/sweetalert/dist/sweetalert.min.js"></script>

  <style>
    body {
      font-family: Arial, sans-serif;
      margin: 20px;
      text-align: center;
      background-color: #f8f8f8;
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
    }
    button:hover {
      background-color: #024ecf;
    }
    #status {
      margin-top: 20px;
      font-size: 16px;
      white-space: pre-wrap;
      background-color: #fff;
      padding: 15px;
      border-radius: 8px;
      box-shadow: 0 4px 6px rgba(0,0,0,0.1);
      max-width: 600px;
      margin: 20px auto;
    }
  </style>
</head>
<body>
  <h1>Civic Pass Gating Demo</h1>

  <!-- Buttons to connect Phantom + call checkgated -->
  <div>
    <button id="connectBtn">Connect Phantom</button>
    <button id="checkGatedBtn">Check Pass</button>
  </div>

  <div id="status">Not connected</div>

  <!-- Main Script -->
  <script type="module">
    /*******************************************************
     * 1) Import libs
     *******************************************************/
    import * as anchor from "https://esm.sh/@project-serum/anchor@0.26.0";
    import { PublicKey } from "https://esm.sh/@solana/web3.js@1.98.0";

    // Confirm loaded
    console.log("Anchor loaded:", typeof anchor !== "undefined");

    /*******************************************************
     * 2) Program + IDL info
     *******************************************************/
    // Put your actual program ID from declare_id! here:
    const PROGRAM_ID = new PublicKey("4DJBep6Jm34REZUnjr1NjEZiwqzm2pS1cjpiejvG2iUF");

    // Devnet cluster (or localnet / mainnet)
    const CLUSTER_URL = "https://api.devnet.solana.com";

    // Minimal IDL snippet for a `checkgated` instruction
    // that expects:
    //   fn checkgated(ctx: Context<CheckGated>) -> Result<()>
    // with accounts: [ user, pass, systemProgram ]
    const IDL = {
      version: "0.1.0",
      name: "your_program_name",
      instructions: [
        {
          name: "checkgated",
          accounts: [
            { name: "user", isMut: true, isSigner: true },
            { name: "pass", isMut: false, isSigner: false },
            { name: "systemProgram", isMut: false, isSigner: false }
          ],
          args: []
        }
      ]
    };

    /*******************************************************
     * 3) Global Vars
     *******************************************************/
    let provider;  // anchor provider
    let program;   // anchor program

    /*******************************************************
     * 4) Connect Phantom
     *******************************************************/
    async function onConnectPhantom() {
      if (!window.solana) {
        alert("Phantom not found. Please install or enable it!");
        return;
      }
      try {
        await window.solana.connect();
        const pubkey = window.solana.publicKey?.toBase58();
        document.getElementById("status").textContent =
          "Phantom connected: " + pubkey;
        console.log("Phantom connected =>", pubkey);
      } catch (err) {
        console.error("Connect error:", err);
        alert("Connect error: " + err.message);
      }
    }

    /*******************************************************
     * 5) Setup Anchor (Provider + Program)
     *******************************************************/
    function setupAnchor() {
      if (!anchor) {
        alert("Anchor library not loaded. Check references.");
        return;
      }
      const connection = new anchor.web3.Connection(CLUSTER_URL, "processed");

      // Minimal wallet interface from Phantom
      const wallet = {
        publicKey: window.solana.publicKey,
        signTransaction: (tx) => window.solana.signTransaction(tx),
        signAllTransactions: (txs) => window.solana.signAllTransactions(txs),
      };

      provider = new anchor.AnchorProvider(connection, wallet, {
        preflightCommitment: "processed"
      });
      program = new anchor.Program(IDL, PROGRAM_ID, provider);
    }

    /*******************************************************
     * 6) checkgated
     *******************************************************/
    async function onCheckGated() {
      if (!program) setupAnchor();
      if (!window.solana?.publicKey) {
        alert("Connect Phantom first!");
        return;
      }

      try {
        // Example pass address: 
        //  - In practice, you'd discover the user's actual Civic pass (gateway token).
        //  - This is just a placeholder and will likely fail if not a real pass.
        const passAddress = new PublicKey("6awvSWyeJ4Y5u3ZX7soDJ23jQVkg25qLHxqeQssmELMJ");

        // Attempt to call "checkgated"
        await program.methods.checkgated()
          .accounts({
            user: window.solana.publicKey,
            pass: passAddress,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();

        document.getElementById("status").textContent =
          "Civic pass check => SUCCESS! (The pass was valid for this user!)";
        console.log("Checkgated success!");
      } catch (err) {
        console.error("Checkgated failed:", err);
        document.getElementById("status").textContent =
          "Checkgated error => " + err.message;
      }
    }

    /*******************************************************
     * 7) Event Listeners
     *******************************************************/
    document.getElementById("connectBtn").onclick = onConnectPhantom;
    document.getElementById("checkGatedBtn").onclick = onCheckGated;

    window.addEventListener("load", () => {
      console.log("Page loaded");
    });
  </script>
</body>
</html>
