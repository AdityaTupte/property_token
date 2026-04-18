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
  getAccount,
  approve
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
  for(let i = 0; i < 5; i++){
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
  for(let i =0;i<7;i++){

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
   for(let i= 0;i<4;i++){

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
  for(let i =0;i<6;i++){

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

  for(let i = 0; i < 6; i++){await connection.requestAirdrop(state_auth[i].publicKey, 1e9); // 1 SOL

  await new Promise(resolve => setTimeout(resolve, 100));}

  const property_proposal = await program.methods.createPropertyProposal(
    country,
    sta,
    new anchor.BN(1),
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

  

  const [property_proposal_key] =  anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("property_proposal"),
      propertyId.toArrayLike(Buffer, "le", 8),
      state_pda_key.toBuffer()
    ],
    program.programId
  ) ;

  const acc = await program.account.propertyProposal.fetch(property_proposal_key);

  console.log(acc);

  
  
  });


  it("execute property",async()=>{

    const tx = await program.methods.executePropertyPropsal(
      country,
      sta,
      new anchor.BN(1),
      new anchor.BN(1),
       propertySystemPda
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


const acc = await program.account.propertyAccount.fetch(property_key);

console.log(acc);



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

  


});
