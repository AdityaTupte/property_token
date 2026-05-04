import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LiteSVM } from "litesvm";
import { PropertyTokenization } from "../target/types/property_tokenization";
import {
  TOKEN_2022_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddressSync,
  getMint,
  getAccount,
  getAssociatedTokenAddress,
  createTransferInstruction,
  createMint,
  createAssociatedTokenAccount,
  createMintToInstruction,
  mintTo
} from "@solana/spl-token";

import {
  PublicKey,
  Transaction,
  Keypair,
} from "@solana/web3.js";
import { getTransactionDecoder } from "@solana/transactions";
import { keccak_256 } from "@noble/hashes/sha3";
import { assert } from "chai";
import fs from "fs";
import os from "os";
import path from "path";
import { log } from "console";

const idl = require("../target/idl/property_tokenization.json");

class LiteSvmConnection {
  commitment = "processed";
  private readonly svm: LiteSVM;

  constructor(svm: LiteSVM) {
    this.svm = svm;
  }

  private currentSlot(): number {
    return Number(this.svm.getClock().slot);
  }

  private toAccountInfo(encodedAccount: any) {
    if (!encodedAccount?.exists) {
      return null;
    }

    return {
      data: Buffer.from(encodedAccount.data),
      executable: encodedAccount.executable,
      lamports: Number(encodedAccount.lamports),
      owner: new PublicKey(encodedAccount.programAddress),
      rentEpoch: 0,
    };
  }

  private decodeAndSend(rawTx: Buffer | Uint8Array) {
    const result = this.svm.sendTransaction(getTransactionDecoder().decode(rawTx));

    if (typeof (result as any).err === "function") {
      const error: any = new Error(`LiteSVM transaction failed: ${(result as any).toString()}`);
      error.logs = (result as any).meta().logs();
      throw error;
    }

    return result as any;
  }

  async getLatestBlockhash() {
    return {
      blockhash: this.svm.latestBlockhash().toString(),
      lastValidBlockHeight: 0,
    };
  }

  async sendRawTransaction(rawTx: Buffer | Uint8Array) {
    const result = this.decodeAndSend(rawTx);
    return anchor.utils.bytes.bs58.encode(result.signature());
  }

  async sendTransaction(tx: Transaction, signers: Keypair[], options?: any) {
    tx.feePayer = tx.feePayer || signers[0]?.publicKey;
    tx.recentBlockhash = (await this.getLatestBlockhash()).blockhash;
    signers.forEach((signer) => tx.partialSign(signer));
    return this.sendRawTransaction(tx.serialize());
  }

  async confirmTransaction(_strategy?: any, _commitment?: any) {
    return {
      context: { slot: this.currentSlot() },
      value: { err: null },
    };
  }

  async getTransaction() {
    return null;
  }

  async getAccountInfo(address: PublicKey) {
    return this.toAccountInfo(this.svm.getAccount(address.toBase58() as any));
  }

  async getAccountInfoAndContext(address: PublicKey) {
    return {
      context: { slot: this.currentSlot() },
      value: await this.getAccountInfo(address),
    };
  }

  async getMultipleAccountsInfo(addresses: PublicKey[]) {
    return Promise.all(addresses.map((address) => this.getAccountInfo(address)));
  }

  async getBalance(address: PublicKey) {
    return Number(this.svm.getBalance(address.toBase58() as any) ?? 0n);
  }

  async getMinimumBalanceForRentExemption(dataLen: number) {
    return Number(this.svm.minimumBalanceForRentExemption(BigInt(dataLen)));
  }

  async requestAirdrop(address: PublicKey, lamports: number) {
    this.svm.airdrop(address.toBase58() as any, BigInt(lamports) as any);
    return Keypair.generate().publicKey.toBase58();
  }

}

class LiteSvmWallet {
  readonly payer: Keypair;
  readonly publicKey: PublicKey;

  constructor() {
    const walletPath = path.join(os.homedir(), ".config/solana/id.json");
    const secretKey = Uint8Array.from(JSON.parse(fs.readFileSync(walletPath, "utf8")));
    this.payer = Keypair.fromSecretKey(secretKey);
    this.publicKey = this.payer.publicKey;
  }

  async signTransaction<T extends Transaction>(tx: T): Promise<T> {
    tx.partialSign(this.payer);
    return tx;
  }

  async signAllTransactions<T extends Transaction[]>(txs: T): Promise<T> {
    txs.forEach((tx) => tx.partialSign(this.payer));
    return txs;
  }
}

class LiteSvmProvider {
  readonly publicKey: PublicKey;
  readonly connection: LiteSvmConnection;
  readonly wallet: LiteSvmWallet;
  readonly opts: any;

  constructor(connection: LiteSvmConnection, wallet: LiteSvmWallet, opts = anchor.AnchorProvider.defaultOptions()) {
    this.connection = connection;
    this.wallet = wallet;
    this.opts = opts;
    this.publicKey = wallet.publicKey;
  }

  async sendAndConfirm(tx: Transaction, signers: Keypair[] = []) {
    tx.feePayer = tx.feePayer ?? this.wallet.publicKey;
    tx.recentBlockhash = (await this.connection.getLatestBlockhash()).blockhash;

    signers.forEach((signer) => tx.partialSign(signer));
    await this.wallet.signTransaction(tx);

    return this.connection.sendRawTransaction(tx.serialize());
  }
}

function advanceClockBy(svm: LiteSVM, seconds: bigint) {
  const clock = svm.getClock();
  clock.unixTimestamp = clock.unixTimestamp + seconds;
  clock.slot = clock.slot + seconds;
  svm.setClock(clock);
}



function toU64LeBuffer(value: number | bigint | anchor.BN): Buffer {
  if (value instanceof anchor.BN) {
    return value.toArrayLike(Buffer, "le", 8);
  }

  const buffer = Buffer.alloc(8);
  buffer.writeBigUInt64LE(BigInt(value));
  return buffer;
}

function keccakBuffer(parts: ReadonlyArray<Buffer | Uint8Array>): Buffer {
  return Buffer.from(keccak_256(Buffer.concat(parts.map((part) => Buffer.from(part)))));
}

function buildMerkleRoot(leaves: ReadonlyArray<Buffer | Uint8Array>): number[] {
  if (leaves.length === 0) {
    throw new Error("Cannot build a Merkle root from zero leaves");
  }

  let level: Uint8Array[] = leaves.map((leaf) => Uint8Array.from(leaf));

  while (level.length > 1) {
    if (level.length % 2 === 1) {
      level.push(level[level.length - 1]);
    }

    const nextLevel: Uint8Array[] = [];

    for (let i = 0; i < level.length; i += 2) {
      const left = Buffer.from(level[i]);
      const right = Buffer.from(level[i + 1]);
      const [first, second] = Buffer.compare(left, right) <= 0 ? [left, right] : [right, left];

      nextLevel.push(keccakBuffer([first, second]));
    }

    level = nextLevel;
  }

  return Array.from(level[0]);
}

function buildMerkleProof(
  leaves: ReadonlyArray<Buffer | Uint8Array>,
  index: number
): number[][] {
  if (leaves.length === 0) {
    throw new Error("Cannot build a Merkle proof from zero leaves");
  }

  if (index < 0 || index >= leaves.length) {
    throw new Error(`Leaf index ${index} is out of range`);
  }

  const proof: number[][] = [];
  let currentIndex = index;
  let level: Uint8Array[] = leaves.map((leaf) => Uint8Array.from(leaf));

  while (level.length > 1) {
    if (level.length % 2 === 1) {
      level.push(level[level.length - 1]);
    }

    const siblingIndex = currentIndex % 2 === 0 ? currentIndex + 1 : currentIndex - 1;
    proof.push(Array.from(level[siblingIndex]));

    const nextLevel: Uint8Array[] = [];

    for (let i = 0; i < level.length; i += 2) {
      const left = Buffer.from(level[i]);
      const right = Buffer.from(level[i + 1]);
      const [first, second] = Buffer.compare(left, right) <= 0 ? [left, right] : [right, left];

      nextLevel.push(keccakBuffer([first, second]));
    }

    level = nextLevel;
    currentIndex = Math.floor(currentIndex / 2);
  }

  return proof;
}

function buildSellProposalLeaf(
  voter: PublicKey,
  proposal: PublicKey,
  governanceMint: PublicKey,
  votingPower: number | bigint | anchor.BN
): Buffer {
  return keccakBuffer([
    Buffer.from("SELLPROPERTY"),
    voter.toBuffer(),
    proposal.toBuffer(),
    governanceMint.toBuffer(),
    toU64LeBuffer(votingPower),
  ]);
}

function buildAuthorityLeaf(
  voter: PublicKey,
  proposal: PublicKey,
  governanceMint: PublicKey,
  votingPower: number | bigint | anchor.BN,
  authorityType: number ,   // 0 for TRUSTEE, 1 for ARBITRATOR
): Buffer {
  return keccakBuffer([
    Buffer.from([authorityType]),
    voter.toBuffer(),
    proposal.toBuffer(),
    governanceMint.toBuffer(),
    toU64LeBuffer(votingPower),
  ]);
}

// function buildTrusteeVoteLeaf(
//   voter: PublicKey,
//   proposal: PublicKey,
//   governanceMint: PublicKey,
//   votingPower: number | bigint | anchor.BN
// ): Buffer {
//   return keccakBuffer([
//     Buffer.from("TRUSTEE"),
//     voter.toBuffer(),
//     proposal.toBuffer(),
//     governanceMint.toBuffer(),
//     toU64LeBuffer(votingPower),
//   ]);
// }



function buildBuyProposalLeaf(
  voter: PublicKey,
  proposal: PublicKey,
  governanceMint: PublicKey,
  votingPower: number | bigint | anchor.BN
): Buffer {
  return keccakBuffer([
    Buffer.from("BUYPROPERTY"),
    voter.toBuffer(),
    proposal.toBuffer(),
    governanceMint.toBuffer(),
    toU64LeBuffer(votingPower),
  ]);
}

// function buildTrusteeVoteProof(
//   entries: ReadonlyArray<{
//     voter: PublicKey;
//     votingPower: number | bigint | anchor.BN;
//     authoritytype:number,
//   }>,
//   targetIndex: number,
//   proposal: PublicKey,
//   governanceMint: PublicKey
// ): number[][] {
//   const leaves = entries.map((entry) =>
//     buildAuthorityLeaf(entry.voter, proposal, governanceMint, entry.votingPower,entry.authoritytype)
//   );

//   return buildMerkleProof(leaves, targetIndex);
// }

function buildSellProposalProof(
  entries: ReadonlyArray<{
    voter: PublicKey;
    votingPower: number | bigint | anchor.BN;
  }>,
  targetIndex: number,
  proposal: PublicKey,
  governanceMint: PublicKey
): number[][] {
  const leaves = entries.map((entry) =>
    buildSellProposalLeaf(entry.voter, proposal, governanceMint, entry.votingPower)
  );

  return buildMerkleProof(leaves, targetIndex);
}

function buildAuthorityProof(
  entries: ReadonlyArray<{
    voter: PublicKey;
    votingPower: number | bigint | anchor.BN;
    authoritytype:number,
  }>,
  targetIndex: number,
  proposal: PublicKey,
  governanceMint: PublicKey
): number[][] {
  const leaves = entries.map((entry) =>
    buildAuthorityLeaf(entry.voter, proposal, governanceMint, entry.votingPower,entry.authoritytype)
  );

  return buildMerkleProof(leaves, targetIndex);
}

function buildBuyProposalProof(
  entries: ReadonlyArray<{
    voter: PublicKey;
    votingPower: number | bigint | anchor.BN;
  }>,
  targetIndex: number,
  proposal: PublicKey,
  governanceMint: PublicKey
): number[][] {
  const leaves = entries.map((entry) =>
    buildBuyProposalLeaf(entry.voter, proposal, governanceMint, entry.votingPower)
  );

  return buildMerkleProof(leaves, targetIndex);
}




