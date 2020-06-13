//  root$ cargo build --target=i686-unknown-linux-gnu && q src/load.q m32

#[macro_use]
extern crate lazy_static;
use kdb::raw::types::*;
use kdb::{c_api, KAny, KError, KIntAtom, KIntList, KSymbolAtom, Unowned};
use nix::fcntl::OFlag;
use nix::unistd::pipe2;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::os::unix::io::RawFd;
use std::sync::RwLock;
use std::{
    fs::File,
    io::{Read, Write},
    os::unix::io::FromRawFd,
};

lazy_static! {
    static ref FD_LIST: RwLock<HashMap<RawFd, RwLock<File>>> = RwLock::new(HashMap::new());
}

#[no_mangle]
pub extern "C" fn start() -> KAny {
    let flags: OFlag = OFlag::from_bits_truncate(libc::O_CLOEXEC | libc::O_NONBLOCK);
    if let Ok((recv_fd, send_fd)) = pipe2(flags) {
        FD_LIST
            .write()
            .unwrap()
            .insert(recv_fd, RwLock::new(unsafe { File::from_raw_fd(recv_fd) }));
        unsafe { c_api::sd1(recv_fd, Some(callback)) };

        std::thread::spawn(move || {
            let mut f = unsafe { File::from_raw_fd(send_fd) };
            loop {
                match write!(&mut f, "Hello pipe!") {
                    Err(e) => {
                        println!("pipe_snd_thread_err:{:?}", e);
                        unsafe { c_api::sd0x(recv_fd, 0i32) };
                        break;
                    }
                    Ok(_) => std::thread::sleep(std::time::Duration::from_millis(1000)),
                };
            }
        });
        let mut list: KIntList = KIntList::new();
        list.push(recv_fd);
        list.push(send_fd);
        list.into()
    } else {
        KError::new("Can't create PIPE").unwrap().into()
    }
}

#[no_mangle]
pub extern "C" fn callback(fd: I) -> *const K {
    let mut buff = [0; 100];
    let k = kdb::Connection::new();
    if let Some(f) = FD_LIST.write().unwrap().get(&fd) {
        match f.write().unwrap().read(&mut buff[..]) {
            Ok(n) => {
                let data = KSymbolAtom::try_from(std::str::from_utf8(&buff[..n]).unwrap()).unwrap();
                let _ = k.publish("upd", KIntAtom::from(fd), data);
            }
            Err(e) => println!("FD:[{}] read error:{:?}", fd, e),
        };
    };
    std::ptr::null()
}

#[no_mangle]
pub extern "C" fn close_fd(fd: Unowned<KIntAtom>) -> KAny {
    if FD_LIST.read().unwrap().contains_key(&*fd) {
        unsafe { c_api::sd0x(**fd, 0i32) };
    };

    if let Some(_f) = FD_LIST.write().unwrap().remove(&*fd) {
        true.into()
    } else {
        KError::new(format!("FD[{:?}] does not exist", *fd).as_str())
            .unwrap()
            .into()
    }
}
