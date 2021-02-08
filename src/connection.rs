//! Connecting to KDB is pretty straightforward.
//! If you are using it embedded, then you can call new() with no parameters to connect to the
//! unerlying instance of KDB.
//! If you are using it externally, the call connect, which takes a host, port, credentials and an optional timeout.

use crate::any::Any;
use crate::error::{ConnectionError, Error};
use crate::k::K;
use crate::k_error::KError;
use crate::k_type::ERROR;
use crate::kapi;
use crate::kbox::KBox;

use std::ffi::CString;
use std::ptr;

/// null pointer with type inferred as *const K.
fn null() -> *const K {
    ptr::null()
}

macro_rules! evaluate {
    ($conn: expr, $func: expr $(, $param:expr)*,) => {
        evaluate!($conn, $func $(, $expr)*);
    };
    ($conn: expr, $func: expr $(, $param:expr)*) => {
        {
            let result = unsafe { kapi::k($conn, CString::new($func).unwrap().as_ptr() $(, $param.into_raw() as *const K)*, null()) };
            if result.is_null() {
                Err(Error::NetworkError)
            } else if $conn > 0 && unsafe { (*result).t == ERROR } {
                let err = unsafe{ KBox::<KError>::from_raw(result) }.into();
                Err(err)
            } else {
                Ok(result)
            }
        }
    };
}

fn from_raw(k: *mut K) -> KBox<Any> {
    unsafe { KBox::from_raw(k) }
}

/// Represents a connection to a remote or embedded KDB instance,
/// which can be used to send and query data on that instance.
pub struct Connection(i32);

impl Connection {
    /// [non-embedded only] Connect to a remote instance of KDB.
    #[cfg(not(feature = "embedded"))]
    pub fn connect(
        hostname: &str,
        port: u16,
        credentials: &str,
        timeout: Option<std::time::Duration>,
    ) -> Result<Self, ConnectionError> {
        let c_hostname = CString::new(hostname).unwrap();
        let c_credentials = CString::new(credentials).unwrap();

        let result = if let Some(duration) = timeout {
            let secs = duration.as_secs() as i32;
            unsafe { kapi::khpun(c_hostname.as_ptr(), port as i32, c_credentials.as_ptr(), secs) }
        } else {
            unsafe { kapi::khpu(c_hostname.as_ptr(), port as i32, c_credentials.as_ptr()) }
        };
        match result {
            0 => Err(ConnectionError::BadCredentials),
            -1 => Err(ConnectionError::CouldNotConnect),
            -2 => Err(ConnectionError::Timeout),
            x => Ok(Self(x)),
        }
    }

    /// [embedded only] Connect to an embedded KDB instance.
    #[cfg(any(feature = "embedded", doc))]
    pub fn new() -> Self {
        Connection(0)
    }

    /// [non-embedded only] Publish a value asynchronously to KDB.
    #[cfg(any(not(feature = "embedded"), doc))]
    pub fn publish(
        &self,
        callback: &str,
        topic: impl Into<KBox<Any>>,
        object: impl Into<KBox<Any>>,
    ) -> Result<(), Error> {
        // Note that when sending asynchronously, we shouldn't call r0 on the return value - it's
        // not an owned K type.
        evaluate!(-self.0, callback, topic.into(), object.into()).map(|_| ())
    }

    /// Evaluate a q expression with no parameters and return a result.
    pub fn eval(&self, query: &str) -> Result<KBox<Any>, Error> {
        evaluate!(self.0, query).map(from_raw)
    }

    /// Evaluate a q function with a single parameter and return the result.
    pub fn eval_1(&self, function: &str, param: impl Into<KBox<Any>>) -> Result<KBox<Any>, Error> {
        evaluate!(self.0, function, param.into()).map(from_raw)
    }

    /// Evaluate a q function with two parameters and return the result.
    pub fn eval_2(
        &self,
        function: &str,
        param: impl Into<KBox<Any>>,
        param_2: impl Into<KBox<Any>>,
    ) -> Result<KBox<Any>, Error> {
        evaluate!(self.0, function, param.into(), param_2.into()).map(from_raw)
    }