describe("property_tokenization", () => {
  const svm = new LiteSVM();
  const programId = new PublicKey(idl.address);
  svm.addProgramFromFile(
    programId.toBase58() as any,
    path.resolve(__dirname, "../target/deploy/property_tokenization.so")
  );

  const provider = new LiteSvmProvider(new LiteSvmConnection(svm), new LiteSvmWallet());
  svm.airdrop(provider.wallet.publicKey.toBase58() as any, 100_000_000_000n as any);
  
  anchor.setProvider(provider as any);
  const program = new Program(idl as PropertyTokenization, provider as any) as Program<PropertyTokenization>;

  const wallet = provider.wallet as anchor.Wallet;
  
  const connection = provider.connection as any;

    const pro1 = Keypair.generate();
    const pro2 = Keypair.generate();
    const pro3 = Keypair.generate();
    const pro4 = Keypair.generate();
    const pro5 = Keypair.generate();
    const pro6 = Keypair.generate();

    let pro_vec = [
  pro1,
  pro2,
  pro3,
  pro4,
  pro5,
  pro6,

    ];


    const prosys1 = Keypair.generate();
    const prosys2 = Keypair.generate();
    const prosys3 = Keypair.generate();
    const prosys4 = Keypair.generate();
    const prosys5 = Keypair.generate();
    const prosys6 = Keypair.generate();

    let pro_vec2 = [
  prosys1,
  prosys2,
  prosys3,
  prosys4,
  prosys5,
  prosys6,

    ];



// system_id used in instruction
const systemId = new anchor.BN(1);

  const auth1 = Keypair.generate();
    const auth2 = Keypair.generate();
    const auth3 = Keypair.generate();
    const auth4 = Keypair.generate();
    const auth5 = Keypair.generate();
    const auth6 = Keypair.generate();
    const auth7 = Keypair.generate();
    const auth8 = Keypair.generate();
    const auth9 = Keypair.generate();
    const auth10 = Keypair.generate();

    let vec = [
  auth1.publicKey,
  auth2.publicKey,
  auth3.publicKey,
  auth4.publicKey,
  auth5.publicKey,
  auth6.publicKey,
  auth7.publicKey,
  auth8.publicKey,
  auth9.publicKey,
  auth10.publicKey,
   
];

function toFixed32(str: string) {
  const buf = Buffer.alloc(32); // fills with 0s
  buf.write(str);
  return buf;
}

  let vec1 = [
  auth1,
  auth2,
  auth3,
  auth4,
  auth5,
  auth6,
  auth7,
  auth8,
  auth9,
  auth10,
   
];

 const country_auth1 = Keypair.generate();
    const country_auth2 = Keypair.generate();
    const country_auth3 = Keypair.generate();
    const country_auth4 = Keypair.generate();
    const country_auth5 = Keypair.generate();
    const country_auth6 = Keypair.generate();
    const country_auth7 = Keypair.generate();
    const country_auth8 = Keypair.generate();
    const country_auth9 = Keypair.generate();
    const country_auth10 = Keypair.generate();

   let country_auth_vec =[
      country_auth1,
      country_auth2,
       country_auth3,
        country_auth4,
         country_auth5,
         country_auth6,
      country_auth7,
       country_auth8,
        country_auth9,
         country_auth10,
   ] 
   const [propertySystemPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("property_system_account"),
    systemId.toArrayLike(Buffer, "le", 8),
  ],
  program.programId
);


  it("create the property_token account and verfiy it",async() =>{
    
//     const [pda] = PublicKey.findProgramAddressSync(
//   [
//     Buffer.from("property_system_account"),
//     new anchor.BN(1).toArrayLike(Buffer, "le", 8),
//   ],
//   program.programId
// );

// console.log("Expected PDA:", pda.toBase58());
    try{const create_property_system = await program.methods.createPropertySystem(
                                            new anchor.BN(1),   
                                            1,                  
                                            new anchor.BN(1000),
                                            10,                 
                                            10,                  
                                            10,                 
                                            10,                 
                                            60,
                                            5,
                                            3,
                                            5,
                                            2   
                                          )
                                          .accounts({
                                        creator: wallet.publicKey,
                                        tokenProgram:TOKEN_2022_PROGRAM_ID,
                                          })
                                            .rpc();}
    
    catch (e) {
  console.log(e.logs);
};



// for second property 

  try{const create_property_system = await program.methods.createPropertySystem(
                                            new anchor.BN(2),   
                                            1,                  
                                            new anchor.BN(1000),
                                            10,                 
                                            10,                  
                                            10,                 
                                            10,                 
                                            60,
                                            5,
                                            3,
                                            4,
                                            2   
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

const account = await program.account.propertySystemAccount.fetch(propertySystemPda);
// console.log("PDA:", propertySystemPda.toBase58());
assert.equal(account.propertySystemId.toString(), "1");
// console.log(account);


// /////// governance mint

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
    provider.connection as any,
    governanceMint,
    undefined,
    TOKEN_2022_PROGRAM_ID
  );

  // console.log("✅ Mint exists");
  // console.log("Decimals:", mintData.decimals);
  // console.log("token supply :", mintData.supply);
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
    provider.connection as any,
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


it("add trustee authority",async()=>{


  // const account = await program.account.propertySystemAccount.fetch(propertySystemPda);
  // console.log(account);

for(let i = 0; i < 6; i++){await connection.requestAirdrop(pro_vec[i].publicKey, 1e9); // 1 SOL

await new Promise(resolve => setTimeout(resolve, 100));}

for(let i = 0; i < 5; i++){

   try {
    const tx = await program.methods.addTrustee(
    new anchor.BN(1),
  ).accounts(
    {authority:wallet.publicKey,
    newTrustee: pro_vec[i].publicKey,
    }
  ).signers([wallet.payer, pro_vec[i]]).rpc();
   } catch (error) {
    console.log(error);
    
   }

}



// for second
for(let i = 0; i < 6; i++){await connection.requestAirdrop(pro_vec2[i].publicKey, 1e9); // 1 SOL

await new Promise(resolve => setTimeout(resolve, 100));}

for(let i = 0; i < 5; i++){

   try {
    const tx = await program.methods.addTrustee(
    new anchor.BN(2),
  ).accounts(
    {authority:wallet.publicKey,
    newTrustee: pro_vec2[i].publicKey,
    }
  ).signers([wallet.payer, pro_vec2[i]]).rpc();
   } catch (error) {
    console.log(error);
    
   }


}

const [trustee_registry] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("trustee_registry"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);
const trustee_registry_pda = await program.account.trusteeRegistry.fetch(trustee_registry);

// console.log(trustee_registry_pda);
})



it("add arbitrator authority",async()=>{

for(let i = 0; i <6; i++){

   try {
    const tx = await program.methods.addArbitrator(
    new anchor.BN(1),
  ).accounts(
    {authority:wallet.publicKey,
    newArbitrator: pro_vec[i].publicKey,
    }
  ).signers([wallet.payer, pro_vec[i]]).rpc();
   } catch (error) {
    console.log(error);
    
   }


}

//second

for(let i = 0; i <5; i++){

   try {
    const tx = await program.methods.addArbitrator(
    new anchor.BN(2),
  ).accounts(
    {authority:wallet.publicKey,
    newArbitrator: pro_vec2[i].publicKey,
    }
  ).signers([wallet.payer, pro_vec2[i]]).rpc();
   } catch (error) {
    console.log(error);
    
   }


}




const [arbitrator_registry] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("arbitrator_registry"),
    propertySystemPda.toBuffer(),
  ],
  program.programId
);
const arbitrator_registry_pda = await program.account.arbitratorRegistry.fetch(arbitrator_registry);

// console.log(arbitrator_registry_pda);

 const account = await program.account.propertySystemAccount.fetch(propertySystemPda);
  // console.log(account);
})




              //COUNTRYCREATIONAUTHORIY


it("Create 10 pubkeyt for authority belongs to approve country", async()=>{

    const new_tx = await program.methods.createApproveCountryAuthority(
          4,
          vec
    ).accounts(
      wallet.payer ,
    ).rpc();

//   const [approve_country_autority] = anchor.web3.PublicKey.findProgramAddressSync(
//   [
//     Buffer.from("AuthorityForApprovingCountry"),
//   ],
//   program.programId
// );
// const account = await program.account.approveCountryAuthority.fetch(approve_country_autority);

// console.log(account.authority);
// console.log(account.threshold.toString());




})


              //  COUNTRYCREATION
  const countryName = toFixed32("INDIA");
              const cou = [...countryName];         
it("create a country_proposal",async()=>{

  
  
  const [countryAccPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("country_proposal"), countryName],
    program.programId
  );

  await program.methods.createCountryProposal(
    cou,
    1,5,3
  ).accounts({
    signer: wallet.publicKey,
    
  }).signers([wallet.payer]).rpc()

 const [country_proposal] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("country_proposal"),
    toFixed32("INDIA"),
  ],
  program.programId
);

// console.log("pubkey",country_proposal)
 
const countryp = await program.account.proposalCountryPda.fetch(country_proposal);

console.log(countryp);

  //  const create_proposal2 = await program.methods.createCountryProposal(
  //   2,
  //   "INDIa",
  //   vec2,
  //   3
  // ).accounts(
  //   wallet.payer
  // ).rpc()
})


it("approve country_proposal",async()=>{

for(let i = 0; i < 9; i++){await connection.requestAirdrop(vec1[i].publicKey, 1e9); // 1 SOL

await new Promise(resolve => setTimeout(resolve, 100));}

 const [country_proposal] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("country_proposal"),
    countryName,
  ],
  program.programId
);

try {
  for(let i = 0; i < 4; i++){
  const tx = await program.methods.approveCountry(
    cou
  )
.accounts({          
  signer: vec1[i].publicKey,          
}).signers(
  [vec1[i]]
).rpc()
}
} catch (error) {
  console.log(error);
}

const countryp = await program.account.proposalCountryPda.fetch(country_proposal);

console.log(countryp);

})


 it("exceute_country_proposal",async()=>{

//    const [country_proposal] = anchor.web3.PublicKey.findProgramAddressSync(
//    [
//      Buffer.from("country_proposal"),
//      countryName,
//    ],
//    program.programId
//  );

 await connection.requestAirdrop(auth1.publicKey, 1e9); // 1 SOL

 await new Promise(resolve => setTimeout(resolve, 1000));

   try {
     const exceute = await program.methods.executeCountryPropsal(
      cou,
     ).accounts(
      {
       signer:auth1.publicKey
       }
   ).signers([auth1]).rpc();


   } catch (error) {
     console.log(error)
   }
  
   const [country] = anchor.web3.PublicKey.findProgramAddressSync(
   [
     Buffer.from("country"),
     countryName,
   ],
   program.programId
 );

   const acc = await program.account.country.fetch(country);

   console.log(acc);

 })

it("add country authorities",async()=>{


try {
  for(let i =0;i<5;i++){

  const tx = await program.methods.addCountryAuthority(
    cou,
  ).accounts(
    {
      signer:auth1.publicKey,
      countryAuthority:country_auth_vec[i].publicKey,
    }
  ).signers([auth1]).rpc();

 

}

} catch (error) {
console.log(error);  
}

 const [country] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("country"),
    countryName,
  ],
  program.programId
);

  const acc = await program.account.country.fetch(country);

  console.log(acc);

})

//             STATECREATION

const stateName = toFixed32("MAHARASHTRA");
              const sta = [...stateName]; 

const [country] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("country"),
    countryName
  ],
  program.programId
);

// const countryName2 = toFixed32("USA");

// const [country2] = anchor.web3.PublicKey.findProgramAddressSync(
//   [
//     Buffer.from("country"),
//     countryName2
//   ],
//   program.programId
// );

it("create a state_proposal",async()=>{

for(let i = 0; i < 9; i++){await connection.requestAirdrop(country_auth_vec[i].publicKey, 1e9); // 1 SOL

await new Promise(resolve => setTimeout(resolve, 100));}



 try {
   const state_proposal = await program.methods.stateCreationProposal(
    sta,
    cou,
    1,5,3
   ).accounts(
    {
      signer:country_auth1.publicKey
    }
   ).signers([country_auth1]).rpc()


 

 } catch (error) {
console.log(error);  
 }

 const [state_proposal_account] =  anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("state_proposal"),
            stateName,
            country.toBuffer(),
          ],
          program.programId
        );

  const proposal = await program.account.stateProposalPda.fetch(state_proposal_account);

  console.log(proposal);
  

})
 
it("approve state",async() => {


   

try {
   for(let i= 0;i<3;i++){

    const [approve] = await program.methods.stateProposalApproval(
    sta,
    cou
  ).accounts({
    signer:(country_auth_vec[i]).publicKey,
  
  }).signers([country_auth_vec[i]]).rpc()
  }
} catch (error) {
  console.log(error);
  
}

  
const [state_proposal_account] =  anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("state_proposal"),
            stateName,
            country.toBytes(),
          ],
          program.programId
        );

  const proposal = await program.account.stateProposalPda.fetch(state_proposal_account);

  console.log(proposal);
  
})


it("execute state proposal",async()=>{

  const exceute = await program.methods.stateProposalExecute(
    sta,
    cou
  ).accounts(
    wallet.payer
  ).rpc();

  const [state_pda_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("state"),
      stateName,
      country.toBuffer(),
    ],
    program.programId,
  );
  console.log("STATE_ACCOUNT PDA",state_pda_key);


  const state_account = await program.account.state.fetch(state_pda_key);

  console.log(state_account);

})


const state_auth1 = Keypair.generate();
const state_auth2= Keypair.generate();
const state_auth3 = Keypair.generate();
const state_auth4 = Keypair.generate();
const state_auth5 = Keypair.generate();
const state_auth6 = Keypair.generate();

const state_auth = [
  state_auth1,
  state_auth2,
  state_auth3,
  state_auth4,
  state_auth5,
  state_auth6,

]
const [state_pda_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("state"),
      stateName,
      country.toBuffer(),
    ],
    program.programId,
  );


it("add state authority",async()=>{


try {
  for(let i =0;i<5;i++){

  const tx = await program.methods.addStateAuhtority(
    cou,
    sta,
  ).accounts(
    {
      signer:country_auth1.publicKey,
      stateAuthority:state_auth[i].publicKey,
    }
  ).signers([country_auth1]).rpc();

}

} catch (error) {
console.log(error);  
}


  console.log("STATE_ACCOUNT PDA",state_pda_key);


  const state_account = await program.account.state.fetch(state_pda_key);

  console.log(state_account);


})

let legal_doc_hash =  [...toFixed32("abc")];

it("create property_proposal",async()=>{

  for(let i = 0; i < 5; i++){await connection.requestAirdrop(state_auth[i].publicKey, 1e9); // 1 SOL

  await new Promise(resolve => setTimeout(resolve, 100));}

  const property_proposal = await program.methods.createPropertyProposal(
    country,
    sta,
    new anchor.BN(1),
    new anchor.BN(1),
    legal_doc_hash,
  ).accounts(
    {
      signer:state_auth1.publicKey,
      
    }
  ).signers([state_auth1]).rpc();



  ///second

  const property_proposal2 = await program.methods.createPropertyProposal(
    country,
    sta,
    new anchor.BN(2),
    new anchor.BN(2),
    legal_doc_hash,
  ).accounts(
    {
      signer:state_auth1.publicKey,
      
    }
  ).signers([state_auth1]).rpc();

// const propertyId = new anchor.BN(1);

//   const [property_proposal_key] =  anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("property_proposal"),
//       propertyId.toArrayLike(Buffer, "le", 8),
//       state_pda_key.toBuffer()
//     ],
//     program.programId
//   ) ;

//   const acc = await program.account.propertyProposal.fetch(property_proposal_key);

//   console.log(acc);


})
const propertyId = new anchor.BN(1);

