import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PropertyTokenization } from "../target/types/property_tokenization";
import {
  ExtensionType,
  TOKEN_2022_PROGRAM_ID,
  getMintLen,
  createInitializeMintInstruction,
  createInitializeTransferHookInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  createTransferCheckedWithTransferHookInstruction,
  getExtraAccountMetas,
  getMint,
  getAccount
} from "@solana/spl-token";

import {
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
} from "@solana/web3.js";
import { assert } from "chai";



describe("property_tokenization", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.propertyTokenization as Program<PropertyTokenization>;

  const wallet = provider.wallet as anchor.Wallet;
  
  const connection = provider.connection;
// system_id used in instruction
const systemId = new anchor.BN(1);

// -------------------------------------
// 1. property_system_acc (MAIN PDA)
// -------------------------------------
const [propertySystemPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("property_system"), // MUST match Rust constant
    systemId.toArrayLike(Buffer, "le", 8),
  ],
  program.programId
);

// -------------------------------------
// 3. treasury
// -------------------------------------
const [treasury] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("treasury"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

// -------------------------------------
// 4. reinvestment
// -------------------------------------
const [reinvestmentPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("reinvestment"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

// -------------------------------------
// 5. safety
// -------------------------------------
const [safetyPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("safety"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

// -------------------------------------
// 6. dividend
// -------------------------------------
const [dividendPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("dividend"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

// -------------------------------------
// 7. trustee_registry
// -------------------------------------
const [trusteeRegistryPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("trustee_registry"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

// -------------------------------------
// 8. arbitrator_registry
// -------------------------------------
const [arbitratorRegistryPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("arbitrator_registry"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);
// // const governanceMint = anchor.web3.Keypair.generate();



  it("create the property_token account and verfiy it",async() =>{
    
    const [pda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("property_system_account"),
    new anchor.BN(1).toArrayLike(Buffer, "le", 8),
  ],
  program.programId
);

console.log("Expected PDA:", pda.toBase58());
    try{const create_property_system = await program.methods.createPropertySystem(
                                            new anchor.BN(1),   // systemId (u64)
                                            1,                  // decimal (u8)
                                            new anchor.BN(1000),// numberOfTokens (u64)
                                            10,                  // safetyThreshold (u8)
                                            10,                  // trusteeSalaryThreshold (u8)
                                            10,                 // param6
                                            10,                 // param7
                                            60   
                                          )
                                          .accounts({
                                        creator: wallet.publicKey,
                                        tokenProgram:TOKEN_2022_PROGRAM_ID,
                                          })
                                            .rpc();}
    
    catch (e) {
  console.log(e.logs);
};


  
//property_system
const [propertySystemPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("property_system_account"),
    systemId.toArrayLike(Buffer, "le", 8),
  ],
  program.programId
);
const account = await program.account.propertySystemAccount.fetch(propertySystemPda);
console.log("PDA:", propertySystemPda.toBase58());
assert.equal(account.propertySystemId.toString(), "1");
console.log(account);


/////// governance mint

const [governanceMint] = anchor.web3.PublicKey.findProgramAddressSync(
  [
   Buffer.from("mint"),
  propertySystemPda.toBuffer(),
  ],
  program.programId
);

const [thresholdPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("threshold"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

 
  const mintData = await getMint(
    provider.connection,
    governanceMint,
    undefined,
    TOKEN_2022_PROGRAM_ID
  );

  console.log("✅ Mint exists");
  console.log("Decimals:", mintData.decimals);
  console.log("token supply :", mintData.supply);
  assert.equal(account.totalTokenSupply.toString(), mintData.supply.toString());

  ///threshold
const thresholdAcc = await program.account.threshold.fetch(thresholdPda);

assert.equal(thresholdAcc.safetyThreshold,10);
assert.equal(thresholdAcc.trusteeSalaryThreshold,10);
assert.equal(thresholdAcc.arbitratorSalaryThreshold,10);
assert.equal(thresholdAcc.dividendThreshold,10);
assert.equal(thresholdAcc.reinvestmentThreshold,60);

// //treasury_pda

const [treasury] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("treasury"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

const treasury_pda = await program.account.treasuryPda.fetch(treasury);

assert.equal(treasury_pda.propertySystemAccout.toBase58(),propertySystemPda.toBase58());


///reinvestment 
const [reinvestment] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("reinvestment"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

const reinvest = await program.account.reinvestmentPda.fetch(reinvestment);

assert.equal(reinvest.propertySystem.toBase58(),propertySystemPda.toBase58());

//safety
const [safety] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("safety"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

const safetypda = await program.account.safetyPda.fetch(safety);

assert.equal(safetypda.propertySystem.toBase58(),propertySystemPda.toBase58());

const [dividend] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("dividend"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

const dividendpda = await program.account.dividendPda.fetch(dividend);

assert.equal(dividendpda.propertySystem.toBase58(),propertySystemPda.toBase58());


const [trustee_registry] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("trustee_registry"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

const trustee_registry_pda = await program.account.trusteeRegistry.fetch(trustee_registry);

assert.equal(trustee_registry_pda.propertySystemAccount.toBase58(),propertySystemPda.toBase58());


const [arbitrator_registry] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("arbitrator_registry"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);

const arbitrator_registry_pda = await program.account.arbitratorRegistry.fetch(arbitrator_registry);

assert.equal(arbitrator_registry_pda.propertySystemAccount.toBase58(),propertySystemPda.toBase58());


const ataInfo =  getAssociatedTokenAddressSync(
  governanceMint,
  wallet.publicKey,
  false,
  TOKEN_2022_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
);

 const ataData = await getAccount(
    provider.connection,
    ataInfo,
    undefined,
    TOKEN_2022_PROGRAM_ID
  );

  assert.equal( ataData.owner.toBase58(),wallet.publicKey.toBase58());
  assert.equal( ataData.mint.toBase58(),governanceMint.toBase58());
  assert.equal( ataData.amount,BigInt(1000));
  // console.log("Owner:", ataData.owner.toBase58());
  // console.log("Mint:", ataData.mint.toBase58());
  // console.log("Balance:", ataData.amount.toString());


})







 

});
