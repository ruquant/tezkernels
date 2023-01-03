use host::{
    path::{Path, RefPath},
    rollup_core::{RawRollupCore, ReadInputMessageInfo},
    runtime::{Runtime, ValueType},
};
#[cfg(not(test))]
use kernel::kernel_entry;
use std::{
    io::{Cursor, Write},
    mem::MaybeUninit,
};

const PATH: RefPath = RefPath::assert_from(b"/installer/kernel/incomplete.wasm");

/// A very lean input reader that avoids dynamic allocation at all.
unsafe fn read_input<'a, H: RawRollupCore>(host: &mut H, buf: &'a mut [u8]) -> Option<&'a [u8]> {
    let mut message_info = MaybeUninit::<ReadInputMessageInfo>::uninit();
    let res =
        RawRollupCore::read_input(host, message_info.as_mut_ptr(), buf.as_mut_ptr(), buf.len());
    match res.try_into() {
        Ok(0) | Err(_) => None,
        Ok(len) => Some(&buf[..len]),
    }
}

pub fn get_checkpoint<H: RawRollupCore>(host: &mut H) -> usize {
    match Runtime::store_has(host, &PATH) {
        Ok(Some(ValueType::Value)) => {
            let size = unsafe { host.store_value_size(PATH.as_ptr(), PATH.size()) };
            if size < 0 {
                panic!()
            }
            if let Ok(size) = size.try_into() {
                size
            } else {
                panic!()
            }
        }
        Ok(None) => 0,
        Ok(_) | Err(_) => panic!(),
    }
}

/// Kernel installer by ingesting kernel pieces limited to 4095 bytes in length.
pub fn installer<H: RawRollupCore>(host: &mut H) {
    let mut buf = [0; 4096];
    loop {
        let Some(input) =
        (unsafe { read_input(host, &mut buf) })
        else {
            break
        };
        let [1, input @ ..] = input else { continue };
        if let [tag, rest @ ..] = input {
            let off = get_checkpoint(host);
            let Ok(()) =
                Runtime::store_write(host, &PATH, rest, off)
                else { panic!() };
            match tag {
                0 => {
                    let mut msg = Cursor::new([0u8; 128]);
                    let _ = write!(&mut msg, "input {} bytes", input.len());
                    let msg = msg.into_inner();
                    host.write_output(&msg).unwrap();
                }
                1 => {
                    let Ok(()) =
                        Runtime::store_move(host, &PATH, &host::path::PATH_KERNEL_BOOT)
                        else { panic!() };
                    host.write_output(b"finished").unwrap();
                }
                _ => panic!(),
            }
        }
    }
}

#[cfg(not(test))]
kernel_entry!(installer);

#[cfg(test)]
mod tests {
    use host::rollup_core::Input;
    use mock_runtime::{host::MockHost, state::HostState};
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::installer;

    // This test is a smoke test to ensure that the kernel are successfully reconstructed
    // from pieces fed from L1 external messages.
    #[wasm_bindgen_test]
    fn test_installer() {
        let mut state = HostState::default();
        state.set_ready_for_input(0);
        state.add_next_inputs(
            0,
            [
                (Input::MessageData, vec![1, 0, 0]),
                (Input::MessageData, vec![1, 1, 1, 2]),
            ]
            .iter(),
        );
        let mut host = MockHost::from(state);
        installer(&mut host);
        let state = host.into_inner();
        let data: Vec<u8> = state.store.get_value("/durable/kernel/boot.wasm");
        assert_eq!(data, [0, 1, 2])
    }
}
