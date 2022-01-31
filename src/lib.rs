//! # Anchor Safe Math
//!
//! `anchor_safe_math` is a collection of helper numeric operation functions that removes the 
//! verbosity of checking for overflow, underflow and division by zero errors.
//! 
//! # Examples
//!
//! ```
//! use anchor_lang::prelude::;
//! use safe_math::{SafeMath};
//! 
//! #[program]
//! pub mod example {
//!   use super::*;
//! 
//!   pub fn instruction(ctx: Context<Instruction>, amount: u64) -> ProgramResult {
//!     let state = &mut ctx.accounts.state;
//! 
//!     // You can apply any of the following operations
//!     state.total_amount = state.total_amount.safe_add(amount)?;
//!     state.total_amount = state.total_amount.safe_sub(amount)?;
//!     state.total_amount = state.total_amount.safe_mul(amount)?;
//!     state.total_amount = state.total_amount.safe_div(amount)?;
//!     state.total_amount = state.total_amount.safe_pow(8_u32)?;
//!   }
//! }
//! 
//! #[derive(Accounts)]
//! pub struct Instruction<'info> {
//!   ...
//! }
//! ```
use anchor_lang::prelude::{
  error,
  thiserror,
  ProgramError,
};

use std::{
  result::Result as StdResult
};

#[error]
/// Errors that can be triggered by executing one of the supported numeric operations
pub enum ErrorCode {
  #[msg("overflow")]
  Overflow,
  #[msg("underflow")]
  Underflow,
  #[msg("division by zero")]
  DivisionByZero,
}

/// Defines a set of safe math operations that return a `ProgramError` which is expected in an anchor instruction execution.
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
    /// $type implementation of the SafeMath trait
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