it("approve property proposal",async()=>{

  for (let element = 0; element<3 ;element++) {
      const tx = await program.methods.approvePropertyProposal(
    country,
    sta,
    new anchor.BN(1)
  ).accounts(
    {signer:state_auth[element].publicKey}
  ).signers([state_auth[element]]).rpc()

  }

  //second
  for (let element = 0; element<3 ;element++) {
      const tx = await program.methods.approvePropertyProposal(
    country,
    sta,
    new anchor.BN(2)
  ).accounts(
    {signer:state_auth[element].publicKey}
  ).signers([state_auth[element]]).rpc()
  }  

  const [property_proposal_key] =  anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("property_proposal"),
      propertyId.toArrayLike(Buffer, "le", 8),
      state_pda_key.toBuffer()
    ],
    program.programId
  ) ;

  const acc = await program.account.propertyProposal.fetch(property_proposal_key);

  // console.log(acc);

  
  
  });


  it("execute property",async()=>{

    const tx = await program.methods.executePropertyProposal(
      country,
      sta,
      new anchor.BN(1),
      new anchor.BN(1),
    ).accounts({
      signer:state_auth1.publicKey
    }).signers([state_auth1]).rpc();


const [property_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("property"),
      propertyId.toArrayLike(Buffer, "le", 8),
      state_pda_key.toBuffer()
    ],
    program.programId,
  )


  //second
  const tx2 = await program.methods.executePropertyProposal(
      country,
      sta,
      new anchor.BN(2),
      new anchor.BN(2),
    ).accounts({
      signer:state_auth1.publicKey
    }).signers([state_auth1]).rpc();


// const acc = await program.account.propertyAccount.fetch(property_key);

// console.log(acc);



const [property_proposal_key] =  anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("property_proposal"),
      propertyId.toArrayLike(Buffer, "le", 8),
      state_pda_key.toBuffer()
    ],
    program.programId
  ) ;

  const acc2 = await program.account.propertyProposal.fetch(property_proposal_key);

  console.log(acc2);

  const [meta_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("property_metadata"),
      property_key.toBuffer()
    ],
    program.programId,
  )


const meta = await program.account.propertyAccountMetadata.fetch(meta_key);

console.log(meta);

  })

    ///SELL PROPERTY


// it("create sell proposal",async()=>{


//   const tx = await program.methods.createSellProposal(
//     new anchor.BN(1),
//     new anchor.BN(1),
//     new anchor.BN(1),
//     state_pda_key,
//     new anchor.BN(1000)
//   ).accounts(
//     {trustee:pro_vec[0].publicKey,}
//   ).signers([pro_vec[0]]).rpc();

//   // const [sell_proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
//   //   [
//   //     Buffer.from("SELLPROPERTY"),
//   //     propertySystemPda.toBuffer(),
//   //     propertyId.toArrayLike(Buffer, "le", 8),
//   //   ],
//   //   program.programId
//   // )

//   // const acc = await program.account.propertySellProposal.fetch(sell_proposal_key);

//   // console.log(acc);

// })
  
//   // APPROVE SELL PROPOSAL BY ARBITRATOR

//   it("approve sell proposal by arbitrator",async()=>{

//     for(let i = 0; i < 4; i++){await connection.requestAirdrop(pro_vec[i].publicKey, 1e9); // 1 SOL

//     await new Promise(resolve => setTimeout(resolve, 100));}

// //     const listener = program.addEventListener(
// //   "snapshotRequested",
// //   (event, slot) => {
// //     console.log("Event:", event);
// //     console.log("Slot:", slot);
// //   }
// // );

//     for(let i = 0; i < 2; i++){
      
//     const tx = await program.methods.sellProposalArbitrarVote(
//       new anchor.BN(1),
//       new anchor.BN(1),
//     ).accounts(
//       {
//         arbitrar:pro_vec[i].publicKey,
        
//       }
//     ).signers([pro_vec[i]]).rpc();
  
//     }
   

//     const [sell_proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("SELLPROPERTY"),
//         propertySystemPda.toBuffer(),
//         propertyId.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     )

//     const acc = await program.account.propertySellProposal.fetch(sell_proposal_key);

//     console.log(acc);
  
//   })

  let receiver1 = Keypair.generate();
  let receiver2 = Keypair.generate();
  let receiver3 = Keypair.generate();
  let receiver4 = Keypair.generate();
  let receiver5 = Keypair.generate();
  let receiver6 = Keypair.generate();

