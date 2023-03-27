// src/counter.rs

use tezos_data_encoding::enc::BinWriter;
// use tezos_data_encoding::nom::NomReader;
// use tezos_data_encoding::encoding::HasEncoding;

#[derive(Debug, PartialEq)]
pub struct Counter {
    pub(crate) counter: i64,
}

impl Default for Counter {
    fn default() -> Counter {
        Counter { counter: 0 }
    }
}

impl Counter {
    fn increment(self) -> Counter {
        Counter {
            counter: self.counter + 1,
        }
    }

    fn decrement(self) -> Counter {
        Counter {
            counter: self.counter - 1,
        }
    }
}

impl TryFrom<Vec<u8>> for Counter {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value
            .try_into()
            .map_err(|_| "i64 is represented by 8 bytes".to_string())
            .map(i64::from_be_bytes)
            .map(|counter| Counter { counter })
    }
}

impl Into<[u8; 8]> for Counter {
    fn into(self) -> [u8; 8] {
        self.counter.to_be_bytes()
    }
}

#[derive(Debug, PartialEq, Eq, BinWriter)]
#[encoding(tags = "u8")]
pub enum UserAction {
    Increment,
    Decrement,
    Reset
}

pub fn transition(counter: Counter, action: UserAction) -> Counter {
    match action {
        UserAction::Increment => counter.increment(),
        UserAction::Decrement => counter.decrement(),
        UserAction::Reset => Counter::default(),
    }
}

impl TryFrom<&[u8]> for UserAction {
    type Error = (); // FIXME: proper error type
    
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            [0, ..] => Ok(UserAction::Increment),
            _ => panic!("")
        }
    }
}
