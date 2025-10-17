#[derive(Clone,Copy,PartialEq)]
pub(crate)enum Unary{
  GNU, // GNU factor format  e.g 60: 2 2 3 5
  GNUNoRepeat, // GNU without repeating  2 2 3 5 
  Factor, // Correct mathematical representation 60: 2^2 * 3 * 5
  FactorNoRepeat, // Returns factorization 2^2 * 3 * 5
  MaxFactor, // Returns the Maximum factor
  MaxFactorNoRepeat,
  Prime, // Returns Primality evaluation
  PrimeNoRepeat, // Returns Primality evaluation without repeating
  Composite,
  CompositeNoRepeat,
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
  CompositeFilter,
  CyclicFilter,
  UnitRatio,
  UnitRatioNoRepeat,
  UnitRatiod,
  UnitRatiodNoRepeat,
  FrobeniusIdx,
  FrobeniusIdxNoRepeat,
}

// Binary input
#[derive(Copy,Clone,PartialEq)]
pub(crate)enum Binary{
  Residue,
  ResidueNoRepeat,
  ResidueSwap,
  ResidueSwapNoRepeat,
  Ord,
  OrdNoRepeat,
  OrdSwap,
  OrdSwapNoRepeat,
  StrongFermat,
  StrongFermatSwap,
  StrongFermatNoRepeat,
  StrongFermatSwapNoRepeat,
  StrongFermatFilter,
  Fermat,
  FermatSwap,
  FermatSwapNoRepeat,
  FermatNoRepeat,
  FermatFilter,
  Kronecker,
  KroneckerNoRepeat,
  KroneckerSwap,
  KroneckerSwapNoRepeat,
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



#[derive(Clone,Copy,PartialEq)]
pub(crate)enum Function{
  ArityOne(Unary),
  ArityTwo(Binary),
}