const [governanceMint] = anchor.web3.PublicKey.findProgramAddressSync(
  [
   Buffer.from("mint"),
  propertySystemPda.toBuffer(),
  ],
  program.programId
);

 it("split token in 6 accounts", async () => {

  const senderAta = getAssociatedTokenAddressSync(
    governanceMint,
    wallet.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  const receivers = [receiver1, receiver2, receiver3,receiver4,receiver5,receiver6];

  const tx = new Transaction();

  for (const r of receivers) {
    const receiverAta = await getAssociatedTokenAddress(
      governanceMint,
      r.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const ataInfo = await provider.connection.getAccountInfo(receiverAta);

    if (!ataInfo) {
      tx.add(
        createAssociatedTokenAccountInstruction(
          wallet.publicKey,
          receiverAta,
          r.publicKey,
          governanceMint,
          TOKEN_2022_PROGRAM_ID,
          ASSOCIATED_TOKEN_PROGRAM_ID
        )
      );
    }

    tx.add(
      createTransferInstruction(
        senderAta,
        receiverAta,
        wallet.publicKey,
        100,
        [],
        TOKEN_2022_PROGRAM_ID
      )
    );
  }

  await provider.sendAndConfirm(tx);

  console.log("Tokens transferred to receivers");
  
  for (const r of receivers) {
    const receiverAta = await getAssociatedTokenAddress(
      governanceMint,
      r.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const accountInfo = await provider.connection.getAccountInfo(receiverAta);

    if (accountInfo) {
      const accountData = Buffer.from(accountInfo.data);
      const amount = accountData.readBigUInt64LE(64); // amount is at offset 64
      console.log(`Receiver ${r.publicKey.toBase58()} has ${amount} tokens`);
    } else {
      console.log(`Receiver ${r.publicKey.toBase58()} does not have an associated token account`);
    }


  }

})



// it("delete sell proposal",async()=>{


//   const proposalId = new anchor.BN(1)

//    const [sellProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("SELLPROPERTY"),
//       propertySystemPda.toBuffer(),
//       proposalId.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );

//   const acc2 = await program.account.propertySellProposal.fetch(sellProposalKey);

//   console.log(acc2);
  


//   const tx = await program.methods.deleteSell(
//     new anchor.BN(1),
//     new anchor.BN(1)
//   ).accounts({
//     trustee:pro1.publicKey
//   }).signers([pro1]).rpc();


//   const acc = await program.account.propertySellProposal.fetch(sellProposalKey);

//   console.log(acc);

// })

// let starttime;

// it("submit snapshot request", async () => {
//   const proposalId = new anchor.BN(1);

//   const [sellProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("SELLPROPERTY"),
//       propertySystemPda.toBuffer(),
//       proposalId.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );

//   const merkleRoot = buildMerkleRoot([
//     buildSellProposalLeaf(receiver1.publicKey, sellProposalKey, governanceMint, 200),
//     buildSellProposalLeaf(receiver2.publicKey, sellProposalKey, governanceMint, 200),
//     buildSellProposalLeaf(receiver3.publicKey, sellProposalKey, governanceMint, 200),
//   ]);

//  // console.log("pubkey", wallet.publicKey);
//   const tx = await program.methods.submitSnapshotForSellProposal(
//     propertySystemPda,
//     proposalId,
//     merkleRoot,
//     2,
//     20,
//     new anchor.BN(500)
//   ).accounts(
//     [wallet.publicKey]
//   ).signers([wallet.payer]).rpc()

//   const [sell_proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("SELLPROPERTY"),
//         propertySystemPda.toBuffer(),
//         proposalId.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     )

//     const acc = await program.account.propertySellProposal.fetch(sell_proposal_key);

//     starttime = acc.startTime;

//     console.log(acc);

// })



// // it("vote for sell proposal before end_time should pass",async()=>{
// //   advanceClockBy(svm, 129_600n);

// //   const proposalId = new anchor.BN(1);
// //   const airdropSignature = await connection.requestAirdrop(receiver1.publicKey, 1e9);
// //   const latestBlockhash = await connection.getLatestBlockhash();
// //   await connection.confirmTransaction(
// //     {
// //       signature: airdropSignature,
// //       ...latestBlockhash,
// //     },
// //     "confirmed"
// //   );

// //   const voterBalance = await connection.getBalance(receiver1.publicKey);
// //   assert.isAtLeast(voterBalance, 1_000_000);
// //   const [sellProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
// //     [
// //       Buffer.from("SELLPROPERTY"),
// //       propertySystemPda.toBuffer(),
// //       proposalId.toArrayLike(Buffer, "le", 8),
// //     ],
// //     program.programId
// //   );

// //   const snapshotEntries = [
// //     { voter: receiver1.publicKey, votingPower: 200 },
// //     { voter: receiver2.publicKey, votingPower: 200 },
// //     { voter: receiver3.publicKey, votingPower: 200 },
// //   ];

// //   const sellProposal = await program.account.propertySellProposal.fetch(sellProposalKey);
// //   assert.isTrue(Number(svm.getClock().unixTimestamp) < sellProposal.endTime.toNumber());

// //   const voter1proof = buildSellProposalProof(
// //     snapshotEntries,
// //     0,
// //     sellProposalKey,
// //     governanceMint
// //   );

// //   const vote1 = await program.methods.votingForSellProposal(
// //     proposalId,
// //     new anchor.BN(1),
// //     voter1proof,
// //     new anchor.BN(200),
// //     true,
// //   ).accounts({
// //     signer:receiver1.publicKey,
// //   }).signers([receiver1]).rpc()



// // })




// // it("skip time to voting end",async() =>{
// //   advanceClockBy(svm, 129_600n);

// // })

// it("vote for sell proposal at end_time should pass",async()=>{

//   const proposalId = new anchor.BN(1);
//   const airdropSignature = await connection.requestAirdrop(receiver2.publicKey, 1e9);
//   const latestBlockhash = await connection.getLatestBlockhash();
//   await connection.confirmTransaction(
//     {
//       signature: airdropSignature,
//       ...latestBlockhash,
//     },
//     "confirmed"
//   );

//   const voterBalance = await connection.getBalance(receiver2.publicKey);
//   assert.isAtLeast(voterBalance, 1_000_000);
//   const [sellProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("SELLPROPERTY"),
//       propertySystemPda.toBuffer(),
//       proposalId.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );

//   const snapshotEntries = [
//     { voter: receiver1.publicKey, votingPower: 200 },
//     { voter: receiver2.publicKey, votingPower: 200 },
//     { voter: receiver3.publicKey, votingPower: 200 },
//   ];

//   const sellProposal = await program.account.propertySellProposal.fetch(sellProposalKey);
//   // assert.equal(Number(svm.getClock().unixTimestamp), sellProposal.endTime.toNumber());

//   const voter2proof = buildSellProposalProof(
//     snapshotEntries,
//     1,
//     sellProposalKey,
//     governanceMint
//   );

//   await program.methods.votingForSellProposal(
//     proposalId,
//     new anchor.BN(1),
//     voter2proof,
//     new anchor.BN(200),
//     true,
//   ).accounts({
//     signer:receiver2.publicKey,
//   }).signers([receiver2]).rpc();
// })

// it("skip time past voting end",async() =>{
//   advanceClockBy(svm, 1n);
// })

// it("vote for sell proposal after end_time should fail",async()=>{

//   const proposalId = new anchor.BN(1);
//   const airdropSignature = await connection.requestAirdrop(receiver3.publicKey, 1e9);
//   const latestBlockhash = await connection.getLatestBlockhash();
//   await connection.confirmTransaction(
//     {
//       signature: airdropSignature,
//       ...latestBlockhash,
//     },
//     "confirmed"
//   );

//   const voterBalance = await connection.getBalance(receiver3.publicKey);
//   assert.isAtLeast(voterBalance, 1_000_000);
//   const [sellProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("SELLPROPERTY"),
//       propertySystemPda.toBuffer(),
//       proposalId.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );

//   const snapshotEntries = [
//     { voter: receiver1.publicKey, votingPower: 200 },
//     { voter: receiver2.publicKey, votingPower: 200 },
//     { voter: receiver3.publicKey, votingPower: 200 },
//   ];

//   const sellProposal = await program.account.propertySellProposal.fetch(sellProposalKey);
//   // assert.isTrue(Number(svm.getClock().unixTimestamp) > sellProposal.endTime.toNumber());

//   const voter3proof = buildSellProposalProof(
//     snapshotEntries,
//     2,
//     sellProposalKey,
//     governanceMint
//   );

//     await program.methods.votingForSellProposal(
//       proposalId,
//       new anchor.BN(1),
//       voter3proof,
//       new anchor.BN(200),
//       true,
//     ).accounts({
//       signer:receiver3.publicKey,
//     }).signers([receiver3]).rpc();

   
 
// })

// it("skip time to voting end",async() =>{
//   advanceClockBy(svm, 432000n);

// })
// it("skip time past voting end",async() =>{
//   advanceClockBy(svm, 5n);
// })




// it("finalize the sell proposal",async()=> {
// const proposalId = new anchor.BN(1);

//     const tx = await program.methods.sellProposalFinalize(
//       new anchor.BN(1),
//       propertySystemPda
//     ).accounts(
//         {
//           signer:wallet.publicKey
//         }
//     ).signers([wallet.payer]).rpc();

    

// const [sellProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("SELLPROPERTY"),
//       propertySystemPda.toBuffer(),
//       proposalId.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );

// const sellProposal = await program.account.propertySellProposal.fetch(sellProposalKey);

//   console.log(sellProposal);
  


// //  assert.isTrue("passed" in sellProposal.status);

// })

// const systemId2 = new anchor.BN(2);
//   const [buyer_key ] =  anchor.web3.PublicKey.findProgramAddressSync(
//     [
//     Buffer.from("property_system_account"),
//     systemId2.toArrayLike(Buffer, "le", 8),
//   ],
//   program.programId
//   );


// it("buy property proposal",async()=>{


//   const tx = await program.methods.createBuyProposal(
//     new anchor.BN(1),
//     new anchor.BN(2),
//     propertySystemPda,
//     new anchor.BN(1),
//     state_pda_key,
//     new anchor.BN(1)
//   ).accounts(
//     {trustee:prosys1.publicKey}
//   ).signers(
//     [prosys1]
//   ).rpc();



//   const [buy_proposal_key ] = await anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("BUYPROPERTY"),
//       buyer_key.toBuffer(),
//       propertyId.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );

//   const acc = await program.account.propertyBuyProposal.fetch(buy_proposal_key);

//   console.log(acc);

// })



// it("approve buy proposal by arbitrator",async()=>{ 


//   for(let i = 0; i < 4; i++){await connection.requestAirdrop(pro_vec2[i].publicKey, 1e9); // 1 SOL

//   await new Promise(resolve => setTimeout(resolve, 100));}

//   for(let i = 0; i < 2; i++){
      
//     const tx = await program.methods.buyProposalArbitrarVote(
//       new anchor.BN(1),
//       new anchor.BN(2),
//     ).accounts(
//       {
//         arbitrar:pro_vec2[i].publicKey,
        
//       }
//     ).signers([pro_vec2[i]]).rpc();
  
//     }
   

//     const systemId2 = new anchor.BN(2);
//     const [buyer_key ] = await anchor.web3.PublicKey.findProgramAddressSync(
//       [
//       Buffer.from("property_system_account"),
//       systemId2.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//     );
  
//     const [buy_proposal_key ] =  anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("BUYPROPERTY"),
//         buyer_key.toBuffer(), 
//         propertyId.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );
  
//     const acc = await program.account.propertyBuyProposal.fetch(buy_proposal_key);
  
//     console.log(acc);

//  })


//  const [buyer_governanceMint] = anchor.web3.PublicKey.findProgramAddressSync(
//   [
//    Buffer.from("mint"),
//   buyer_key.toBuffer(),
//   ],
//   program.programId
// );

//   let buy_receiver1 = Keypair.generate();
//   let buy_receiver2 = Keypair.generate();
//   let buy_receiver3 = Keypair.generate();


// it("split token in 3 accounts for buy proposal", async () => {

//   const senderAta = getAssociatedTokenAddressSync(
//     buyer_governanceMint,
//     wallet.publicKey,
//     false,
//     TOKEN_2022_PROGRAM_ID,
//     ASSOCIATED_TOKEN_PROGRAM_ID
//   );


//   const buy_receivers = [buy_receiver1, buy_receiver2, buy_receiver3];

//   const tx = new Transaction();

//   for (const r of buy_receivers) {
//     const receiverAta = await getAssociatedTokenAddress(
//       buyer_governanceMint,
//       r.publicKey,
//       false,
//       TOKEN_2022_PROGRAM_ID,
//       ASSOCIATED_TOKEN_PROGRAM_ID
//     );

//     const ataInfo = await provider.connection.getAccountInfo(receiverAta);

//     if (!ataInfo) {
//       tx.add(
//         createAssociatedTokenAccountInstruction(
//           wallet.publicKey,
//           receiverAta,
//           r.publicKey,
//           buyer_governanceMint,
//           TOKEN_2022_PROGRAM_ID,
//           ASSOCIATED_TOKEN_PROGRAM_ID
//         )
//       );
//     }

//     tx.add(
//       createTransferInstruction(
//         senderAta,
//         receiverAta,
//         wallet.publicKey,
//         200,
//         [],
//         TOKEN_2022_PROGRAM_ID
//       )
//     );
//   }

//   await provider.sendAndConfirm(tx);

//   console.log("Tokens transferred to receivers");
  
//   // for (const r of buy_receivers) {
//   //   const receiverAta = await getAssociatedTokenAddress(
//   //     buyer_governanceMint,
//   //     r.publicKey,
//   //     false,
//   //     TOKEN_2022_PROGRAM_ID,
//   //     ASSOCIATED_TOKEN_PROGRAM_ID
//   //   );

//   //   const accountInfo = await provider.connection.getAccountInfo(receiverAta);

//   //   if (accountInfo) {
//   //     const accountData = Buffer.from(accountInfo.data);
//   //     const amount = accountData.readBigUInt64LE(64); // amount is at offset 64
//   //     console.log(`Receiver ${r.publicKey.toBase58()} has ${amount} tokens`);
//   //   } else {
//   //     console.log(`Receiver ${r.publicKey.toBase58()} does not have an associated token account`);
//   //   }

//   // }
// })

// // it("delete buy proposal",async()=>{


// //   const proposalId = new anchor.BN(1)

// //    const [buyProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
// //     [
// //       Buffer.from("BUYPROPERTY"),
// //       buyer_key.toBuffer(),
// //       proposalId.toArrayLike(Buffer, "le", 8),
// //     ],
// //     program.programId
// //   );

// //   const acc2 = await program.account.propertyBuyProposal.fetch(buyProposalKey);

// //   console.log(acc2);
  


// //   const tx = await program.methods.deleteBuyProposal(
// //     new anchor.BN(1),
// //     new anchor.BN(2)
// //   ).accounts({
// //     trustee:prosys1.publicKey
// //   }).signers([prosys1]).rpc();


// //   const acc = await program.account.propertyBuyProposal.fetch(buyProposalKey);

// //   console.log(acc);

// // })


// it("submit for buy proposal",async()=>{

//    const proposalId = new anchor.BN(1);

//   const [buyProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("BUYPROPERTY"),
//       buyer_key.toBuffer(),
//       proposalId.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );

//   const merkleRoot2 = buildMerkleRoot([
//     buildBuyProposalLeaf(buy_receiver1.publicKey, buyProposalKey, buyer_governanceMint, 200),
//     buildBuyProposalLeaf(buy_receiver2.publicKey, buyProposalKey, buyer_governanceMint, 200),
//     buildBuyProposalLeaf(buy_receiver3.publicKey, buyProposalKey, buyer_governanceMint, 200),
//   ]);

//  // console.log("pubkey", wallet.publicKey);
//   const tx = await program.methods.buySubmitSnapshot(
//     buyer_key,
//     proposalId,
//     merkleRoot2,
//     2,
//     20,
//     new anchor.BN(500)
//   ).accounts(
//     [wallet.publicKey]
//   ).signers([wallet.payer]).rpc()

//   const [buy_proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("BUYPROPERTY"),
//         buyer_key.toBuffer(),
//         propertyId.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     )

//     const acc = await program.account.propertyBuyProposal.fetch(buy_proposal_key);

//     starttime = acc.startTime;

//     console.log(acc);



// })



// it("vote for buy proposal before end_time should pass",async()=>{
//   advanceClockBy(svm, 129_600n);

//   const proposalId = new anchor.BN(1);
//   const airdropSignature = await connection.requestAirdrop(buy_receiver1.publicKey, 1e9);
//   const latestBlockhash = await connection.getLatestBlockhash();
//   await connection.confirmTransaction(
//     {
//       signature: airdropSignature,
//       ...latestBlockhash,
//     },
//     "confirmed"
//   );

//   const voterBalance = await connection.getBalance(buy_receiver1.publicKey);
//   assert.isAtLeast(voterBalance, 1_000_000);
//   const [buyProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("BUYPROPERTY"),
//       buyer_key.toBuffer(),
//       proposalId.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );

//   const snapshotEntries = [
//     { voter: buy_receiver1.publicKey, votingPower: 200 },
//     { voter: buy_receiver2.publicKey, votingPower: 200 },
//     { voter: buy_receiver3.publicKey, votingPower: 200 },
//   ];

//   const buyProposal = await program.account.propertyBuyProposal.fetch(buyProposalKey);
//   assert.isTrue(Number(svm.getClock().unixTimestamp) < buyProposal.endTime.toNumber());
//   const voter1proof = buildBuyProposalProof(
//     snapshotEntries,
//     0,
//     buyProposalKey,
//     buyer_governanceMint
//   );

//   const vote1 = await program.methods.buyProposalVoting(
//     proposalId,
//     new anchor.BN(2),
//     voter1proof,
//     new anchor.BN(200),
//     true,
//   ).accounts({
//     signer:buy_receiver1.publicKey,
//   }).signers([buy_receiver1]).rpc()


//   //   const buyProposal2 = await program.account.propertyBuyProposal.fetch(buyProposalKey);
//   // console.log(buyProposal2);
  

// })



// // it("skip time to voting end",async() =>{
// //   advanceClockBy(svm, 129_600n);

// // })

// it("vote for buy proposal at end_time should pass",async()=>{

//   const proposalId = new anchor.BN(1);
//   const airdropSignature = await connection.requestAirdrop(buy_receiver2.publicKey, 1e9);
//   const latestBlockhash = await connection.getLatestBlockhash();
//   await connection.confirmTransaction(
//     {
//       signature: airdropSignature,
//       ...latestBlockhash,
//     },
//     "confirmed"
//   );

//   const voterBalance = await connection.getBalance(buy_receiver2.publicKey);
//   assert.isAtLeast(voterBalance, 1_000_000);
//   const [buyProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("BUYPROPERTY"),
//       buyer_key.toBuffer(),
//       proposalId.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );

//   const snapshotEntries = [
//     { voter: buy_receiver1.publicKey, votingPower: 200 },
//     { voter: buy_receiver2.publicKey, votingPower: 200 },
//     { voter: buy_receiver3.publicKey, votingPower: 200 },
//   ];

//   const buyProposal = await program.account.propertyBuyProposal.fetch(buyProposalKey);
//   // assert.equal(Number(svm.getClock().unixTimestamp), buyProposal.endTime.toNumber());

//   const voter2proof = buildBuyProposalProof(
//     snapshotEntries,
//     1,
//     buyProposalKey,
//     buyer_governanceMint
//   );

//   await program.methods.buyProposalVoting(
//     proposalId,
//     new anchor.BN(2),
//     voter2proof,
//     new anchor.BN(200),
//     true,
//   ).accounts({
//     signer:buy_receiver2.publicKey,
//   }).signers([buy_receiver2]).rpc();
// })



// it("vote for sell proposal after end_time should fail",async()=>{

//   const proposalId = new anchor.BN(1);
//   const airdropSignature = await connection.requestAirdrop(buy_receiver3.publicKey, 1e9);
//   const latestBlockhash = await connection.getLatestBlockhash();
//   await connection.confirmTransaction(
//     {
//       signature: airdropSignature,
//       ...latestBlockhash,
//     },
//     "confirmed"
//   );
  
//   const voterBalance = await connection.getBalance(buy_receiver3.publicKey);
//   assert.isAtLeast(voterBalance, 1_000_000);
//   const [buyProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("BUYPROPERTY"),
//       buyer_key.toBuffer(),
//       proposalId.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );

//   const snapshotEntries = [
//     { voter: buy_receiver1.publicKey, votingPower: 200 },
//     { voter: buy_receiver2.publicKey, votingPower: 200 },
//     { voter: buy_receiver3.publicKey, votingPower: 200 },
//   ];

//   const buyProposal = await program.account.propertyBuyProposal.fetch(buyProposalKey);
//   // assert.isTrue(Number(svm.getClock().unixTimestamp) > sellProposal.endTime.toNumber());

//   const voter3proof = buildBuyProposalProof(
//     snapshotEntries,
//     2,
//     buyProposalKey,
//     buyer_governanceMint
//   );

//     await program.methods.buyProposalVoting(
//       proposalId,
//       new anchor.BN(2),
//       voter3proof,
//       new anchor.BN(200),
//       true,
//     ).accounts({
//       signer:buy_receiver3.publicKey,
//     }).signers([buy_receiver3]).rpc();


//    const buyProposal2 = await program.account.propertyBuyProposal.fetch(buyProposalKey);
//   console.log(buyProposal2);

   
 
// })


// it("skip time to voting end",async() =>{
//   advanceClockBy(svm, 432000n);

// })
// it("skip time past voting end",async() =>{
//   advanceClockBy(svm, 5n);
// })

// it("finalize the buy proposal",async()=> {
//   const proposalId = new anchor.BN(1);

//     const tx = await program.methods.buyProposalFinalize(
//       proposalId,
//       buyer_key,
//     ).accounts(
//         {
//           signer:wallet.publicKey
//         }
//     ).signers([wallet.payer]).rpc();

//     const [buyProposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("BUYPROPERTY"),
//         buyer_key.toBuffer(),
//         proposalId.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );

//     const buyProposal = await program.account.propertyBuyProposal.fetch(buyProposalKey);

//     console.log(buyProposal); 

// })

// let uni_mint ;


// it ("create universal mint ",async()=>{

//   const mintKeypair = Keypair.generate();
  
//   let universal_mint = await createMint(
//     connection,
//     wallet.payer,
//     wallet.publicKey,
//     null,
//     0,
//     mintKeypair,
//     {commitment:"confirmed"},
//     TOKEN_2022_PROGRAM_ID
//   );

//   uni_mint = universal_mint;

//   console.log("Universal Mint created:", universal_mint.toBase58());

// const [buyerReinvestmentPdaKey] = anchor.web3.PublicKey.findProgramAddressSync(
//   [
//     Buffer.from("reinvestment"),
//     buyer_key.toBuffer(),
//   ],
//   program.programId
// );

// const buyerReinvestmentPdaInfo = await program.account.reinvestmentPda.fetch(buyerReinvestmentPdaKey);

// console.log("Buyer Reinvestment PDA:", buyerReinvestmentPdaKey.toBase58());
// console.log("Buyer Reinvestment PDA Info:", buyerReinvestmentPdaInfo);




// const universalMintAccountInfo = await connection.getAccountInfo(universal_mint);

  
//   const reinvestmentAtaAddress = getAssociatedTokenAddressSync(
//     universal_mint,
//     buyerReinvestmentPdaKey,
//     true,  
//     TOKEN_2022_PROGRAM_ID,
//     ASSOCIATED_TOKEN_PROGRAM_ID
//   );


//   const ataInfo = await connection.getAccountInfo(reinvestmentAtaAddress);
//   if (!ataInfo) {
//     const createAtaIx = createAssociatedTokenAccountInstruction(
//       wallet.publicKey,  // payer
//       reinvestmentAtaAddress,
//       buyerReinvestmentPdaKey,  // owner (PDA)
//       universal_mint,  // mint PublicKey
//       TOKEN_2022_PROGRAM_ID,
//       ASSOCIATED_TOKEN_PROGRAM_ID
//     );
//     const tx = new Transaction().add(createAtaIx);
//     await provider.sendAndConfirm(tx);
//     console.log("ATA created for PDA:", reinvestmentAtaAddress.toBase58());
//   }

//   const mint_to_reinvestment_pda = await mintTo(
//     connection,
//     wallet.payer,
//     universal_mint,
//     reinvestmentAtaAddress,
//     wallet.publicKey,
//     1000,
//     [],
//     {commitment:"confirmed"},
//     TOKEN_2022_PROGRAM_ID
//   );

//   // console.log("Minted 2000 tokens to reinvestment ATA");

//   // Fetch the ATA info to verify
//   const reinvestmentAtaInfo = await getAccount(
//     connection,
//     reinvestmentAtaAddress,
//     undefined,
//     TOKEN_2022_PROGRAM_ID
//   );

//   console.log("Reinvestment ATA balance:", reinvestmentAtaInfo);

// })




// it("excute the buy prposal with reinvestment",async()=>{


//   const proposalId = new anchor.BN(1);

//   const tx = await program.methods.executeBuyProposal(
//     new anchor.BN(1),
//     new anchor.BN(2),
//      new anchor.BN(1),
//      new anchor.BN(1),
//      state_pda_key,
//      new anchor.BN(1),
//   ).accounts(
//     {
//       trustee:prosys1.publicKey,
//       mint:uni_mint,
//       tokenProgram:TOKEN_2022_PROGRAM_ID,
      
//     }
//   ).signers([prosys1]).rpc();

//   const [buyreinvestmentKey] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("reinvestment"),
//         buyer_key.toBuffer(),
       
//       ],
//       program.programId
//     );

//   //   const buyProposal = await program.account.propertyBuyProposal.fetch(buyProposalKey);

  
//   // console.log(buyProposal);

//   //  const [sellproposalKey] = anchor.web3.PublicKey.findProgramAddressSync(
//   //     [
//   //       Buffer.from("SELLPROPERTY"),
//   //       propertySystemPda.toBuffer(),
//   //       proposalId.toArrayLike(Buffer, "le", 8),
//   //     ],
//   //     program.programId
//   //   );

//   //   const sellProposal = await program.account.propertySellProposal.fetch(sellproposalKey);

  
//   // console.log(sellProposal);



//    const [selltreasuryKey] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("treasury"),
//         propertySystemPda.toBuffer(),
        
//       ],
//       program.programId
//     );



// const sellerAtaAddress = getAssociatedTokenAddressSync(
//     uni_mint,
//     selltreasuryKey,
//     true,  
//     TOKEN_2022_PROGRAM_ID,
//     ASSOCIATED_TOKEN_PROGRAM_ID
//   );

// const sellerAtaInfo = await connection.getAccountInfo(sellerAtaAddress);

// if (sellerAtaInfo) {
//   const accountData = Buffer.from(sellerAtaInfo.data);
//   const amount = accountData.readBigUInt64LE(64); // amount is at offset 64
//   console.log(`Seller ATA has ${amount} tokens`);
// } else {
//   console.log(`Seller ATA does not exist`);
// }


// const buyerAtaAddress = getAssociatedTokenAddressSync(
//     uni_mint,
//     buyreinvestmentKey,
//     true,  
//     TOKEN_2022_PROGRAM_ID,
//     ASSOCIATED_TOKEN_PROGRAM_ID
//   );

// const buyerAtaInfo = await connection.getAccountInfo(buyerAtaAddress);

// if (buyerAtaInfo) {
//   const accountData = Buffer.from(buyerAtaInfo.data);
//   const amount = accountData.readBigUInt64LE(64); // amount is at offset 64
//   console.log(`Buyer ATA has ${amount} tokens`);
// } else {
//   console.log(`Buyer ATA does not exist`);
// }


// })




it("trustee resign proposal",async()=>{

for(let i =0;i<2;i++){

  
  const tx = await program.methods.trusteeResign(
    new anchor.BN(1),
    new anchor.BN(1),
  ).accounts(
    {
      trustee:pro_vec[i].publicKey
    }
  ).signers([pro_vec[i]]).rpc();

}


  const [resgination_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("trustee_resignation"),
      propertySystemPda.toBuffer(),
      pro1.publicKey.toBuffer()
    ],
    program.programId
  ) 

  const acc = await program.account.resignation.fetch(resgination_key);

  console.log(acc);

   const [election_proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("elect_trustee"),
      propertySystemPda.toBuffer(),
      new anchor.BN(1).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  ) 
  const acc2 = await program.account.electAuthority.fetch(election_proposal_key);

  console.log(acc2);

})


it("arbitrar vote to elect trustee",async()=>{

  for (let i = 2;i<5;i++){

     const tx = await program.methods.arbitrarApproveTrusteeElection(
    new anchor.BN(1),
    new anchor.BN(1),
  ).accounts({
    signer:pro_vec[i].publicKey
  }).signers([pro_vec[i]]).rpc()
  }

   const [election_proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("elect_trustee"),
      propertySystemPda.toBuffer(),
      new anchor.BN(1).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  ) 
  const acc2 = await program.account.electAuthority.fetch(election_proposal_key);

  console.log(acc2);


  const [election_vote_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("trustee_election_receipt"),
      election_proposal_key.toBuffer(),
      pro3.publicKey.toBuffer()
    ],
    program.programId
  ) 

  const acc3 = await program.account.voteReceiptForAuthorityElection.fetch(election_vote_key);

  console.log(acc3);

})

it("submits snapshot for trustee election",async()=>{


   const proposalId = new anchor.BN(1);

  const [trustee_election_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("elect_trustee"),
      propertySystemPda.toBuffer(),
      proposalId.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const merkleRoot = buildMerkleRoot([
    buildAuthorityLeaf(receiver1.publicKey, trustee_election_key, governanceMint, 100,0),
    buildAuthorityLeaf(receiver2.publicKey, trustee_election_key, governanceMint, 100,0),
    buildAuthorityLeaf(receiver3.publicKey, trustee_election_key, governanceMint, 100,0),
    buildAuthorityLeaf(receiver4.publicKey, trustee_election_key, governanceMint, 100,0),
    buildAuthorityLeaf(receiver5.publicKey, trustee_election_key, governanceMint, 100,0),
    buildAuthorityLeaf(receiver6.publicKey, trustee_election_key, governanceMint, 100,0)
  ]);

 // console.log("pubkey", wallet.publicKey);
  const tx = await program.methods.submitSnapshotForAuthority(
    proposalId,
    propertySystemPda,
    2,
    2,
    2,
    2,
    merkleRoot
  ).accounts(
    [wallet.publicKey]
  ).signers([wallet.payer]).rpc()


    const acc = await program.account.electAuthority.fetch(trustee_election_key);



    console.log(acc);



})

let candidate1 = Keypair.generate();
let candidate2 = Keypair.generate();
let candidate3 = Keypair.generate();
let candidate4 = Keypair.generate();

let candidate_vec = [candidate1,candidate2,candidate3,candidate4];


it("create candiadate profile ",async()=>{

  for(let i = 0; i<4 ;i++){

  await connection.requestAirdrop(candidate_vec[i].publicKey, 1e9);

}

for(let i = 0; i<4 ;i++){
  const tx = await program.methods.createCandidateProfile(
  legal_doc_hash,
).accounts({
  candidate:candidate_vec[i].publicKey,
}).signers([wallet.payer]).rpc()


}

const [profile_key] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("candidate_profile"),
    candidate1.publicKey.toBuffer()
  ],
  program.programId
)

// const acc = await program.account.candidateProfile.fetch(profile_key);

// console.log(acc);



})

