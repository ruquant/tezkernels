// SPDX-FileCopyrightText: 2022 TriliTech <contact@trili.tech>
// SPDX-FileCopyrightText: 2022 Nomadic Labs <contact@nomadic-labs.com>
// SPDX-FileCopyrightText: 2022 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

//! Use this kernel with test unit and using the mode mock_host only

#![allow(dead_code)]
#![allow(unused_imports)]

use debug::debug_msg;
use host::input::{Input, MessageData, SlotData};
use host::rollup_core::{
    Input as InputType, RawRollupCore, MAX_INPUT_MESSAGE_SIZE, MAX_INPUT_SLOT_DATA_CHUNK_SIZE,
};
use host::runtime::Runtime;
use mock_host::{host_loop, HostInput};
use mock_runtime::state::HostState;

// host max read input size: 4096
const MAX_READ_INPUT_SIZE: usize = if MAX_INPUT_MESSAGE_SIZE > MAX_INPUT_SLOT_DATA_CHUNK_SIZE {
    MAX_INPUT_MESSAGE_SIZE
} else {
    MAX_INPUT_SLOT_DATA_CHUNK_SIZE
};

// Kernel: This kernel read input and write output to both the kernel output and log

pub fn test_output_run<Host: RawRollupCore>(host: &mut Host) {
    match host.read_input(MAX_READ_INPUT_SIZE) {
        Ok(Some(Input::Slot(message @ SlotData { level, id, .. }))) => {
            debug_msg!(Host, "slot data at level:{} - id:{}", level, id);
            host.write_output(message.as_ref()).unwrap();
        }
        Ok(Some(Input::Message(message @ MessageData { level, id, .. }))) => {
            debug_msg!(Host, "message data at level:{} - id:{}", level, id);

            host.write_output(message.as_ref()).unwrap();
        }
        Ok(None) => debug_msg!(Host, "no input"),
        Err(_) => todo!("Handle errors later"),
    }
}

#[cfg(not(test))]
mod entry {
    use super::*;

    use kernel::kernel_entry;

    kernel_entry!(test_output_run);
}

fn host_next(level: i32) -> HostInput {
    if level < 5 {
        HostInput::NextLevel(level)
    } else {
        HostInput::Exit
    }
}

fn get_input_batch(level: i32) -> Vec<(InputType, Vec<u8>)> {
    (1..level)
        .map(|l| {
            let input = if l % 2 == 0 {
                InputType::MessageData
            } else {
                InputType::SlotDataChunk
            };
            let bytes = format!("message at {} value {}", level, l).into();
            (input, bytes)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_host::host_loop;
    use mock_runtime::state::HostState;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test() {
        // Arrange
        let init = HostState::default();

        // calling the kernel with mock mode
        let final_state = host_loop(init, test_output_run, host_next, get_input_batch);

        // Assert inputs have been written to outputs
        let mut outputs: Vec<_> = final_state
            .store
            .as_ref()
            .iter()
            .filter(|(k, _)| k.starts_with("/output") && k.as_str() != "/output/id")
            .collect();
        outputs.sort();

        let mut inputs: Vec<_> = final_state
            .store
            .as_ref()
            .iter()
            .filter(|(k, _)| k.starts_with("/input") && k.contains("/payload"))
            .collect();
        inputs.sort();

        assert_eq!(
            outputs.iter().map(|(_, v)| v).collect::<Vec<_>>(),
            inputs.iter().map(|(_, v)| v).collect::<Vec<_>>()
        );
    }
}
