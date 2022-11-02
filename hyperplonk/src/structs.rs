//! Main module for the HyperPlonk PolyIOP.

use crate::{custom_gate::CustomizedGates, selectors::SelectorColumn};
use ark_ec::PairingEngine;
use ark_ff::PrimeField;
use ark_poly::DenseMultilinearExtension;
use ark_std::log2;
use std::rc::Rc;
use subroutines::{
    pcs::PolynomialCommitmentScheme,
    poly_iop::prelude::{PermutationCheck, ZeroCheck},
};

/// The proof for the HyperPlonk PolyIOP, consists of the following:
///   - a batch commitment to all the witness MLEs
///   - a batch opening to all the MLEs at certain index
///   - the zero-check proof for checking custom gate-satisfiability
///   - the permutation-check proof for checking the copy constraints
#[derive(Clone, Debug, PartialEq)]
pub struct HyperPlonkProof<E, PC, PCS>
where
    E: PairingEngine,
    PC: PermutationCheck<E, PCS>,
    PCS: PolynomialCommitmentScheme<E>,
{
    // PCS commit for witnesses
    pub witness_merged_commit: PCS::Commitment,
    pub witness_commits: Vec<PCS::Commitment>,
    pub batch_prod_x_openings: PCS::BatchProof,
    pub batch_witness_and_selector_openings: PCS::BatchProof,
    pub perm_check_opening: PCS::Proof,
    pub perm_check_eval: PCS::Evaluation,
    // =======================================================================
    // IOP proofs
    // =======================================================================
    // the custom gate zerocheck proof
    pub zero_check_proof: <PC as ZeroCheck<E::Fr>>::ZeroCheckProof,
    // the permutation check proof for copy constraints
    pub perm_check_proof: PC::PermutationProof,
}

/// The HyperPlonk instance parameters, consists of the following:
///   - the number of constraints
///   - number of public input columns
///   - the customized gate function
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HyperPlonkParams {
    /// the number of constraints
    pub num_constraints: usize,
    /// number of public input
    // public input is only 1 column and is implicitly the first witness column.
    // this size must not exceed number of constraints.
    pub num_pub_input: usize,
    /// customized gate function
    pub gate_func: CustomizedGates,
}

impl HyperPlonkParams {
    /// Number of variables in a multilinear system
    pub fn num_variables(&self) -> usize {
        log2(self.num_constraints) as usize
    }

    /// number of selector columns
    pub fn num_selector_columns(&self) -> usize {
        self.gate_func.num_selector_columns()
    }

    /// number of witness columns
    pub fn num_witness_columns(&self) -> usize {
        self.gate_func.num_witness_columns()
    }
}

/// The HyperPlonk index, consists of the following:
///   - HyperPlonk parameters
///   - the wire permutation
///   - the selector vectors
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HyperPlonkIndex<F: PrimeField> {
    pub params: HyperPlonkParams,
    pub permutation: Vec<F>,
    pub selectors: Vec<SelectorColumn<F>>,
}

impl<F: PrimeField> HyperPlonkIndex<F> {
    /// Number of variables in a multilinear system
    pub fn num_variables(&self) -> usize {
        self.params.num_variables()
    }

    /// number of selector columns
    pub fn num_selector_columns(&self) -> usize {
        self.params.num_selector_columns()
    }

    /// number of witness columns
    pub fn num_witness_columns(&self) -> usize {
        self.params.num_witness_columns()
    }
}

/// The HyperPlonk proving key, consists of the following:
///   - the hyperplonk instance parameters
///   - the preprocessed polynomials output by the indexer
///   - the commitment to the selectors
///   - the parameters for polynomial commitment
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HyperPlonkProvingKey<E: PairingEngine, PCS: PolynomialCommitmentScheme<E>> {
    /// Hyperplonk instance parameters
    pub params: HyperPlonkParams,
    /// The preprocessed permutation polynomials
    pub permutation_oracle: Rc<DenseMultilinearExtension<E::Fr>>,
    /// The preprocessed selector polynomials
    pub selector_oracles: Vec<Rc<DenseMultilinearExtension<E::Fr>>>,
    /// A commitment to the preprocessed selector polynomials
    pub selector_commitments: Vec<PCS::Commitment>,
    /// The parameters for PCS commitment
    pub pcs_param: PCS::ProverParam,
}

/// The HyperPlonk verifying key, consists of the following:
///   - the hyperplonk instance parameters
///   - the commitments to the preprocessed polynomials output by the indexer
///   - the parameters for polynomial commitment
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HyperPlonkVerifyingKey<E: PairingEngine, PCS: PolynomialCommitmentScheme<E>> {
    /// Hyperplonk instance parameters
    pub params: HyperPlonkParams,
    /// The preprocessed permutation polynomials
    pub permutation_oracle: Rc<DenseMultilinearExtension<E::Fr>>,
    /// The parameters for PCS commitment
    pub pcs_param: PCS::VerifierParam,
    /// A commitment to the preprocessed selector polynomials
    pub selector_commitments: Vec<PCS::Commitment>,
    /// Permutation oracle's commitment
    pub perm_com: PCS::Commitment,
}