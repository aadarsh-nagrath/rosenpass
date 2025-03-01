//! Implementation of the tree-like structure used for the label derivation in [labeled_prf](crate::labeled_prf)
use crate::coloring::Secret;

use anyhow::Result;
use rosenpass_ciphers::{hash, KEY_LEN};
use rosenpass_to::To;

// TODO Use a proper Dec interface
#[derive(Clone, Debug)]
pub struct PrfTree([u8; KEY_LEN]);
#[derive(Clone, Debug)]
pub struct PrfTreeBranch([u8; KEY_LEN]);
#[derive(Clone, Debug)]
pub struct SecretPrfTree(Secret<KEY_LEN>);
#[derive(Clone, Debug)]
pub struct SecretPrfTreeBranch(Secret<KEY_LEN>);

impl PrfTree {
    pub fn zero() -> Self {
        Self([0u8; KEY_LEN])
    }

    pub fn dup(self) -> PrfTreeBranch {
        PrfTreeBranch(self.0)
    }

    pub fn into_secret_prf_tree(self) -> SecretPrfTree {
        SecretPrfTree(Secret::from_slice(&self.0))
    }

    // TODO: Protocol! Use domain separation to ensure that
    pub fn mix(self, v: &[u8]) -> Result<Self> {
        Ok(Self(hash::hash(&self.0, v).collect::<[u8; KEY_LEN]>()?))
    }

    pub fn mix_secret<const N: usize>(self, v: Secret<N>) -> Result<SecretPrfTree> {
        SecretPrfTree::prf_invoc(&self.0, v.secret())
    }

    pub fn into_value(self) -> [u8; KEY_LEN] {
        self.0
    }
}

impl PrfTreeBranch {
    pub fn mix(&self, v: &[u8]) -> Result<PrfTree> {
        Ok(PrfTree(hash::hash(&self.0, v).collect::<[u8; KEY_LEN]>()?))
    }

    pub fn mix_secret<const N: usize>(&self, v: Secret<N>) -> Result<SecretPrfTree> {
        SecretPrfTree::prf_invoc(&self.0, v.secret())
    }
}

impl SecretPrfTree {
    pub fn prf_invoc(k: &[u8], d: &[u8]) -> Result<SecretPrfTree> {
        let mut r = SecretPrfTree(Secret::zero());
        hash::hash(k, d).to(r.0.secret_mut())?;
        Ok(r)
    }

    pub fn zero() -> Self {
        Self(Secret::zero())
    }

    pub fn dup(self) -> SecretPrfTreeBranch {
        SecretPrfTreeBranch(self.0)
    }

    pub fn danger_from_secret(k: Secret<KEY_LEN>) -> Self {
        Self(k)
    }

    pub fn mix(self, v: &[u8]) -> Result<SecretPrfTree> {
        Self::prf_invoc(self.0.secret(), v)
    }

    pub fn mix_secret<const N: usize>(self, v: Secret<N>) -> Result<SecretPrfTree> {
        Self::prf_invoc(self.0.secret(), v.secret())
    }

    pub fn into_secret(self) -> Secret<KEY_LEN> {
        self.0
    }

    pub fn into_secret_slice(mut self, v: &[u8], dst: &[u8]) -> Result<()> {
        hash::hash(v, dst).to(self.0.secret_mut())
    }
}

impl SecretPrfTreeBranch {
    pub fn mix(&self, v: &[u8]) -> Result<SecretPrfTree> {
        SecretPrfTree::prf_invoc(self.0.secret(), v)
    }

    pub fn mix_secret<const N: usize>(&self, v: Secret<N>) -> Result<SecretPrfTree> {
        SecretPrfTree::prf_invoc(self.0.secret(), v.secret())
    }

    // TODO: This entire API is not very nice; we need this for biscuits, but
    // it might be better to extract a special "biscuit"
    // labeled subkey and reinitialize the chain with this
    pub fn danger_into_secret(self) -> Secret<KEY_LEN> {
        self.0
    }
}