let proposalId = new anchor.BN(1);
const [trustee_election_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("elect_trustee"),
      propertySystemPda.toBuffer(),
      proposalId.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );


it("candidate profile submission for trustee",async()=>{


for(let i = 0; i<4 ;i++){
  const tx = await program.methods.submitTrusteeCandidate(
      new anchor.BN(1),
      new anchor.BN(1),
  ).accounts(
    {
      signer:candidate_vec[i].publicKey,
    }
  ).signers([candidate_vec[i]]).rpc();


}

  

  

  const [candidate_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      trustee_election_key.toBuffer(),
      candidate1.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc = await program.account.authorityCandidate.fetch(candidate_recepit);

  console.log(acc);

})

it("skip time to voting end",async() =>{
  advanceClockBy(svm, 180800n);

})

it("vote for candidate",async()=>{

  const snapshotEntries = [
    { voter: receiver1.publicKey, votingPower: 100,authoritytype:0 },
    { voter: receiver2.publicKey, votingPower: 100,authoritytype:0  },
    { voter: receiver3.publicKey, votingPower: 100,authoritytype:0  },
     { voter: receiver4.publicKey, votingPower: 100 ,authoritytype:0 },
      { voter: receiver5.publicKey, votingPower: 100,authoritytype:0  },
       { voter: receiver6.publicKey, votingPower: 100 ,authoritytype:0 },
  ];


  let proposalId = new anchor.BN(1);

 const [trustee_election_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("elect_trustee"),
      propertySystemPda.toBuffer(),
      proposalId.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );



  const elect_proposal = await program.account.electAuthority.fetch(trustee_election_key);

  // assert.isTrue(Number(svm.getClock().unixTimestamp) < sellProposal.endTime.toNumber());

 
 const receivers = [receiver1, receiver2, receiver3,receiver4,receiver5,receiver6];

  for(let i = 0; i < 6; i++){await connection.requestAirdrop(receivers[i].publicKey, 1e9)};


  // console.log(elect_proposal);
  

  for(let i =0 ;i<2;i++){


     const voter1proof = buildAuthorityProof(
    snapshotEntries,
    i,
    trustee_election_key,
    governanceMint
  );

const tx = await program.methods.voteForTrusteeCandiate(
    new anchor.BN(1),
    new anchor.BN(1),
    candidate1.publicKey,
    voter1proof,
    new anchor.BN(100),
  ).accounts(
    {
      signer:receivers[i].publicKey
    }
  ).signers([receivers[i]]).rpc();


  }

  for(let i =2 ;i<4;i++){
     const voter1proof = buildAuthorityProof(
    snapshotEntries,
    i,
    trustee_election_key,
    governanceMint
  );

const tx1 = await program.methods.voteForTrusteeCandiate(
    new anchor.BN(1),
    new anchor.BN(1),
    candidate2.publicKey,
    voter1proof,
    new anchor.BN(100),
  ).accounts(
    {
      signer:receivers[i].publicKey
    }
  ).signers([receivers[i]]).rpc();

  }

    
 const voter1proof = buildAuthorityProof(
    snapshotEntries,
    4,
    trustee_election_key,
    governanceMint
  );
const tx2 = await program.methods.voteForTrusteeCandiate(
    new anchor.BN(1),
    new anchor.BN(1),
    candidate3.publicKey,
    voter1proof,
    new anchor.BN(100),
  ).accounts(
    {
      signer:receiver5.publicKey
    }
  ).signers([receiver5]).rpc();


 const voter6proof = buildAuthorityProof(
    snapshotEntries,
    5,
    trustee_election_key,
    governanceMint
  );
  const tx3 = await program.methods.voteForTrusteeCandiate(
    new anchor.BN(1),
    new anchor.BN(1),
    candidate4.publicKey,
    voter6proof,
    new anchor.BN(100),
  ).accounts(
    {
      signer:receiver6.publicKey
    }
  ).signers([receiver6]).rpc();

  





//  for(let i =0 ;i<4;i++){

//    const [candidate_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("authority_candidate"),
//       propertySystemPda.toBuffer(),
//       trustee_election_key.toBuffer(),
//       candidate_vec[i].publicKey.toBuffer()
//     ],
//     program.programId
//   )

//   const acc = await program.account.authorityCandidate.fetch(candidate_recepit);

//   console.log(acc);

//  }

})

it("skip time to 2 days ",async() =>{
  advanceClockBy(svm, 180800n);

})

it("add new trustee",async()=>{
  

  const tx = await program.methods.addNewTrustee(
    candidate2.publicKey,
    new anchor.BN(1),
    new anchor.BN(1),
    2
  ).accounts({
    signer:pro1.publicKey}
  ).signers([pro1]).rpc();

  

  const [rankacc_key2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("ranking_account"),
      Buffer.from([2]),
      trustee_election_key.toBuffer(),
      propertySystemPda.toBuffer()
    ],
    program.programId
  ) 

  let acc = await program.account.rankingAccount.fetch(rankacc_key2);

  console.log(acc);


  const tx2 = await program.methods.addNewTrustee(
    candidate3.publicKey,
    new anchor.BN(1),
    new anchor.BN(1),
    1
  ).accounts({
    signer:pro1.publicKey}
  ).signers([pro1]).rpc();

  const [rankacc_key3] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("ranking_account"),
      Buffer.from([1]),
      trustee_election_key.toBuffer(),
      propertySystemPda.toBuffer()
    ],
    program.programId
  ) 

  let acc2 = await program.account.rankingAccount.fetch(rankacc_key3);

  console.log(acc2);


  //   const [candidate_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("authority_candidate"),
  //     propertySystemPda.toBuffer(),
  //     trustee_election_key.toBuffer(),
  //     candidate2.publicKey.toBuffer()
  //   ],
  //   program.programId
  // )

  // const acc3 = await program.account.authorityCandidate.fetch(candidate_recepit);

  // console.log(acc3);

  //  const [candidate_recepit2] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("authority_candidate"),
  //     propertySystemPda.toBuffer(),
  //     trustee_election_key.toBuffer(),
  //     candidate3.publicKey.toBuffer()
  //   ],
  //   program.programId
  // )

  // const acc4 = await program.account.authorityCandidate.fetch(candidate_recepit2);

  // console.log(acc4);


})


