use jubjub::curve::{JubjubEngine, edwards::Point, PrimeOrder, FixedGenerators, JubjubParams};
use pairing::io;
use merlin::Transcript;
use crate::transcript::*;
use crate::commitment::Commitment;

pub struct Cosigners<E: JubjubEngine> {
    pos: usize,
    pub_key: Point<E, PrimeOrder>,
}

impl<E: JubjubEngine> Cosigners<E> {
    pub fn new(pos: usize, pub_key: Point<E, PrimeOrder>) -> Self {
        Cosigners {
            pos,
            pub_key,
        }
    }

    pub fn commit(self, commitment: Commitment) -> CosignersCommited<E> {
        CosignersCommited {
            pos: self.pos,
            pub_key: self.pub_key,
            commitment,
        }
    }
}

pub struct CosignersCommited<E: JubjubEngine> {
    pos: usize,
    pub_key: Point<E, PrimeOrder>,
    commitment: Commitment,
}

impl<E: JubjubEngine> CosignersCommited<E> {
    pub fn verify_witness(self, R: &Point<E, PrimeOrder>) -> io::Result<CosignersRevealed<E>> {
        let received_comm = Commitment::from_R(R)?;
        let eq = self.commitment.ct_eq(&received_comm);

        if !eq {
            return Err(io::Error::InvalidData)
        }

        Ok(CosignersRevealed {
            pos: self.pos,
            pub_key: self.pub_key,
            reveal: *R,
        })
    }
}

pub struct CosignersRevealed<E: JubjubEngine> {
    pos: usize,
    pub_key: Point<E, PrimeOrder>,
    reveal: Point<E, PrimeOrder>,
}

impl<E: JubjubEngine> CosignersRevealed<E> {
    pub fn verify_share(self, share: E::Fs, transcript: &Transcript) -> io::Result<E::Fs> {
        unimplemented!();
    }
}