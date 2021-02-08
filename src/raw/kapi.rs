use super::types::*;

pub type KCallback = extern "C" fn(arg1: I) -> *const K;

pub const K_NANO_OFFSET: i64 = 946_684_800_000_000_000;
pub const K_SEC_OFFSET: i64 = K_NANO_OFFSET / 1_000_000_000;
pub const K_DAY_OFFSET: i32 = (K_SEC_OFFSET / 86_400) as i32;

pub const K_TYPE_TIMESTAMP: i32 = -12;
pub const K_TYPE_TIMESPAN: i32 = -16;

pub(crate) unsafe fn tst(nanos: i64) -> *const K {
    ktj(K_TYPE_TIMESTAMP, nanos)
}

pub(crate) unsafe fn tsp(nanos: i64) -> *const K {
    ktj(K_TYPE_TIMESPAN, nanos)
}

#[cfg_attr(not(feature = "embedded"), link(name = "kdb"))]
extern "C" {
    pub fn b9(mode: I, x: *const K) -> *const K;

    pub fn d9(x: *const K) -> *const K;
    pub fn dj(date: I) -> I;
    pub fn dl(f: *mut V, n: I) -> *const K;
    pub fn dot(x: K, y: K) -> *const K;

    pub fn ee(x: *const K) -> *const K;

    pub fn ja(list: *mut *mut K, atom: *const V) -> *const K;
    pub fn js(list: *mut *mut K, symbol: S) -> *const K;
    pub fn jk(list: *mut *mut K, k: *const K) -> *const K;
    pub fn jv(list1: *mut *mut K, list2: *const K) -> *const K;

    pub fn k(handle: I, query: S, ...) -> *const K;
    pub fn ka(k_type: I) -> *const K;
    pub fn kb(boolean: I) -> *const K;
    pub fn kc(c_char: I) -> *const K;
    pub fn kd(date: I) -> *const K;
    pub fn ke(real: F) -> *const K;
    pub fn kf(float: F) -> *const K;
    pub fn kg(byte: I) -> *const K;
    pub fn kh(short: I) -> *const K;
    pub fn ki(int: I) -> *const K;
    pub fn kj(long: J) -> *const K;
    pub fn kp(c_str: S) -> *const K;
    pub fn kpn(c_str: S, len: J) -> *const K;
    pub fn krr(c_str: S) -> *const K;
    pub fn ks(c_str: S) -> *const K;
    pub fn kt(time: I) -> *const K;
    pub fn ktd(keyed_table: *const K) -> *const K;
    pub fn ktj(begin: I, end: J) -> *const K;
    pub fn ktn(k_type: I, len: J) -> *const K;
    pub fn knk(arg1: I, ...) -> *const K;
    pub fn ku(guid: Guid) -> *const K;
    pub fn kz(date_time: F) -> *const K;

    pub fn m9() -> V;

    pub fn orr(c_str: S) -> *const K;
    pub fn okx(x: *const K) -> I;

    pub fn r0(k: *const K) -> V;
    pub fn r1(k: *const K) -> *const K;

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
