mod account;
mod amount;
mod communicate;
mod computer;
mod customer;
mod teller;

use communicate::*;
use computer::*;
use teller::*;

fn main() {
    let communicate = IoCommunicate::new();
    let computer = LiveComputer();

    let teller = Teller {
        communicate: &communicate,
        computer: &computer,
    };

    teller.interact();
}