it("adjust ranking",async()=>{

  const tx = await program.methods.adjustRanks(
    new anchor.BN(1),
    propertySystemPda,
    candidate2.publicKey,
    candidate3.publicKey,
    2,1
  ).accounts(
    {signer:wallet.publicKey}
  ).signers([wallet.payer]).rpc();


    const [candidate_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      trustee_election_key.toBuffer(),
      candidate2.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc3 = await program.account.authorityCandidate.fetch(candidate_recepit);

  console.log(acc3);

   const [candidate_recepit2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      trustee_election_key.toBuffer(),
      candidate3.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc4 = await program.account.authorityCandidate.fetch(candidate_recepit2);

  console.log(acc4);
  


})

it("skip time to 2 days ",async() =>{
  advanceClockBy(svm, 170800n);

})


it("challenge the new trustee",async()=>{




  // const elect_proposal = await program.account.electAuthority.fetch(trustee_election_key);


  // console.log(elect_proposal);

    const [candidate_recepit2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      trustee_election_key.toBuffer(),
      candidate3.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc22 = await program.account.authorityCandidate.fetch(candidate_recepit2);

  console.log(acc22);
  

  const tx = await program.methods.challengeAgainstNewTrustee(
    new anchor.BN(1),
    candidate1.publicKey,
    candidate3.publicKey,
    2,
    new anchor.BN(1),
  ).accounts(
    {signer: candidate1.publicKey}
  ).signers([candidate1]).rpc()


  //   const [rankacc_key2] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("ranking_account"),
  //     Buffer.from([2]),
  //     trustee_election_key.toBuffer(),
  //     propertySystemPda.toBuffer()
  //   ],
  //   program.programId
  // ) 

  // let acc = await program.account.rankingAccount.fetch(rankacc_key2);

  // console.log(acc);

  const [candidate_recepit1] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      trustee_election_key.toBuffer(),
      candidate1.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc2 = await program.account.authorityCandidate.fetch(candidate_recepit1);

  console.log(acc2);

  const [candidate_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      trustee_election_key.toBuffer(),
      candidate3.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc = await program.account.authorityCandidate.fetch(candidate_recepit);

  console.log(acc);
  
  


})

it("skip time to 2 days ",async() =>{
  advanceClockBy(svm, 170800n);

})

it("finalize the old trustee",async()=>{

  const tx = await program.methods.finalizeOldTrsutee(
    new anchor.BN(1),
     new anchor.BN(1),
     pro1.publicKey
  ).accounts({
    signer:wallet.publicKey
  }).signers([wallet.payer]).rpc()


  const tx2 = await program.methods.finalizeOldTrsutee(
    new anchor.BN(1),
     new anchor.BN(1),
     pro2.publicKey
  ).accounts({
    signer:wallet.publicKey
  }).signers([wallet.payer]).rpc()

  //  const [resgination_key] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("trustee_resignation"),
  //     propertySystemPda.toBuffer(),
  //     pro1.publicKey.toBuffer()
  //   ],
  //   program.programId
  // ) 

  // const acc = await program.account.resignation.fetch(resgination_key);

  // console.log(acc);

})

it("finalize the new trsutee",async()=>{


  

  const tx = await program.methods.finalizeNewTrustee(
    candidate1.publicKey,
    new anchor.BN(1),
  new anchor.BN(1), 
  ).accounts(
    {
      signer:wallet.publicKey,
      candidate:candidate1.publicKey
    }
  ).signers([wallet.payer]).rpc()



  const tx2 = await program.methods.finalizeNewTrustee(
    candidate2.publicKey,
    new anchor.BN(1),
  new anchor.BN(1), 
  ).accounts(
    {
      signer:wallet.publicKey,
      candidate:candidate2.publicKey
    }
  ).signers([wallet.payer]).rpc()

  const acc = await program.account.electAuthority.fetch(trustee_election_key);

  console.log(acc);


  // const tx3 = await program.methods.finalizeNewTrustee(
  //   candidate3.publicKey,
  //   new anchor.BN(1),
  // new anchor.BN(1), 
  // ).accounts(
  //   {
  //     signer:wallet.publicKey,
  //     candidate:candidate3.publicKey
  //   }
  // ).signers([wallet.payer]).rpc()

})


const [election_proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("elect_arbitrar"),
      propertySystemPda.toBuffer(),
      new anchor.BN(1).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  ) 

//Arbitar resgination

it("arbitar resign proposal",async()=>{

for(let i =0;i<2;i++){

  
  const tx = await program.methods.arbitrarResign(
    new anchor.BN(1),
    new anchor.BN(1),
  ).accounts(
    {
      arbitrar:pro_vec[i].publicKey
    }
  ).signers([pro_vec[i]]).rpc();

}


  const [resgination_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("arbitrar_resignation"),
      propertySystemPda.toBuffer(),
      pro1.publicKey.toBuffer()
    ],
    program.programId
  ) 

  const acc = await program.account.resignation.fetch(resgination_key);

  console.log(acc);

   
  const acc2 = await program.account.electAuthority.fetch(election_proposal_key);

  console.log(acc2);

})

it("trustee vote to elect arbitar",async()=>{

  for (let i = 2;i<5;i++){

     const tx = await program.methods.trusteeApproveArbitrarElection(
    new anchor.BN(1),
    new anchor.BN(1),
  ).accounts({
    signer:pro_vec[i].publicKey
  }).signers([pro_vec[i]]).rpc()
  }

  //  const [election_proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("elect_arbitrar"),
  //     propertySystemPda.toBuffer(),
  //     new anchor.BN(1).toArrayLike(Buffer, "le", 8),
  //   ],
  //   program.programId
  // ) 
  // const acc2 = await program.account.electAuthority.fetch(election_proposal_key);

  // console.log(acc2);


  // const [election_vote_key] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("arbitrar_election_receipt"),
  //     election_proposal_key.toBuffer(),
  //     pro3.publicKey.toBuffer()
  //   ],
  //   program.programId
  // ) 

  // const acc3 = await program.account.voteReceiptForAuthorityElection.fetch(election_vote_key);

  // console.log(acc3);

})


it("submits snapshot for arbitrar election",async()=>{


   const proposalId = new anchor.BN(1);

  const [trustee_election_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("elect_arbitrar"),
      propertySystemPda.toBuffer(),
      proposalId.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const merkleRoot = buildMerkleRoot([
    buildAuthorityLeaf(receiver1.publicKey, trustee_election_key, governanceMint, 100,1),
    buildAuthorityLeaf(receiver2.publicKey, trustee_election_key, governanceMint, 100,1),
    buildAuthorityLeaf(receiver3.publicKey, trustee_election_key, governanceMint, 100,1),
    buildAuthorityLeaf(receiver4.publicKey, trustee_election_key, governanceMint, 100,1),
    buildAuthorityLeaf(receiver5.publicKey, trustee_election_key, governanceMint, 100,1),
    buildAuthorityLeaf(receiver6.publicKey, trustee_election_key, governanceMint, 100,1)
  ]);

 // console.log("pubkey", wallet.publicKey);
  const tx = await program.methods.submitSnapshotForArbitrarElection(
    proposalId,
    propertySystemPda,
    2,
    2,
    2,
    2,
    merkleRoot
  ).accounts(
    [wallet.publicKey]
  ).signers([wallet.payer]).rpc()


    const acc = await program.account.electAuthority.fetch(trustee_election_key);



    console.log(acc);



})


it("candidate profile submission for arbitrar",async()=>{


for(let i = 0; i<4 ;i++){
  const tx = await program.methods.submitArbitrarCandidate(
      new anchor.BN(1),
      new anchor.BN(1),
  ).accounts(
    {
      signer:candidate_vec[i].publicKey,
    }
  ).signers([candidate_vec[i]]).rpc();


}

  

  

  const [candidate_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      election_proposal_key.toBuffer(),
      candidate1.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc = await program.account.authorityCandidate.fetch(candidate_recepit);

  console.log(acc);

})

it("skip time to voting end",async() =>{
  advanceClockBy(svm, 180800n);

})


it("vote for candidate",async()=>{

  const snapshotEntries = [
    { voter: receiver1.publicKey, votingPower: 100,authoritytype:1 },
    { voter: receiver2.publicKey, votingPower: 100,authoritytype:1  },
    { voter: receiver3.publicKey, votingPower: 100,authoritytype:1  },
     { voter: receiver4.publicKey, votingPower: 100 ,authoritytype:1 },
      { voter: receiver5.publicKey, votingPower: 100,authoritytype:1  },
       { voter: receiver6.publicKey, votingPower: 100 ,authoritytype:1 },
  ];


  let proposalId = new anchor.BN(1);

 const [trustee_election_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("elect_arbitrar"),
      propertySystemPda.toBuffer(),
      proposalId.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );



  const elect_proposal = await program.account.electAuthority.fetch(trustee_election_key);

  // assert.isTrue(Number(svm.getClock().unixTimestamp) < sellProposal.endTime.toNumber());

 
 const receivers = [receiver1, receiver2, receiver3,receiver4,receiver5,receiver6];

  for(let i = 0; i < 6; i++){await connection.requestAirdrop(receivers[i].publicKey, 1e9)};


  // console.log(elect_proposal);
  

  for(let i =0 ;i<2;i++){


     const voter1proof = buildAuthorityProof(
    snapshotEntries,
    i,
    election_proposal_key,
    governanceMint
  );

const tx = await program.methods.voteForArbitrarCandiate(
    new anchor.BN(1),
    new anchor.BN(1),
    candidate1.publicKey,
    voter1proof,
    new anchor.BN(100),
  ).accounts(
    {
      signer:receivers[i].publicKey
    }
  ).signers([receivers[i]]).rpc();


  }

  for(let i =2 ;i<4;i++){
     const voter1proof = buildAuthorityProof(
    snapshotEntries,
    i,
    election_proposal_key,
    governanceMint
  );

const tx1 = await program.methods.voteForArbitrarCandiate(
    new anchor.BN(1),
    new anchor.BN(1),
    candidate2.publicKey,
    voter1proof,
    new anchor.BN(100),
  ).accounts(
    {
      signer:receivers[i].publicKey
    }
  ).signers([receivers[i]]).rpc();

  }

    
 const voter1proof = buildAuthorityProof(
    snapshotEntries,
    4,
    election_proposal_key,
    governanceMint
  );
const tx2 = await program.methods.voteForArbitrarCandiate(
    new anchor.BN(1),
    new anchor.BN(1),
    candidate3.publicKey,
    voter1proof,
    new anchor.BN(100),
  ).accounts(
    {
      signer:receiver5.publicKey
    }
  ).signers([receiver5]).rpc();


 const voter6proof = buildAuthorityProof(
    snapshotEntries,
    5,
    election_proposal_key,
    governanceMint
  );
  const tx3 = await program.methods.voteForArbitrarCandiate(
    new anchor.BN(1),
    new anchor.BN(1),
    candidate4.publicKey,
    voter6proof,
    new anchor.BN(100),
  ).accounts(
    {
      signer:receiver6.publicKey
    }
  ).signers([receiver6]).rpc();

  





 for(let i =0 ;i<4;i++){

   const [candidate_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      election_proposal_key.toBuffer(),
      candidate_vec[i].publicKey.toBuffer()
    ],
    program.programId
  )

  const acc = await program.account.authorityCandidate.fetch(candidate_recepit);

  console.log(acc);

 }

})

it("skip time to voting end",async() =>{
  advanceClockBy(svm, 180800n);

})


it("add new arbitrar",async()=>{
  

  const tx = await program.methods.addNewArbitrar(
    candidate2.publicKey,
    new anchor.BN(1),
    new anchor.BN(1),
    2
  ).accounts({
    signer:pro4.publicKey}
  ).signers([pro4]).rpc();

  

  // const [rankacc_key2] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("ranking_account"),
  //     Buffer.from([2]),
  //     election_proposal_key.toBuffer(),
  //     propertySystemPda.toBuffer()
  //   ],
  //   program.programId
  // ) 

  // let acc = await program.account.rankingAccount.fetch(rankacc_key2);

  // console.log(acc);


  const tx2 = await program.methods.addNewArbitrar(
    candidate3.publicKey,
    new anchor.BN(1),
    new anchor.BN(1),
    1
  ).accounts({
    signer:pro4.publicKey}
  ).signers([pro4]).rpc();

  const [rankacc_key3] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("ranking_account"),
      Buffer.from([1]),
      election_proposal_key.toBuffer(),
      propertySystemPda.toBuffer()
    ],
    program.programId
  ) 

  // let acc2 = await program.account.rankingAccount.fetch(rankacc_key3);

  // console.log(acc2);


  //   const [candidate_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("authority_candidate"),
  //     propertySystemPda.toBuffer(),
  //     trustee_election_key.toBuffer(),
  //     candidate2.publicKey.toBuffer()
  //   ],
  //   program.programId
  // )

  // const acc3 = await program.account.authorityCandidate.fetch(candidate_recepit);

  // console.log(acc3);

  //  const [candidate_recepit2] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("authority_candidate"),
  //     propertySystemPda.toBuffer(),
  //     trustee_election_key.toBuffer(),
  //     candidate3.publicKey.toBuffer()
  //   ],
  //   program.programId
  // )

  // const acc4 = await program.account.authorityCandidate.fetch(candidate_recepit2);

  // console.log(acc4);


})



it("adjust ranking for arbitrar",async()=>{

  const tx = await program.methods.adjustArbitrarRanks(
    new anchor.BN(1),
    propertySystemPda,
    candidate2.publicKey,
    candidate3.publicKey,
    2,1
  ).accounts(
    {signer:wallet.publicKey}
  ).signers([wallet.payer]).rpc();


    const [candidate_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      election_proposal_key.toBuffer(),
      candidate2.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc3 = await program.account.authorityCandidate.fetch(candidate_recepit);

  console.log(acc3);

   const [candidate_recepit2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      election_proposal_key.toBuffer(),
      candidate3.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc4 = await program.account.authorityCandidate.fetch(candidate_recepit2);

  console.log(acc4);
  


})

it("skip time to voting end",async() =>{
  advanceClockBy(svm, 180800n);

})

it("challenge the new arbitrar",async()=>{




  // const elect_proposal = await program.account.electAuthority.fetch(trustee_election_key);


  // console.log(elect_proposal);

    const [candidate_recepit2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      election_proposal_key.toBuffer(),
      candidate3.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc22 = await program.account.authorityCandidate.fetch(candidate_recepit2);

  console.log(acc22);
  

  const tx = await program.methods.challengeAgainstNewArbitrar(
    new anchor.BN(1),
    candidate1.publicKey,
    candidate3.publicKey,
    2,
    new anchor.BN(1),
  ).accounts(
    {signer: candidate1.publicKey}
  ).signers([candidate1]).rpc()


  //   const [rankacc_key2] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("ranking_account"),
  //     Buffer.from([2]),
  //     trustee_election_key.toBuffer(),
  //     propertySystemPda.toBuffer()
  //   ],
  //   program.programId
  // ) 

  // let acc = await program.account.rankingAccount.fetch(rankacc_key2);

  // console.log(acc);

  const [candidate_recepit1] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      election_proposal_key.toBuffer(),
      candidate1.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc2 = await program.account.authorityCandidate.fetch(candidate_recepit1);

  console.log(acc2);

  const [candidate_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      election_proposal_key.toBuffer(),
      candidate3.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc = await program.account.authorityCandidate.fetch(candidate_recepit);

  console.log(acc);
  
  


})



it("skip time to voting end",async() =>{
  advanceClockBy(svm, 180800n);

})


it("finalize the old arbitrar",async()=>{

  const tx = await program.methods.finalizeOldArbitrar(
    new anchor.BN(1),
     new anchor.BN(1),
     pro1.publicKey
  ).accounts({
    signer:wallet.publicKey
  }).signers([wallet.payer]).rpc()


  const tx2 = await program.methods.finalizeOldArbitrar(
    new anchor.BN(1),
     new anchor.BN(1),
     pro2.publicKey
  ).accounts({
    signer:wallet.publicKey
  }).signers([wallet.payer]).rpc()

  //  const [resgination_key] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("trustee_resignation"),
  //     propertySystemPda.toBuffer(),
  //     pro1.publicKey.toBuffer()
  //   ],
  //   program.programId
  // ) 

  // const acc = await program.account.resignation.fetch(resgination_key);

  // console.log(acc);

})



it("finalize the new arbitrar",async()=>{


  

  const tx = await program.methods.finalizeNewArbitrar(
    candidate1.publicKey,
    new anchor.BN(1),
  new anchor.BN(1), 
  ).accounts(
    {
      signer:wallet.publicKey,
      candidate:candidate1.publicKey
    }
  ).signers([wallet.payer]).rpc()



  const tx2 = await program.methods.finalizeNewArbitrar(
    candidate2.publicKey,
    new anchor.BN(1),
  new anchor.BN(1), 
  ).accounts(
    {
      signer:wallet.publicKey,
      candidate:candidate2.publicKey
    }
  ).signers([wallet.payer]).rpc()

  const acc = await program.account.electAuthority.fetch(election_proposal_key);

  console.log(acc);


  // const tx3 = await program.methods.finalizeNewTrustee(
  //   candidate3.publicKey,
  //   new anchor.BN(1),
  // new anchor.BN(1), 
  // ).accounts(
  //   {
  //     signer:wallet.publicKey,
  //     candidate:candidate3.publicKey
  //   }
  // ).signers([wallet.payer]).rpc()

})


it("challenge authority",async()=>{



     const proposalId = new anchor.BN(1);

  const [trustee_election_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("elect_arbitrar"),
      propertySystemPda.toBuffer(),
      proposalId.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const merkleRoot = buildMerkleRoot([
    buildAuthorityLeaf(receiver1.publicKey, trustee_election_key, governanceMint, 100,1),
    buildAuthorityLeaf(receiver2.publicKey, trustee_election_key, governanceMint, 100,1),
    buildAuthorityLeaf(receiver3.publicKey, trustee_election_key, governanceMint, 100,1),
    buildAuthorityLeaf(receiver4.publicKey, trustee_election_key, governanceMint, 100,1),
    buildAuthorityLeaf(receiver5.publicKey, trustee_election_key, governanceMint, 100,1),
    buildAuthorityLeaf(receiver6.publicKey, trustee_election_key, governanceMint, 100,1)
  ]);

    const tx = await program.methods.challengeAuthority(
    new anchor.BN(1),
    new anchor.BN(1),
    merkleRoot,
    merkleRoot,
  ).accounts({
    signer:receiver1.publicKey,
    tokenProgram:TOKEN_2022_PROGRAM_ID,
    mint:governanceMint
  }).signers([receiver1]).rpc()

//  const [proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("challenge_authority"),
//       propertySystemPda.toBuffer(),
//       new anchor.BN(1).toArrayLike(Buffer, "le", 8),

//     ],program.programId
//   )


//   const acc = await program.account.challengeProposal.fetch(proposal_key);


//   console.log(acc);
  




})


it("add  offender trustee and arbitrar",async()=>{

 const tx = await program.methods.addTrusteeOffender(
  new anchor.BN(1),
  new anchor.BN(1)
 ).accounts(
  {
    signer:receiver1.publicKey,
    trusteeOffender:candidate1.publicKey,
  }
 ).signers([receiver1]).rpc();


// const [key] = anchor.web3.PublicKey.findProgramAddressSync(
//   [
//     Buffer.from("arbitrator_recepit"),
//     propertySystemPda.toBuffer(),
//     candidate2.publicKey.toBuffer()
//   ],
//   program.programId
// )

// const acc = await program.account.arbitratorRecepit.fetch(key);

// console.log(acc);


 const tx2 = await program.methods.addArbitrarOffender(
  new anchor.BN(1),
  new anchor.BN(1)
 ).accounts(
  {
    signer:receiver1.publicKey,
    arbitrarOffender:candidate1.publicKey,
  }
 ).signers([receiver1]).rpc();


  // const [proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("challenge_authority"),
  //     propertySystemPda.toBuffer(),
  //     new anchor.BN(1).toArrayLike(Buffer, "le", 8),

  //   ],program.programId
  // )


  // const acc = await program.account.challengeProposal.fetch(proposal_key);


  // console.log(acc);


})

    const [proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("challenge_authority"),
      propertySystemPda.toBuffer(),
      new anchor.BN(1).toArrayLike(Buffer, "le", 8),

    ],program.programId
  )


it("submit snapshot for voting",async()=>{




  const merkleRoot = buildMerkleRoot([
    buildAuthorityLeaf(receiver1.publicKey, proposal_key, governanceMint, 100,5),
    buildAuthorityLeaf(receiver2.publicKey, proposal_key, governanceMint, 100,5),
    buildAuthorityLeaf(receiver3.publicKey, proposal_key, governanceMint, 100,5),
    buildAuthorityLeaf(receiver4.publicKey, proposal_key, governanceMint, 100,5),
    buildAuthorityLeaf(receiver5.publicKey, proposal_key, governanceMint, 100,5),
    buildAuthorityLeaf(receiver6.publicKey, proposal_key, governanceMint, 100,5)
  ]);

  const tx = await program.methods.submitSnaphotForVotingOnChallengeProposal(
    new anchor.BN(1),
    new anchor.BN(1),
    merkleRoot
  ).accounts(
    {
      signer:receiver1.publicKey
    }
  ).signers([receiver1]).rpc()

})


it("vote for challenge proposal",async()=>{


   const snapshotEntries = [
    { voter: receiver1.publicKey, votingPower: 100,authoritytype:5 },
    { voter: receiver2.publicKey, votingPower: 100,authoritytype:5  },
    { voter: receiver3.publicKey, votingPower: 100,authoritytype:5  },
     { voter: receiver4.publicKey, votingPower: 100 ,authoritytype:5 },
      { voter: receiver5.publicKey, votingPower: 100,authoritytype:5  },
       { voter: receiver6.publicKey, votingPower: 100 ,authoritytype:5 },
  ];

   const voter1proof = buildAuthorityProof(
    snapshotEntries,
    0,
    proposal_key,
    governanceMint
  );


  const tx = await program.methods.voteForChallengeProposal(
    new anchor.BN(1),
    new anchor.BN(1),
    voter1proof,
    new anchor.BN(100)
  ).accounts({
    signer:receiver1.publicKey
  }).signers([receiver1]).rpc()


  // const voter2proof = buildAuthorityProof(
  //   snapshotEntries,
  //   1,
  //   proposal_key,
  //   governanceMint
  // );


  // const tx2 = await program.methods.voteForChallengeProposal(
  //   new anchor.BN(1),
  //   new anchor.BN(1),
  //   voter2proof,
  //   new anchor.BN(100)
  // ).accounts({
  //   signer:receiver2.publicKey
  // }).signers([receiver2]).rpc()


  // const voter3proof = buildAuthorityProof(
  //   snapshotEntries,
  //   2,
  //   proposal_key,
  //   governanceMint
  // );


  // const tx3 = await program.methods.voteForChallengeProposal(
  //   new anchor.BN(1),
  //   new anchor.BN(1),
  //   voter3proof,
  //   new anchor.BN(100)
  // ).accounts({
  //   signer:receiver2.publicKey
  // }).signers([receiver2]).rpc()

// const acc = await program.account.challengeProposal.fetch(proposal_key);

// console.log(acc);



})


it("outcome of proposal",async()=>{

  const tx = await program.methods.outcomeOfProposal(
    new anchor.BN(1),
    new anchor.BN(1),
    { fraud: {} },
  ).accounts(
    wallet.publicKey
  ).signers([wallet.payer]).rpc()

  const acc = await program.account.challengeProposal.fetch(proposal_key);

console.log(acc);

})


it("finalize candidate profile for challenge propsal",async()=>{

  // const tx = await program.methods.finalizeCandidateProfileForChallengeProposal(
  //   new anchor.BN(1),
  //   new anchor.BN(1),
  //   candidate1.publicKey
  // ).accounts(
  //   {signer:wallet.publicKey}
  // ).signers([wallet.payer]).rpc()


  const [trustee_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("offender"),
      Buffer.from("trustee_recepit"),
      propertySystemPda.toBuffer(),
      proposal_key.toBuffer(),
      candidate1.publicKey.toBuffer()
    ],
    program.programId
  );

  const [arbitrar_recepit] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("offender"),
      Buffer.from("arbitrar_recepit"),
      propertySystemPda.toBuffer(),
      proposal_key.toBuffer(),
      candidate1.publicKey.toBuffer()
    ],
    program.programId
  );

  const acc2 = program.account.offenderReceipt.fetch(trustee_recepit);

  const tx = await program.methods.finalizeTrusteeCandidateProfileForChallengeProposal(
    new anchor.BN(1),
    new anchor.BN(1),
    candidate1.publicKey
  ).accounts(
    {
      signer:wallet.publicKey,

    }
  ).signers([wallet.payer]).rpc()

  const tx2 = await program.methods.finalizeArbitrarCandidateProfileForChallengeProposal(
    new anchor.BN(1),
    new anchor.BN(1),
    candidate1.publicKey
  ).accounts(
    {
      signer:wallet.publicKey,

    }
  ).signers([wallet.payer]).rpc()

  const [profile_key] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("candidate_profile"),
    candidate1.publicKey.toBuffer()
  ],
  program.programId
)

const acc = await program.account.candidateProfile.fetch(profile_key);

console.log(acc);

})



it("remove guilt trustee and arbitrar proposal",async()=>{


  const tx = await program.methods.removeTrusteeGuiltAuthorityProposal(
      new anchor.BN(1),
      new anchor.BN(1),
  ).accounts({
    signer:receiver1.publicKey,
    mint:governanceMint,
    tokenProgram:TOKEN_2022_PROGRAM_ID
  }).signers([receiver1]).rpc()



   await program.methods.removeArbitrarGuiltAuthorityProposal(
      new anchor.BN(1),
      new anchor.BN(1),
  ).accounts({
    signer:receiver1.publicKey,
    mint:governanceMint,
    tokenProgram:TOKEN_2022_PROGRAM_ID
  }).signers([receiver1]).rpc()


//   const [key] = anchor.web3.PublicKey.findProgramAddressSync(
//   [
//       Buffer.from("remove_trustee_proposal"),
//       propertySystemPda.toBuffer(),
//       proposal_key.toBuffer()
//   ],
//   program.programId
// )

// const acc = await program.account.electAuthority.fetch(key);

// console.log(acc);


})

it("add trustee and arbitrar to remove ",async()=>{

  const proposalId = new anchor.BN(1);
const propertySystemId = new anchor.BN(1);


  const tx = await program.methods.addTrusteeForRemoval(
    proposalId,
    propertySystemId,
    
  ).accounts(
    {
      signer:receiver1.publicKey,
      trustee:candidate1.publicKey
    }
  ).signers([receiver1]).rpc()


  const tx2 = await program.methods.addArbitrarForRemoval(
    proposalId,
    propertySystemId,

  ).accounts(
    {
      signer:receiver1.publicKey,
      arbitrar:candidate1.publicKey
    }
  ).signers([receiver1]).rpc()


})

// const [proposal_key] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("challenge_authority"),
//       propertySystemPda.toBuffer(),
//       new anchor.BN(1).toArrayLike(Buffer, "le", 8),

//     ],program.programId
//   )
   const [key] = anchor.web3.PublicKey.findProgramAddressSync(
  [
      Buffer.from("remove_trustee_proposal"),
      propertySystemPda.toBuffer(),
      proposal_key.toBuffer()
  ],
  program.programId
)

   const [key2] = anchor.web3.PublicKey.findProgramAddressSync(
  [
      Buffer.from("remove_arbitrar_proposal"),
      propertySystemPda.toBuffer(),
      proposal_key.toBuffer()
  ],
  program.programId
)


it("submit snapshot for removal proposal",async()=>{



  const merkleRoot1 = buildMerkleRoot([
    buildAuthorityLeaf(receiver1.publicKey, key, governanceMint, 100,0),
    buildAuthorityLeaf(receiver2.publicKey, key, governanceMint, 100,0),
    buildAuthorityLeaf(receiver3.publicKey, key, governanceMint, 100,0),
    buildAuthorityLeaf(receiver4.publicKey, key, governanceMint, 100,0),
    buildAuthorityLeaf(receiver5.publicKey, key, governanceMint, 100,0),
    buildAuthorityLeaf(receiver6.publicKey, key, governanceMint, 100,0)
  ]);

  const tx = await program.methods.submitSnapshotForRemovalProposal(
    proposal_key,
    new anchor.BN(1),
    merkleRoot1
  ).accounts({
    removalProposal:key
  },
  ).signers([wallet.payer]).rpc();

  const acc = await program.account.electAuthority.fetch(key);

  console.log(acc);


   const merkleRoot2 = buildMerkleRoot([
    buildAuthorityLeaf(receiver1.publicKey, key2, governanceMint, 100,1),
    buildAuthorityLeaf(receiver2.publicKey, key2, governanceMint, 100,1),
    buildAuthorityLeaf(receiver3.publicKey, key2, governanceMint, 100,1),
    buildAuthorityLeaf(receiver4.publicKey, key2, governanceMint, 100,1),
    buildAuthorityLeaf(receiver5.publicKey, key2, governanceMint, 100,1),
    buildAuthorityLeaf(receiver6.publicKey, key2, governanceMint, 100,1)
  ]);

    const tx2 = await program.methods.submitSnapshotForRemovalProposal(
    proposal_key,
    new anchor.BN(1),
    merkleRoot2
  ).accounts({
    removalProposal:key2
  },
  ).signers([wallet.payer]).rpc();

  const acc2 = await program.account.electAuthority.fetch(key);

  console.log(acc);
  

})


it("submit candidate for proposal",async()=>{


  const tx = await program.methods.submitCandidateForTrusteeAuthorityForRemoveProposal(
    proposal_key,
    new anchor.BN(1)
  ).accounts(
    {
      signer:candidate3.publicKey,
    
    }
  ).signers([candidate3]).rpc()


  const tx2 = await program.methods.submitCandidateForArbitrarAuthorityForRemoveProposal(
    proposal_key,
    new anchor.BN(1)
  ).accounts(
    {
      signer:candidate3.publicKey,
    
    }
  ).signers([candidate3]).rpc()


})


it("skip time to voting end",async() =>{
  advanceClockBy(svm, 302400n);

})




it("vote for new authority for in removal proposal",async()=>{


  // const acc = await program.account.electAuthority.fetch(key);

  // console.log(acc);
  

  const snapshotEntries = [
    { voter: receiver1.publicKey, votingPower: 100,authoritytype:0 },
    { voter: receiver2.publicKey, votingPower: 100,authoritytype:0  },
    { voter: receiver3.publicKey, votingPower: 100,authoritytype:0  },
     { voter: receiver4.publicKey, votingPower: 100 ,authoritytype:0 },
      { voter: receiver5.publicKey, votingPower: 100,authoritytype:0  },
       { voter: receiver6.publicKey, votingPower: 100 ,authoritytype:0 },
  ];

   const voter1proof = buildAuthorityProof(
    snapshotEntries,
    0,
    key,
    governanceMint
  );
  const voter2proof = buildAuthorityProof(
    snapshotEntries,
    1,
    key,
    governanceMint
  );

   const voter3proof = buildAuthorityProof(
    snapshotEntries,
    2,
    key,
    governanceMint
  );
   const voter4proof = buildAuthorityProof(
    snapshotEntries,
    3,
    key,
    governanceMint
  );
   const voter5proof = buildAuthorityProof(
    snapshotEntries,
    4,
    key,
    governanceMint
  );



  const tx = await program.methods.voteForNewTrusteeAuthorityForRemovalProposal(
    proposal_key,
    new anchor.BN(1),
    candidate3.publicKey,
    voter1proof,
    new anchor.BN(100)
  ).accounts(
    {
      signer:receiver1.publicKey
    }
  ).signers([receiver1]).rpc()

  const tx2 = await program.methods.voteForNewTrusteeAuthorityForRemovalProposal(
    proposal_key,
    new anchor.BN(1),
    candidate3.publicKey,
    voter2proof,
    new anchor.BN(100)
  ).accounts(
    {
      signer:receiver2.publicKey
    }
  ).signers([receiver2]).rpc()

   await program.methods.voteForNewTrusteeAuthorityForRemovalProposal(
    proposal_key,
    new anchor.BN(1),
    candidate3.publicKey,
    voter3proof,
    new anchor.BN(100)
  ).accounts(
    {
      signer:receiver3.publicKey
    }
  ).signers([receiver3]).rpc()


 await program.methods.voteForNewTrusteeAuthorityForRemovalProposal(
    proposal_key,
    new anchor.BN(1),
    candidate3.publicKey,
    voter4proof,
    new anchor.BN(100)
  ).accounts(
    {
      signer:receiver4.publicKey
    }
  ).signers([receiver4]).rpc()



  //////////////////////////////////////////


  const snapshotEntries2 = [
    { voter: receiver1.publicKey, votingPower: 100,authoritytype:1 },
    { voter: receiver2.publicKey, votingPower: 100,authoritytype:1  },
    { voter: receiver3.publicKey, votingPower: 100,authoritytype:1  },
     { voter: receiver4.publicKey, votingPower: 100 ,authoritytype:1 },
      { voter: receiver5.publicKey, votingPower: 100,authoritytype:1  },
       { voter: receiver6.publicKey, votingPower: 100 ,authoritytype:1 },
  ];

   const voterAproof = buildAuthorityProof(
    snapshotEntries2,
    0,
    key2,
    governanceMint
  );
  const voterBproof = buildAuthorityProof(
    snapshotEntries2,
    1,
    key2,
    governanceMint
  );

  const voterCproof = buildAuthorityProof(
    snapshotEntries2,
    2,
    key2,
    governanceMint
  );
  const voterDproof = buildAuthorityProof(
    snapshotEntries2,
    3,
    key2,
    governanceMint
  );



    await program.methods.voteForNewArbitrarAuthorityForRemovalProposal(
    proposal_key,
    new anchor.BN(1),
    candidate3.publicKey,
    voterAproof,
    new anchor.BN(100)
  ).accounts(
    {
      signer:receiver1.publicKey
    }
  ).signers([receiver1]).rpc()

   await program.methods.voteForNewArbitrarAuthorityForRemovalProposal(
    proposal_key,
    new anchor.BN(1),
    candidate3.publicKey,
    voterBproof,
    new anchor.BN(100)
  ).accounts(
    {
      signer:receiver2.publicKey
    }
  ).signers([receiver2]).rpc()

   await program.methods.voteForNewArbitrarAuthorityForRemovalProposal(
    proposal_key,
    new anchor.BN(1),
    candidate3.publicKey,
    voterCproof,
    new anchor.BN(100)
  ).accounts(
    {
      signer:receiver3.publicKey
    }
  ).signers([receiver3]).rpc()

   await program.methods.voteForNewArbitrarAuthorityForRemovalProposal(
    proposal_key,
    new anchor.BN(1),
    candidate3.publicKey,
    voterDproof,
    new anchor.BN(100)
  ).accounts(
    {
      signer:receiver4.publicKey
    }
  ).signers([receiver4]).rpc()









})


it("skip time to voting end",async() =>{
  advanceClockBy(svm, 302400n);

})


it("finalize remove proposal",async()=>{


  await program.methods.finalizeRemoveProposal().accounts(
    {
      signer:receiver5.publicKey,
      removeProposal:key
    }
  ).signers([receiver5]).rpc()


  await program.methods.finalizeRemoveProposal().accounts(
    {
      signer:receiver5.publicKey,
      removeProposal:key2
    }
  ).signers([receiver5]).rpc()

  const acc = await program.account.electAuthority.fetch(key2);

  console.log(acc);
  

})




it("add new authority for removal proposal",async()=>{

  const tx = await program.methods.addNewAuthorityForTrusteeRemoveProposal(
    proposal_key,
    candidate3.publicKey,
    new anchor.BN(1),
    new anchor.BN(1),
    1
  ).accounts(
    {signer:wallet.publicKey}
  ).signers([wallet.payer]).rpc();




   await program.methods.addNewAuthorityForArbitrarRemoveProposal(
    proposal_key,
    candidate3.publicKey,
    new anchor.BN(1),
    new anchor.BN(1),
    1
  ).accounts(
    {signer:wallet.publicKey}
  ).signers([wallet.payer]).rpc();



  const [candidate_auth_key] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority_candidate"),
      propertySystemPda.toBuffer(),
      key.toBuffer(),
      candidate3.publicKey.toBuffer()
    ],
    program.programId
  )

  const acc = await program.account.authorityCandidate.fetch(candidate_auth_key);

  console.log(acc);
  



})

