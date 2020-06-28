mod account;
mod amount;
mod communicate;
mod customer;
mod teller;

use communicate::*;
use teller::*;

fn main() {
    let communicate = IoCommunicate::new();

    let teller = Teller {
        communicate: &communicate,
    };
}
