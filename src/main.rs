// sim-package system

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Validity {
    Daily,
    Weekly,
    Monthly
}

impl Validity {
    fn days(&self) -> u32 {
        match self {
            Validity::Daily => 1,
            Validity::Weekly => 7,
            Validity::Monthly => 30,
        }
    }
}

#[derive(Debug, Clone)]
enum Bundle {
    Airtime {
        amount: u32
    },
    Data {
        megabytes: u32, validity: Validity
    },
    Voice {
        minutes: u32, validity: Validity
    },
}


impl Bundle {
    fn describe(&self) -> String {
        match self {
            Bundle::Airtime { amount } => format!("UGX {} airtime", amount),
            Bundle::Data { megabytes, validity } if *megabytes >= 1024 => {
                format!(" {} GB data, {} day(s)", megabytes /1024, validity.days())
            }
            Bundle::Data { megabytes, validity } => {
                format!(" {} MB data, {} day(s)", megabytes, validity.days())
            }
            Bundle::Voice { minutes, validity } => {
                format!(" {} minutes voice, {} day(s)", minutes, validity.days())
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum SimStatus{
    Active,
    PendingKYC,
    Blocked,
}

#[derive(Debug)]
enum SimError {
    InsufficientAirtimeBalance { needed: u32, available: u32 },
    InvalidBundle,
    SimBlocked,
    NoDataBundle,
}


impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimError::InsufficientAirtimeBalance {
                needed, available => write!(f, "Insufficient airtime balance: needed {}, you have {}", needed, available),
                SimError::InvalidBundle => write!(f, "Invalid Bundle"),
                SimError::SimBlocked => write!(f, "Sim is blocked, Contact your service provider for support"),
                SimError::NoDataBundle => write!(f, "Your data bundle has expired, please recharge to continue using data services"),
            }
        }
    }
}



fn main() {
    println!("Hello, world!");
}
