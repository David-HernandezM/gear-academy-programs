#![no_std]
use gstd::{prelude::*, msg, ActorId};
use escrow_io::*;

/*
channel = "nightly-2023-04-25"
targets = ["wasm32-unknown-unknown"]
profile = "default"
 */

static  mut ESCROW: Option<Escrow> = None;

#[no_mangle]
extern "C" fn init() {
    let init_config: InitEscrow = msg::load()
        .expect("Error in decoding `InitEscrow");
    let escrow = Escrow {
        seller: init_config.seller,
        buyer: init_config.buyer,
        price: init_config.price,
        state: EscrowState::AwaitingPayment
    };
    unsafe {
        ESCROW = Some(escrow)
    };
}

#[no_mangle]
extern "C" fn handle() {
    let action: EscrowAction = msg::load()
        .expect("Unable to decode `EscrowAction");
    let escrow: &mut Escrow = unsafe {
        ESCROW
            .as_mut()
            .expect("The contract is no initialized")
    };
    match action {
        EscrowAction::Deposit => escrow.deposit(),
        EscrowAction::ConfirmDelivery => escrow.confirm_delivery()
    }
}

#[no_mangle]
extern "C" fn state() {
    let escrow = unsafe {
        ESCROW.get_or_insert(Default::default())  
    };
    msg::reply(escrow, 0).expect("Failed to share state");
}