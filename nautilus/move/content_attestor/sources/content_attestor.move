
module content_attestor::content_attestor;

use enclave::enclave::{Self, Enclave};
use std::string::String;

const PROOF_INTENT: u8 = 42;
const EInvalidSignature: u64 = 1;

// NFT that proves a specific content hash was verified at a time,
public struct ProofNFT has key, store {
    id: UID,
    content_hash: String,
    model: String,
    prompt: String,
    timestamp_ms: u64,
}

public struct ProofResponse has copy, drop {
    content_hash: String,
}

public struct CONTENT_ATTESTOR has drop {}

fun init(otw: CONTENT_ATTESTOR, ctx: &mut TxContext) {
    let cap = enclave::new_cap(otw, ctx);

    cap.create_enclave_config(
        b"proofspace enclave".to_string(),
        x"000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", // pcr0
        x"000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", // pcr1
        x"000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", // pcr2
        ctx,
    );

    transfer::public_transfer(cap, ctx.sender())
}

#[allow(unused_type_parameter)]
public fun update_proof<T>(
    content_hash: String, 
    timestamp_ms: u64,
    sig: &vector<u8>,
    model: String,
    prompt: String,
    enclave: &Enclave<T>,
    ctx: &mut TxContext,
): ProofNFT {
    let ok = enclave.verify_signature(
        PROOF_INTENT,
        timestamp_ms, 
        ProofResponse { content_hash },
        sig,
    );
    assert!(ok, EInvalidSignature);

    ProofNFT {
        id: object::new(ctx),
        content_hash,
        model,
        prompt,
        timestamp_ms,
    }
}

public fun transfer_nft(nft: ProofNFT, recipient: address) {
    transfer::transfer(nft, recipient);
}

public fun nft_hash(nft: &ProofNFT): &String { &nft.content_hash }

public fun nft_model(nft: &ProofNFT): &String { &nft.model }

public fun nft_prompt(nft: &ProofNFT): &String { &nft.prompt }

public fun nft_timestamp(nft: &ProofNFT): u64 { nft.timestamp_ms}