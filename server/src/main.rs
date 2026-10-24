use std::{
    io::{self, Read},
    net::Shutdown,
    os::unix::net::UnixStream,
};

use amane::IpcCall;

fn main() {
    let socket_path = amane::ipc_socket();
    let stream = UnixStream::connect(socket_path).unwrap();

    let call = IpcCall::new("topbar", &["toggle-floating".into()]);
    send(stream, &call).unwrap();
}

fn send(mut stream: UnixStream, call: &IpcCall) -> io::Result<String> {
    call.write(&mut stream)?;

    // closing our sending side is how the shell knows the call is complete
    stream.shutdown(Shutdown::Write)?;

    let mut reply = String::new();

    stream.read_to_string(&mut reply)?;

    Ok(reply)
}
