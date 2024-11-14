//! # Data Wrapper
//! We use CPI calls to circumvent the 10kb log limit on Solana transactions.
//! Instead of logging events to the runtime, we execute a CPI to the `wrapper` program
//! where the log data is serialized into the instruction data.
//!
//! This works because CPI instruction data is never truncated. Logging information is
//! vital to the functioning of compression. When compression logs are truncated, indexers can fallback to
//! deserializing the CPI instruction data.

use crate::events::{AccountCompressionEvent, ApplicationDataEvent, ApplicationDataEventV1};
use anchor_lang::{prelude::*, solana_program::{pubkey, instruction::Instruction, program::invoke}};

#[derive(Clone)]
pub struct Noop;

const noop_program_id: Pubkey = pubkey!("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV");

pub fn instruction(data: Vec<u8>) -> Instruction {
    Instruction {
        program_id: noop_program_id,
        accounts: vec![],
        data,
    }
}

impl anchor_lang::Id for Noop {
    fn id() -> Pubkey {
        noop_program_id
    }
}

pub fn wrap_event<'info>(
    event: &AccountCompressionEvent,
    noop_program: &Program<'info, Noop>,
) -> Result<()> {
    invoke(
        &instruction(event.try_to_vec()?),
        &[noop_program.to_account_info()],
    )?;
    Ok(())
}

/// Wraps a custom event in the most recent version of application event data
pub fn wrap_application_data_v1<'info>(
    custom_data: Vec<u8>,
    noop_program: &Program<'info, Noop>,
) -> Result<()> {
    let versioned_data = ApplicationDataEventV1 {
        application_data: custom_data,
    };
    wrap_event(
        &AccountCompressionEvent::ApplicationData(ApplicationDataEvent::V1(versioned_data)),
        noop_program,
    )
}
