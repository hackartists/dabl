use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub id: String,

    // signer is code of a sign algorithm using private keys.
    pub signer: String,

    // verifier is code of a verify algorithm using public keys.
    pub verifier: String,

    // public_keys is a list of public keys.
    pub public_keys: Vec<u64>,

    // private_keys is a list of private keys.
    pub private_keys: Vec<u64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicAccount {
    pub id: String,

    // signer is code of a sign algorithm using private keys.
    pub signer: String,

    // verifier is code of a verify algorithm using public keys.
    pub verifier: String,

    // public_keys is a list of public keys.
    pub public_keys: Vec<u64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AttackAccount {
    pub id: String,

    // attacker is an account ID of attacker who is trying to take private keys.
    pub attacker: String,

    // target is an account ID of target.
    pub target: String,

    pub private_keys: Vec<u64>,
}
