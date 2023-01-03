# Introduction

# Steps to bootstrap and install

## Split the kernel
It is necessary to split the final kernel into pieces of no more than 4095 bytes,
because L1 external messages in SORU inboxes are limited to 4096 bytes.
Of the 4096 bytes, one byte is required for signaling the end of transmission and
the rest is the pieces of kernel to be used to reconstruct the final kernel.

    mkdir -p out
    rm out/kernel-*
    wasm-strip -o /dev/stdout <path-to-wasm> | split -db4095 - out/kernel-
    readarray -t fs < <(ls out/kernel-* | sort)
    nr="${#fs}"
    for i in "${!fs[@]}"
    do
        if [[ $i = $nr ]]
        then
            printf '\x01' | cat - "${fs[$i]}" | jq -Rsa '{external:.}'
        else
            printf '\x00' | cat - "${fs[$i]}" | jq -Rsa '{external:.}'
        fi
    done | jq -sa '[.]' >messages.json

Once completed, `messages.json` is ready for use for simulation in REPL.
It can be loaded as inbox messages.
When the installer kernel is step through, the kernel will be installed at the end.
