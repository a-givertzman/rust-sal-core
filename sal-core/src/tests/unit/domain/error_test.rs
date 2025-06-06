#[cfg(test)]

use std::{sync::Once, time::Duration};
use sal_core_macros::{err, err_new, err_pass};
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use crate::error::Error;
///
///
static INIT: Once = Once::new();
///
/// once called initialisation
fn init_once() {
    INIT.call_once(|| {
        // implement your initialisation code to be called only once for current test file
    })
}
///
/// returns:
///  - ...
fn init_each() -> () {}
///
/// Testing Error
#[test]
fn pass() -> Result<(), Box<dyn std::error::Error>> {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = "str_err";
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(dbg, Duration::from_secs(10));
    test_duration.run().unwrap();
    let test_data = [
        (
            1,
            "Nested-1 | Nested-1 raised error",
            {
                let dbg = "Nested-1";
                Error::new(dbg, "").err("Nested-1 raised error")
            }
        ),
        (
            2,
            "Nested-2 | \
            \n   └──Nested-1 | Nested-1 raised error",
            {
                Error::new("Nested-2", "").pass(Error::new("Nested-1", "").err("Nested-1 raised error"))
            }
        ),
        (
            3,
            "Nested-2 | Nested-2 raised error \
            \n   └──Nested-1 | Nested-1 raised error",
            {
                Error::new("Nested-2", "").pass_with("Nested-2 raised error", Error::new("Nested-1", "").err("Nested-1 raised error"))
            }
        ),
        (
            4,
            "Root | Root raised error \
            \n   └──Nested-3 | \
            \n      └──Nested-2 | \
            \n         └──Nested-1 | Nested-1 raised error",
            {
                let err = {
                    let err = {
                        let err = {
                            let dbg = "Nested-1";
                            Error::new(dbg, "").err("Nested-1 raised error")
                        };
                        let dbg = "Nested-2";
                        Error::new(dbg, "").pass(err)
                    };
                    let dbg = "Nested-3";
                    Error::new(dbg, "").pass(err)
                };
                let dbg = "Root";
                Error::new(dbg, "").pass_with("Root raised error", err)
            }
        ),
    ];
    for (step, target, err) in test_data {
        let result = err.to_string();
        log::debug!("{}", err);
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    {
        match true {
            true => Ok(()),
            false => Err(Error::new(dbg, "").err("Returns error")),
        }
    }?;
    test_duration.exit();
    Ok(())
}
///
/// Testing error macros
#[test]
fn pass_err_macro() -> Result<(), Box<dyn std::error::Error>> {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg_ = "str_err";
    log::debug!("\n{}", dbg_);
    let test_duration = TestDuration::new(dbg_, Duration::from_secs(10));
    test_duration.run().unwrap();
    let my_struct = MyStruct { dbg: "MyStruct".into() };
    let err = my_struct.show(12);       // Err("MyStruct.show | val: 12")
    log::debug!("err: {:?}", err);
    test_duration.exit();
    Ok(())
}
///
/// For testing only
struct MyStruct {
    dbg: String,    // any type implements Display, the name of this field must be `dbg`
}
impl MyStruct {
    #[err("self.dbg")]
    pub fn show(&self, val: usize) -> Result<(), Error> {
        // Ok(())
        let err = err_new!("Error in {} seconds", val);
        log::debug!("{}", err);
        let err = err_pass!(err);
        log::debug!("{}", err);
        let err = err_pass!(err, "Error in {} seconds", val);
        log::debug!("{}", err);
        Err(err)
    }
}
