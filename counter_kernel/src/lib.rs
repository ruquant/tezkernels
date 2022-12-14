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

use anyhow::{ ensure, Result };
use debug::debug_msg;
use host::input::{ Input as InputType, MessageData };
use host::rollup_core::{ Input };
use host::path::OwnedPath;
use host::rollup_core::{ RawRollupCore, MAX_INPUT_MESSAGE_SIZE, MAX_INPUT_SLOT_DATA_CHUNK_SIZE };
use host::runtime::Runtime;
use kernel::kernel_entry;
use mock_runtime::host::MockHost;

const MAX_READ_INPUT_SIZE: usize = if MAX_INPUT_MESSAGE_SIZE > MAX_INPUT_SLOT_DATA_CHUNK_SIZE {
    MAX_INPUT_MESSAGE_SIZE
} else {
    MAX_INPUT_SLOT_DATA_CHUNK_SIZE
};

/// Counter
#[derive(Debug)]
pub struct Counter {
    val: i8,
}

impl Counter {
    /// Create a new counter
    pub fn new(val: i8) -> Self {
        Self { val }
    }

    /// Public read-only method: Returns the counter value
    pub fn get_num(&self) -> i8 {
        self.val
    }

    /// Increment the counter
    pub fn increment(&mut self) -> i8 {
        self.val += 1;
        return self.val;
    }

    pub fn decrement(&mut self) -> i8 {
        self.val -= 1;
        return self.val;
    }

    /// Reset the counter to 0
    pub fn reset(&mut self) -> i8 {
        self.val = 0;
        return self.val;
    }
}

pub fn handle_input_message<H: RawRollupCore>(host: &mut H, message: MessageData) -> Result<()> {
    // processing counter function
    let path: OwnedPath = "/counter"
        .as_bytes()
        .to_vec()
        .try_into()
        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?;
    let counter = match Runtime::store_read(host, &path, 0, 1) {
        Ok(counter) => counter,
        Err(_) => vec![0],
    };
    ensure!(counter.len() == 1, "counter is not a byte");
    let mut counter = Counter {
        val: i8::from_le_bytes(counter[..].try_into().unwrap()),
    };

    // Handle message and counter incr
    debug_msg!(H, "Received message {:?}", message);
    counter.increment();
    debug_msg!(H, "Counter {:#?}", counter);

    // We need to persist the effect in `counter`
    Runtime::store_write(host, &path, &counter.val.to_le_bytes(), 0).map_err(|e|
        anyhow::Error::msg(format!("{e:?}"))
    )?;
    Ok(())
}

pub fn counter_run<Host: RawRollupCore>(host: &mut Host) {
    // Reading the input from host
    match host.read_input(MAX_READ_INPUT_SIZE) {
        Ok(Some(InputType::Message(message))) => {
            debug_msg!(Host, "message data at level:{} - id:{}", message.level, message.id);
            if let Err(_) = handle_input_message(host, message) {
                // log and gracefully exit
            }
        }
        Ok(Some(InputType::Slot(_message))) => todo!("handle slot message"),
        Ok(None) => debug_msg!(Host, "no input"),
        Err(_) => todo!("handle errors"),
    }
}

#[cfg(feature = "counter-kernel")]
pub mod counter_kernel {
    kernel_entry!(counter_run);
}

#[test]
fn test() {
    use mock_runtime::host::MockHost;
    kernel_entry!(counter_run);
    use crate::counter_run;

    // Arrange
    let mut mock_runtime = MockHost::default();
    let val = 0;
    let counter = Counter::new(val);
    let message = format!("counter {}", counter.get_num()).into();

    let level = 10;
    mock_runtime.as_mut().set_ready_for_input(level);
    mock_runtime.as_mut().add_next_inputs(10, vec![(Input::MessageData, message)].iter());

    // Act
    counter_run(&mut mock_runtime);
}