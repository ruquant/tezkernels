// SPDX-FileCopyrightText: 2022 TriliTech <contact@trili.tech>
// SPDX-FileCopyrightText: 2022 Nomadic Labs <contact@nomadic-labs.com>
// SPDX-FileCopyrightText: 2022 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

#[cfg(feature = "write-debug")]
#[no_mangle]
pub unsafe extern "C" fn kernel_run() {
    use host::rollup_core::RawRollupCore;
    use host::wasm_host::WasmHost;

    let msg = b"hello-world";
    WasmHost::write_debug(msg.as_ptr(), msg.len());
}
