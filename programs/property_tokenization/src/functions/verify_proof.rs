use anchor_spl::associated_token::spl_associated_token_account::solana_program::keccak;



pub fn verify_proof(
    leaf: [u8; 32],
    proof: &Vec<[u8; 32]>,
    root: [u8; 32],
) -> bool {
    
    let mut computed = leaf;

    for p in proof.iter() {
        if computed <= *p {
            computed = keccak::hashv(&[&computed, p]).0;
        } else {
            computed = keccak::hashv(&[p, &computed]).0;
        }
    }

    computed == root
}



