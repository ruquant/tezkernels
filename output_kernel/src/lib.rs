/*****************************************************************************/
/*                                                                           */
/* Open Source License                                                       */
/* Copyright (c) 2022 Nomadic Labs <contact@nomadic-labs.com>                */
/*                                                                           */
/* Permission is hereby granted, free of charge, to any person obtaining a   */
/* copy of this software and associated documentation files (the "Software"),*/
/* to deal in the Software without restriction, including without limitation */
/* the rights to use, copy, modify, merge, publish, distribute, sublicense,  */
/* and/or sell copies of the Software, and to permit persons to whom the     */
/* Software is furnished to do so, subject to the following conditions:      */
/*                                                                           */
/* The above copyright notice and this permission notice shall be included   */
/* in all copies or substantial portions of the Software.                    */
/*                                                                           */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR*/
/* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,  */
/* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL   */
/* THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER*/
/* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING   */
/* FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER       */
/* DEALINGS IN THE SOFTWARE.                                                 */
/*                                                                           */
/*****************************************************************************/

/* Use this kernel with test unit and using the mode mock_host only */

#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate kernel;
#[macro_use]
extern crate debug;
extern crate alloc;

use host::rollup_core::{
    Input as InputType,
    RawRollupCore,
    MAX_INPUT_MESSAGE_SIZE,
    MAX_INPUT_SLOT_DATA_CHUNK_SIZE,
};
use host::input::{ Input, MessageData, SlotData };
use host::runtime::Runtime;
use mock_host::{ host_loop, HostInput };
use mock_runtime::state::HostState;

// host max read input size: 4096
const MAX_READ_INPUT_SIZE: usize = if MAX_INPUT_MESSAGE_SIZE > MAX_INPUT_SLOT_DATA_CHUNK_SIZE {
    MAX_INPUT_MESSAGE_SIZE
} else {
    MAX_INPUT_SLOT_DATA_CHUNK_SIZE
};

/* Kernel: 
    This kernel read input and write output to both the kernel output and log
*/
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

kernel_entry!(test_output_run);

fn host_next(level: i32) -> HostInput {
    if level < 5 { HostInput::NextLevel(level) } else { HostInput::Exit }
}

fn get_input_batch(level: i32) -> Vec<(InputType, Vec<u8>)> {
    (1..level)
        .map(|l| {
            let input = if l % 2 == 0 { InputType::MessageData } else { InputType::SlotDataChunk };
            let bytes = format!("message at {} value {}", level, l).into();
            (input, bytes)
        })
        .collect()
}

#[test]
fn test() {
    // Arrange
    let init = HostState::default();

    // calling the kernel with mock mode
    let final_state = host_loop(init, mock_kernel_run, host_next, get_input_batch);

    // Assert inputs have been written to outputs
    let mut outputs: Vec<_> = final_state.store
        .as_ref()
        .iter()
        .filter(|(k, _)| k.starts_with("/output") && k.as_str() != "/output/id")
        .collect();
    outputs.sort();

    let mut inputs: Vec<_> = final_state.store
        .as_ref()
        .iter()
        .filter(|(k, _)| k.starts_with("/input") && k.contains("/payload"))
        .collect();
    inputs.sort();

    assert_eq!(
        outputs
            .iter()
            .map(|(_, v)| v)
            .collect::<Vec<_>>(),
        inputs
            .iter()
            .map(|(_, v)| v)
            .collect::<Vec<_>>()
    );
}