fn main() {
    if let Err(err) = capnpc::CompilerCommand::new()
        .file("schema/hello.capnp")
        .run()
    {
        panic!("capnpc: {err:?}")
    }
}
