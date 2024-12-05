use std::time::SystemTime;

use anyhow::{anyhow, Result};
use blake3::Hash;
use ed25519_dalek::{
    ed25519::signature::Keypair, Digest, DigestSigner, Sha512, Signature, SigningKey, VerifyingKey,
};
use rand::{rngs::ThreadRng, CryptoRng, RngCore};
use serde::{Deserialize, Serialize};

use crate::domains::{Domain, Record};

#[derive(Serialize, Deserialize)]
pub enum Transaction {
    Declaration {
        domain: Domain,
        verifying_key: VerifyingKey,
        updates_sum: u32,
    },
    Update {
        declaration: Hash,
        records: Vec<Record>,
        signature: Signature,
    },
}

impl Transaction {
    pub fn declare(domain: Domain, rng: &mut ThreadRng) -> (Self, SigningKey) {
        let signing_key = SigningKey::generate(rng);
        (
            Self::Declaration {
                domain,
                verifying_key: signing_key.verifying_key(),
                updates_sum: 0,
            },
            signing_key,
        )
    }

    pub fn update(
        &mut self,
        declaration_hash: Hash,
        records: Vec<Record>,
        signing_key: SigningKey,
    ) -> Result<Self> {
        match self {
            Transaction::Declaration {
                updates_sum,
                domain: _,
                verifying_key: _,
            } => {
                let mut digest = Sha512::new();
                digest.update(updates_sum.to_le_bytes());
                bincode::serialize_into(&mut digest, &records)?;

                let signature = signing_key
                    .with_context(declaration_hash.as_bytes())?
                    .sign_digest(digest);

                Ok(Self::Update {
                    declaration: declaration_hash,
                    records,
                    signature,
                })
            }
            Transaction::Update { .. } => Err(anyhow!("Unexpected variant")),
        }
    }
}
