use anchor_lang::prelude::*;
use std::{
  result::Result as StdResult
};

#[error]
pub enum ErrorCode {
  #[msg("overflow")]
  Overflow,
  #[msg("underflow")]
  Underflow,
  #[msg("division by zero")]
  DivisionByZero,
}

// Third alternative solution is to extend the types with the custom trait
pub trait SafeMath {
  type Output;

  fn safe_add(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError>;
  fn safe_sub(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError>;
  fn safe_div(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError>;
  fn safe_mul(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError>;
  fn safe_pow(&self, exp: u32) -> StdResult<Self::Output, ProgramError>;
}

macro_rules! safe_math {
  ($type: ident) => {
    impl SafeMath for $type {
      type Output = $type;

      fn safe_add(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError> {
        match self.checked_add(rhs) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::Overflow.into())
        }
      }
    
      fn safe_sub(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError> {
        match self.checked_sub(rhs) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::Underflow.into())
        }
      }

      fn safe_mul(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError> {
        match self.checked_mul(rhs) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::Underflow.into())
        }
      }

      fn safe_div(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError> {
        match self.checked_div(rhs) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::DivisionByZero.into())
        }
      }

      fn safe_pow(&self, exp: u32) -> StdResult<Self::Output, ProgramError> {
        match self.checked_pow(exp) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::Overflow.into())
        }
      }
    }
  }
}

safe_math!(u128);
safe_math!(u64);
safe_math!(u32);
safe_math!(u16);
safe_math!(u8);
