Anchor Safe Math
===

```
use anchor_lang::prelude::*;
use safe_math::{SafeMath};

#[program]
pub mod example {
  use super::*;

  pub fn instruction(ctx: Context<Instruction>, amount: u64) -> ProgramResult {
    let state = &mut ctx.accounts.state;

    // You can apply any of the following operations
    state.total_amount = state.total_amount.safe_add(amount)?;
    state.total_amount = state.total_amount.safe_sub(amount)?;
    state.total_amount = state.total_amount.safe_mul(amount)?;
    state.total_amount = state.total_amount.safe_div(amount)?;
    state.total_amount = state.total_amount.safe_pow(8_u32)?;
  }
}

#[derive(Accounts)]
pub struct Instruction<'info> {
  ...
}
```

Works with `u128`, `u64`, `u32`, `u16` and `u8`
