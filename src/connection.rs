//! Connecting to KDB is pretty straightforward.
//! If you are using it embedded, then you can call new() with no parameters to connect to the
//! unerlying instance of KDB.
//! If you are using it externally, the call connect, which takes a host, port, credentials and an optional timeout.

use crate::any::KAny;
use crate::atoms::KError;
use crate::error::Error;
use crate::raw::kapi;
use crate::raw::types::{ERROR, I, K};
use std::ffi::CString;
use std::ptr;

fn null() -> *const K {
    ptr::null()
}

macro_rules! evaluate {
    ($conn: expr, $func: expr $(, $param:expr)*,) => {
        evaluate!($conn, $func $(, $expr)*);
    };
    ($conn: expr, $func: expr $(, $param:expr)*) => {
        {
            let result = unsafe { kapi::k($conn, CString::new($func).unwrap().as_ptr() $(, $param.into_ptr())*, null()) };
            if result.is_null() {
                Err(Error::NetworkError)
            } else if $conn > 0 && unsafe { (*result).t == ERROR } {
                let err = KError(result).into();
                Err(err)
            } else {
                Ok(KAny(result))
            }
        }
    };
}

#[derive(Debug)]
pub struct Connection(I);

impl Connection {
    #[cfg(not(feature = "embedded"))]
    pub fn connect(
        hostname: &str,
        port: u16,
        credentials: &str,
        timeout: Option<std::time::Duration>,
    ) -> Result<Self, crate::error::ConnectionError> {
        use crate::error::ConnectionError;
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

    #[cfg(feature = "embedded")]
    pub fn new() -> Self {
        Connection(0)
    }

    pub fn publish(&self, callback: &str, topic: impl Into<KAny>, object: impl Into<KAny>) -> Result<(), Error> {
        evaluate!(-self.0, callback, topic.into(), object.into()).map(|_| ())
    }

    /// Evaluate a q expression with no parameters and return a result
    pub fn eval(&self, query: &str) -> Result<KAny, Error> {
        evaluate!(self.0, query)
    }

    /// Evaluate a q function with a single parameter and return the result
    pub fn eval_1(&self, function: &str, param: impl Into<KAny>) -> Result<KAny, Error> {
        evaluate!(self.0, function, param.into())
    }

    /// Evaluate a q function with two parameters and return the result
    pub fn eval_2(&self, function: &str, param: impl Into<KAny>, param_2: impl Into<KAny>) -> Result<KAny, Error> {
        evaluate!(self.0, function, param.into(), param_2.into())
    }

    /// Evaluate a q function with three parameters and return the result
    pub fn eval_3(
        &self,
        function: &str,
        param: impl Into<KAny>,
        param_2: impl Into<KAny>,
        param_3: impl Into<KAny>,
    ) -> Result<KAny, Error> {
        evaluate!(self.0, function, param.into(), param_2.into(), param_3.into())
    }

    /// Evaluate a q function with four parameters and return the result
    pub fn eval_4(
        &self,
        function: &str,
        param: impl Into<KAny>,
        param_2: impl Into<KAny>,
        param_3: impl Into<KAny>,
        param_4: impl Into<KAny>,
    ) -> Result<KAny, Error> {
        evaluate!(
            self.0,
            function,
            param.into(),
            param_2.into(),
            param_3.into(),
            param_4.into()
        )
    }

    /// Evaluate a q function with five parameters and return the result
    pub fn eval_5(
        &self,
        function: &str,
        param: impl Into<KAny>,
        param_2: impl Into<KAny>,
        param_3: impl Into<KAny>,
        param_4: impl Into<KAny>,
        param_5: impl Into<KAny>,
    ) -> Result<KAny, Error> {
        evaluate!(
            self.0,
            function,
            param.into(),
            param_2.into(),
            param_3.into(),
            param_4.into(),
            param_5.into()
        )
    }

    /// Evaluate a q function with six parameters and return the result
    pub fn eval_6(
        &self,
        function: &str,
        param: impl Into<KAny>,
        param_2: impl Into<KAny>,
        param_3: impl Into<KAny>,
        param_4: impl Into<KAny>,
        param_5: impl Into<KAny>,
        param_6: impl Into<KAny>,
    ) -> Result<KAny, Error> {
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
    }

    /// Evaluate a q function with seven parameters and return the result
    pub fn eval_7(
        &self,
        function: &str,
        param: impl Into<KAny>,
        param_2: impl Into<KAny>,
        param_3: impl Into<KAny>,
        param_4: impl Into<KAny>,
        param_5: impl Into<KAny>,
        param_6: impl Into<KAny>,
        param_7: impl Into<KAny>,
    ) -> Result<KAny, Error> {
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
    }

    /// See above and add one parameter.
    pub fn eval_8(
        &self,
        function: &str,
        param: impl Into<KAny>,
        param_2: impl Into<KAny>,
        param_3: impl Into<KAny>,
        param_4: impl Into<KAny>,
        param_5: impl Into<KAny>,
        param_6: impl Into<KAny>,
        param_7: impl Into<KAny>,
        param_8: impl Into<KAny>,
    ) -> Result<KAny, Error> {
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