it("skip time to voting end",async() =>{
  advanceClockBy(svm, 302400n);

})
it("skip time to voting end",async() =>{
  advanceClockBy(svm, 302400n);

})

it("remove old trustee and arbitrar",async()=>{
  

  


  const tx = await program.methods.removeOldTrusteeRemoveProposal(
    new anchor.BN(1),
    proposal_key,
    new anchor.BN(1),
    candidate1.publicKey
  ).accounts(
    {
      signer:receiver1.publicKey
    }
  ).signers([receiver1]).rpc()

  const tx2 = await program.methods.removeOldArbitrarRemoveProposal(
    new anchor.BN(1),
    proposal_key,
    new anchor.BN(1),
    candidate1.publicKey
  ).accounts(
    {
      signer:receiver1.publicKey
    }
  ).signers([receiver1]).rpc()




//   const [trustee_registry] = PublicKey.findProgramAddressSync(
//   [
//     Buffer.from("trustee_registry"),
//     propertySystemPda.toBuffer(),
//   ],
//   program.programId
// );

// const trustee_registry_pda = await program.account.trusteeRegistry.fetch(trustee_registry);


// console.log(trustee_registry_pda);


})

it("finalize new trustee for remove proposal",async()=>{



  await program.methods.finalizeNewTrusteeForRemoveProposal(
    candidate3.publicKey,
    new anchor.BN(1),
    new anchor.BN(1),
    proposal_key
  ).accounts(
    {signer:wallet.publicKey,
     candidate:candidate3.publicKey
    }
  ).signers([wallet.payer]).rpc()



  await program.methods.finalizeNewArbitrarForRemoveProposal(
    candidate3.publicKey,
    new anchor.BN(1),
    new anchor.BN(1),
    proposal_key
  ).accounts(
    {signer:wallet.publicKey,
     candidate:candidate3.publicKey
    }
  ).signers([wallet.payer]).rpc()


//     const [key] = anchor.web3.PublicKey.findProgramAddressSync(
//   [
//       Buffer.from("remove_trustee_authority"),
//       propertySystemPda.toBuffer(),
//       proposal_key.toBuffer()
//   ],
//   program.programId
// )

const acc = await program.account.electAuthority.fetch(key2);

console.log(acc);





  //  const [candidate_auth_key] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("authority_candidate"),
  //     propertySystemPda.toBuffer(),
  //     key.toBuffer(),
  //     candidate3.publicKey.toBuffer()
  //   ],
  //   program.programId
  // )

  // const acc = await program.account.authorityCandidate.fetch(candidate_auth_key);

  // console.log(acc);
  //  const [candidate_auth_key] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("authority_candidate"),
  //     propertySystemPda.toBuffer(),
  //     key.toBuffer(),
  //     candidate3.publicKey.toBuffer()
  //   ],
  //   program.programId
  // )

  // const acc = await program.account.authorityCandidate.fetch(candidate_auth_key);


})



  });
