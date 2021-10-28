use actix::prelude::Message;

#[derive(Message)]
#[rtype(result = "bool")]
pub struct Withdraw(pub usize);

#[derive(Message)]
#[rtype(result = "bool")]
pub struct Deposit(pub usize);

#[derive(Message)]
#[rtype(result = "()")]
pub struct BalanceUpdated<'a>(pub &'a str, pub isize);
