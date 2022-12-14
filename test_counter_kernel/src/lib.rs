#[macro_use]
extern crate kernel;
extern crate alloc;
extern crate debug;
extern crate kernel_core;
extern crate counter_kernel;
extern crate crypto;

use counter_kernel::{ counter_run, Counter };
use crypto::hash::{ ContractKt1Hash, HashTrait };
use host::{ rollup_core::Input as InputType, input::Input };
use kernel_core::{
    bls::BlsKey,
    encoding::{
        contract::Contract,
        string_ticket::StringTicket,
        public_key_hash::PublicKeyHash,
        smart_rollup::SmartRollupAddress,
        micheline::MichelineString,
        michelson::MichelsonPair,
        entrypoint::Entrypoint,
    },
    inbox::{
        Signer,
        Transfer,
        InternalInboxMessage,
        sendable::ExternalInboxMessage,
        sendable::InboxMessage,
        v1::{ sendable::{ Batch, Transaction }, Operation, OperationContent },
    },
    transactions::withdrawal,
    transactions_run,
};
use mock_host::{ host_loop, HostInput };
use mock_runtime::state::HostState;
use tezos_encoding::{ enc::BinWriter, nom::NomReader };

#[test]
fn test_counter() {
    kernel_entry!(transactions_run);

    // Arrange
    let bls_key = BlsKey::from_ikm([1; 32]);
    let signer = Signer::BlsPublicKey(bls_key.compressed_public_key().clone());
    let address = bls_key.public_key_hash().clone();

    let originator = Contract::Originated(
        ContractKt1Hash::from_b58check("KT1ThEdxfUcWUwqsdergy3QnbCWGHSUHeHJq").unwrap()
    );
    let contents = "Hello, Ticket!".to_string();

    let val = 0;
    let mut counter = Counter::new(val);
    let incr_counter = counter.increment();
    //let counter_bytes = format!("counter at {:#?}", counter).into();

    let string_ticket = StringTicket::new(originator.clone(), contents.clone(), incr_counter);
    //let string_ticket = StringTicket::new(originator.clone(), contents.clone(), 500);

    let sender = ContractKt1Hash::from_b58check("KT1PWx2mnDueood7fEmfbBDKx1D9BAnnXitn").unwrap();

    let source = PublicKeyHash::from_b58check("tz1Ke2h7sDdakHJQh8WX4Z372du1KChsksyU").unwrap();

    let destination = SmartRollupAddress::from_b58check(
        "scr1HLXM32GacPNDrhHDLAssZG88eWqCUbyLF"
    ).unwrap();

    // Deposit message is a counter
    let deposit = InboxMessage::Internal(
        InternalInboxMessage::Transfer(Transfer {
            payload: MichelsonPair(
                MichelineString(address.to_base58_check()),
                string_ticket.into()
            ),
            sender: sender.clone(),
            source,
            destination,
        })
    );

    // let counter 0 be the deposit message

    let mut deposit_message = Vec::new();
    deposit.bin_write(&mut deposit_message).unwrap();

    // Withdrawal
    let string_ticket = StringTicket::new(
        Contract::Originated(
            ContractKt1Hash::from_b58check("KT1ThEdxfUcWUwqsdergy3QnbCWGHSUHeHJq").unwrap()
        ),
        "Hello, Ticket".to_string(),
        450
    );
    let withdrawal = OperationContent::withdrawal(
        sender.clone(),
        string_ticket,
        Entrypoint::default()
    );
    let operation = Operation {
        signer,
        counter: 0,
        contents: vec![withdrawal],
    };
    let transaction = Transaction::new([(operation, bls_key.clone())]).expect("Valid Transaction");
    let batch = Batch::new(vec![transaction]);
    let message = InboxMessage::External(ExternalInboxMessage::V1(batch));
    // withdraw also counter 0
    /*let val_with = 0;
    let counter_with = Counter::new(val_with);
    let counter_withdraw_bytes = format!("counter at {:#?}", counter_with).into();*/
    let mut withdrawal_message = Vec::new();
    message.bin_write(&mut withdrawal_message).unwrap();

    // Old test
    /*let input_messages = |level: i32| -> Vec<(InputType, Vec<u8>)> {
        // Start the counter at 0
        let val = 0;
        let counter = Counter::new(val);
        let bytes = format!("counter at {:#?}", counter).into();
        if level == 1 {
            vec![(InputType::MessageData, bytes)]
        } else {
            vec![]
        }
    };*/
    let input_messages = |level: i32| -> Vec<(InputType, Vec<u8>)> {
        if level == 1 {
            vec![
                (InputType::MessageData, deposit_message.clone()),
                (InputType::MessageData, withdrawal_message.clone())
            ]
        } else {
            vec![]
        }
    };

    // Prepare Host
    let init = HostState::default();

    let host_next = |level: i32| -> HostInput {
        if level > 1 { HostInput::Exit } else { HostInput::NextLevel(1) }
    };

    let final_state = host_loop(init, mock_kernel_run, host_next, input_messages);

    // Get storage of outputs
    /*let mut outputs: Vec<_> = final_state.store
        .as_ref()
        .iter()
        .filter(|(k, _)| k.starts_with("/output") && k.as_str() != "/output/id")
        .collect();
    outputs.sort();*/
    let outputs: Vec<_> = final_state.store
        .as_ref()
        .iter()
        .filter(|(k, _)| k.starts_with("/output") && k.as_str() != "/output/id")
        .collect();

    assert_eq!(1, outputs.len(), "There should be a single outbox message");
}