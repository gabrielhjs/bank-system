use actix::prelude::*;
use bank_system::actors::{
  account::Account,
  account_logger::AccountLogger,
  messages::{Deposit, Withdraw},
};

#[actix_rt::main]
async fn main() {
  let account_logger_addr = AccountLogger::new().start();
  let account_gabriel_addr =
    Account::new("gabriel", 0, account_logger_addr.clone()).start();
  let account_eunice_addr =
    Account::new("eunice", 100, account_logger_addr).start();

  let _ = account_gabriel_addr.send(Deposit(100)).await;
  let _ = account_eunice_addr.send(Withdraw(50)).await;

  System::current().stop();
}
