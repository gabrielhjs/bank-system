use actix::prelude::*;
use std::convert::TryInto;

use super::account_logger::AccountLogger;
use super::messages::{BalanceUpdated, Deposit, Withdraw};

pub struct Account {
  pub owner: &'static str,
  pub balance: isize,
  pub logger: Addr<AccountLogger>,
}

impl Account {
  pub fn new(
    owner: &'static str,
    balance: isize,
    logger: Addr<AccountLogger>,
  ) -> Self {
    Account {
      owner,
      balance,
      logger,
    }
  }
}

impl Actor for Account {
  type Context = Context<Self>;
}

impl Handler<Withdraw> for Account {
  type Result = bool;

  fn handle(
    &mut self,
    msg: Withdraw,
    _ctx: &mut Self::Context,
  ) -> Self::Result {
    match msg.0.try_into() {
      Ok::<isize, _>(amount) => {
        if self.balance >= amount {
          self.balance -= amount;
          self
            .logger
            .do_send(BalanceUpdated(self.owner, self.balance));
          true
        } else {
          false
        }
      }
      Err(_) => false,
    }
  }
}

impl Handler<Deposit> for Account {
  type Result = bool;

  fn handle(&mut self, msg: Deposit, _ctx: &mut Self::Context) -> Self::Result {
    match msg.0.try_into() {
      Ok::<isize, _>(amount) => {
        self.balance += amount;
        self
          .logger
          .do_send(BalanceUpdated(self.owner, self.balance));
        true
      }
      Err(_) => false,
    }
  }
}
