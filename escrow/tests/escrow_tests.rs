use escrow_io::{ InitEscrow, EscrowAction, EscrowEvent };
use gtest::{ Log, Program, System };

const BUYER: u64 = 100;
const SELLER: u64 = 101;
const PRICE: u64 = 100_000;
const ESCROW_ID: u64  = 1;

#[test]
fn testing_correct_init() {
    let sys = System::new();
    sys.init_logger();
    let escrow = Program::current(&sys);
    let res = escrow.send(
        2,
        InitEscrow {
            seller: 3.into(), //SELLER.into(),
            buyer: 4.into(), //BUYER.into(),
            price: 100_100
        }
    );

    assert!(!res.main_failed());
}


/*
#[test]
fn deposit() {
    let sys = System::new();
    sys.init_logger();
    let escrow = Program::current(&sys);
    let res = escrow.send(
        SELLER,
        InitEscrow {
            seller: SELLER.into(),
            buyer: BUYER.into(),
            price: PRICE.into()
        }
    );

    assert!(!res.main_failed());

    // sending a message from buyers accound
    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, PRICE.into());

    let res = escrow.send_with_value (
        BUYER,
        EscrowAction::Deposit,
        PRICE.into()
    );
    let log  = Log::builder()
        .dest(BUYER)
        .payload(EscrowEvent::FundsDeposited);
    assert!(res.contains(&log));

    let escrow_balance = sys.balance_of(ESCROW_ID);
    assert_eq!(escrow_balance, PRICE.into());
}

#[test]
fn deposit2() {
    let sys = System::new();
    let escrow = Program::current(&sys);
    let res = escrow.send(
        SELLER,
        InitEscrow {
            seller: SELLER.into(),
            buyer: BUYER.into(),
            price: PRICE.into(),
        },
    );
    assert!(res.log().is_empty());
    let res = escrow.send_with_value(BUYER, EscrowAction::Deposit, PRICE.into());
}

#[test]
fn deposit_failures() {
    let sys = System::new();
    sys.init_logger();
    let escrow = Program::current(&sys);
    let res = escrow.send(
        SELLER,
        InitEscrow {
            seller: SELLER.into(),
            buyer: BUYER.into(),
            price: PRICE.into()
        }
    );
    assert!(!res.main_failed());

    sys.mint_to(BUYER, (2*PRICE).into());
    // must fail since BUYER attaches not enough value
    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        (2*PRICE - 500).into()
    );
    assert!(res.main_failed());

    //  must fail  since  the  message sender   is  not BUYER
    let res = escrow.send(SELLER, EscrowAction::Deposit);
    assert!(res.main_failed());

    //sucessful deposit
    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        PRICE.into()
    );
    assert!(!res.main_failed());

    // must fail since the state must be `AwaittingPayment`
    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        PRICE.into()
    );
    assert!(res.main_failed());
}

*/