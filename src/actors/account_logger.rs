use actix::prelude::*;

use super::messages::BalanceUpdated;

pub struct AccountLogger;

impl AccountLogger {
  pub fn new() -> Self {
    AccountLogger {}
  }
}

impl Actor for AccountLogger {
  type Context = Context<Self>;
}

impl Handler<BalanceUpdated<'_>> for AccountLogger {
  type Result = ();

  fn handle(
    &mut self,
    msg: BalanceUpdated,
    _ctx: &mut Self::Context,
  ) -> Self::Result {
    println!("Balance of account ({}) is updated to: ({})", msg.0, msg.1)
  }
}
