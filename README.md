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

    state.total_amount = state.total_amount.safe_add(amount)?;
    state.total_amount = state.total_amount.safe_sub(amount)?;
  }
}

#[derive(Accounts)]
pub struct Instruction<'info> {
  ...
}
```
