//! Provides ffi definitions for the KDB C API.

use crate::date_time_types::{Minute, Month, Second};
use crate::k::{F, I, J, K, S, V};
use crate::k_type::{MINUTE_ATOM, MONTH_ATOM, SECOND_ATOM};

pub type KCallback = extern "C" fn(arg1: I) -> *const K;

pub const K_TYPE_TIMESTAMP: i32 = -12;
pub const K_TYPE_TIMESPAN: i32 = -16;

pub(crate) unsafe fn tst(nanos: i64) -> *const K {
    ktj(K_TYPE_TIMESTAMP, nanos)
}

pub(crate) unsafe fn tsp(nanos: i64) -> *const K {
    ktj(K_TYPE_TIMESPAN, nanos)
}

pub(crate) unsafe fn ksec(sec: Second) -> *mut K {
    let atom = ka(SECOND_ATOM.into());
    (*atom).union.sec = sec;
    atom
}

pub(crate) unsafe fn kmin(min: Minute) -> *mut K {
    let atom = ka(MINUTE_ATOM.into());
    (*atom).union.min = min;
    atom
}

pub(crate) unsafe fn kmonth(m: Month) -> *mut K {
    let atom = ka(MONTH_ATOM.into());
    (*atom).union.m = m;
    atom
}

#[cfg_attr(not(feature = "embedded"), link(name = "kdb"))]
extern "C" {
    pub fn b9(mode: I, x: *const K) -> *mut K;

    pub fn d9(x: *const K) -> *const K;
    pub fn dj(date: I) -> I;
    pub fn dl(f: *mut V, n: I) -> *const K;
    pub fn dot(x: *const K, y: *const K) -> *mut K;

    pub fn ee(x: *mut K) -> *mut K;

    pub fn ja(list: *mut *mut K, atom: *const V) -> *mut K;
    pub fn js(list: *mut *mut K, symbol: S) -> *mut K;
    pub fn jk(list: *mut *mut K, k: *const K) -> *mut K;
    pub fn jv(list1: *mut *mut K, list2: *const K) -> *mut K;

    pub fn k(handle: I, query: S, ...) -> *mut K;
    pub fn ka(k_type: I) -> *mut K;
    pub fn kb(boolean: I) -> *mut K;
    pub fn kc(c_char: I) -> *mut K;
    pub fn kd(date: I) -> *mut K;
    pub fn ke(real: F) -> *mut K;
    pub fn kf(float: F) -> *mut K;
    pub fn kg(byte: I) -> *mut K;
    pub fn kh(short: I) -> *mut K;
    pub fn ki(int: I) -> *mut K;
    pub fn kj(long: J) -> *mut K;
    pub fn kp(c_str: S) -> *mut K;
    pub fn kpn(c_str: S, len: J) -> *mut K;
    pub fn krr(c_str: S) -> *mut K;
    pub fn ks(c_str: S) -> *mut K;
    pub fn kt(time: I) -> *mut K;
    pub fn ktd(keyed_table: *const K) -> *const K;
    pub fn ktj(begin: I, end: J) -> *const K;
    pub fn ktn(k_type: I, len: J) -> *mut K;

    #[cfg(feature = "uuid")]
    pub fn ku(guid: uuid::Uuid) -> *mut K;
    pub fn kz(date_time: F) -> *const K;

    pub fn m9() -> V;

    pub fn orr(c_str: S) -> *const K;
    pub fn okx(x: *const K) -> I;

    pub fn r0(k: *mut K) -> V;
    pub fn r1(k: *mut K) -> *mut K;

    pub fn sd1(d: I, cb: Option<KCallback>) -> *const K;
    pub fn sd0(d: I) -> V;
    pub fn sd0x(d: I, f: I) -> V;
    pub fn setm(m: I) -> I;
    pub fn ss(c_str: S) -> S;
    pub fn sn(c_str: S, len: I) -> S;

    pub fn xT(dict: *const K) -> *const K;
    pub fn xD(keys: *const K, values: *const K) -> *const K;

    pub fn ymd(y: I, m: I, d: I) -> I;
}

#[cfg(not(feature = "embedded"))]
#[link(name = "kdb")]
extern "C" {
    pub fn kclose(handle: I) -> V;
    pub fn khp(hostname: S, port: I) -> I;
    pub fn khpu(hostname: S, port: I, credentials: S) -> I;
    pub fn khpun(hostname: S, port: I, credentials: S, timeout: I) -> I;
}