    /// Evaluate a q function with three parameters and return the result.
    pub fn eval_3(
        &self,
        function: &str,
        param: impl Into<KBox<Any>>,
        param_2: impl Into<KBox<Any>>,
        param_3: impl Into<KBox<Any>>,
    ) -> Result<KBox<Any>, Error> {
        evaluate!(self.0, function, param.into(), param_2.into(), param_3.into()).map(from_raw)
    }

    /// Evaluate a q function with four parameters and return the result.
    pub fn eval_4(
        &self,
        function: &str,
        param: impl Into<KBox<Any>>,
        param_2: impl Into<KBox<Any>>,
        param_3: impl Into<KBox<Any>>,
        param_4: impl Into<KBox<Any>>,
    ) -> Result<KBox<Any>, Error> {
        evaluate!(
            self.0,
            function,
            param.into(),
            param_2.into(),
            param_3.into(),
            param_4.into()
        )
        .map(from_raw)
    }

    /// Evaluate a q function with five parameters and return the result.
    pub fn eval_5(
        &self,
        function: &str,
        param: impl Into<KBox<Any>>,
        param_2: impl Into<KBox<Any>>,
        param_3: impl Into<KBox<Any>>,
        param_4: impl Into<KBox<Any>>,
        param_5: impl Into<KBox<Any>>,
    ) -> Result<KBox<Any>, Error> {
        evaluate!(
            self.0,
            function,
            param.into(),
            param_2.into(),
            param_3.into(),
            param_4.into(),
            param_5.into()
        )
        .map(from_raw)
    }

    /// Evaluate a q function with six parameters and return the result.
    #[allow(clippy::clippy::too_many_arguments)]
    pub fn eval_6(
        &self,
        function: &str,
        param: impl Into<KBox<Any>>,
        param_2: impl Into<KBox<Any>>,
        param_3: impl Into<KBox<Any>>,
        param_4: impl Into<KBox<Any>>,
        param_5: impl Into<KBox<Any>>,
        param_6: impl Into<KBox<Any>>,
    ) -> Result<KBox<Any>, Error> {
        evaluate!(
            self.0,
            function,
            param.into(),
            param_2.into(),
            param_3.into(),
            param_4.into(),
            param_5.into(),
            param_6.into()
        )
        .map(from_raw)
    }

    /// Evaluate a q function with seven parameters and return the result.
    #[allow(clippy::clippy::too_many_arguments)]
    pub fn eval_7(
        &self,
        function: &str,
        param: impl Into<KBox<Any>>,
        param_2: impl Into<KBox<Any>>,
        param_3: impl Into<KBox<Any>>,
        param_4: impl Into<KBox<Any>>,
        param_5: impl Into<KBox<Any>>,
        param_6: impl Into<KBox<Any>>,
        param_7: impl Into<KBox<Any>>,
    ) -> Result<KBox<Any>, Error> {
        evaluate!(
            self.0,
            function,
            param.into(),
            param_2.into(),
            param_3.into(),
            param_4.into(),
            param_5.into(),
            param_6.into(),
            param_7.into()
        )
        .map(from_raw)
    }

    /// See above and add one parameter.
    #[allow(clippy::clippy::too_many_arguments)]
    pub fn eval_8(
        &self,
        function: &str,
        param: impl Into<KBox<Any>>,
        param_2: impl Into<KBox<Any>>,
        param_3: impl Into<KBox<Any>>,
        param_4: impl Into<KBox<Any>>,
        param_5: impl Into<KBox<Any>>,
        param_6: impl Into<KBox<Any>>,
        param_7: impl Into<KBox<Any>>,
        param_8: impl Into<KBox<Any>>,
    ) -> Result<KBox<Any>, Error> {
        evaluate!(
            self.0,
            function,
            param.into(),
            param_2.into(),
            param_3.into(),
            param_4.into(),
            param_5.into(),
            param_6.into(),
            param_7.into(),
            param_8.into()
        )
        .map(from_raw)
    }
}

impl Drop for Connection {
    #[cfg(not(feature = "embedded"))]
    fn drop(&mut self) {
        unsafe {
            kapi::kclose(self.0);
        }
    }
    #[cfg(feature = "embedded")]
    fn drop(&mut self) {}
}

#[cfg(feature = "embedded")]
impl Default for Connection {
    fn default() -> Self {
        Connection(0)
    }
}
