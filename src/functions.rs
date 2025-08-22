#[derive(Clone,Copy,PartialEq)]
pub(crate)enum Unary{
  GNU, // GNU factor format  e.g 60: 2 2 3 5
  GNUNoRepeat, // GNU without repeating  2 2 3 5 
  Factor, // Correct mathematical representation 60: 2^2 * 3 * 5
  FactorNoRepeat, // Returns factorization 2^2 * 3 * 5 
  Prime, // Returns Primality evaluation
  PrimeNoRepeat, // Returns Primality evaluation without repeating
  Sigma,
  SigmaNoRepeat,
  Euler,
  EulerNoRepeat,
  Exp,
  ExpNoRepeat,
  Omega,
  OmegaNoRepeat,
  OmegaTwo,
  OmegaTwoNoRepeat,
  Cyclic,
  CyclicNoRepeat,
  Mobius,
  MobiusNoRepeat,
  Liar,
  LiarNoRepeat,
  StrongLiar,
  StrongLiarNoRepeat,
  Liouville,
  LiouvilleNoRepeat,
  PrimeFilter,
  CyclicFilter,
}

// Binary input
#[derive(Copy,Clone,PartialEq)]
pub(crate)enum Binary{
  Ord,
  OrdNoRepeat,
  OrdSwap,
  OrdSwapNoRepeat,
  StrongFermat,
  StrongFermatNoRepeat,
  StrongFermatFilter,
  Fermat,
  FermatNoRepeat,
  FermatFilter,
  Kronecker,
  KroneckerNoRepeat,
  Gcd,
  GcdNoRepeat,
  Coprime,
  CoprimeFilter,
  CoprimeNoRepeat,
  MulInverse,
  MulInverseNoRepeat,
  MulInverseSwap,
  MulInverseSwapNoRepeat,
  Lcm,
  LcmNoRepeat,
}

// Quadratic residue detector legendre() or modular sqrt algorithm
// Nonquadratic residue detector j(a,b) = -1
#[derive(Clone,Copy,PartialEq)]
pub(crate)enum Function{
  ArityOne(Unary),
  ArityTwo(Binary),
}

