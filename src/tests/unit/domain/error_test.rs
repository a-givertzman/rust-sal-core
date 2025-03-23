#[cfg(test)]

use std::{sync::Once, time::Duration};
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

use crate::Error;
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
/// Testing such functionality / behavior
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
                Error::new(dbg, "Nested-1 raised error")
            }
        ),
        (
            2,
            "Nested-2 | \n   └──Nested-1 | Nested-1 raised error",
            {
                Error::pass("Nested-2", Error::new("Nested-1", "Nested-1 raised error"))
            }
        ),
        (
            3,
            "Nested-2 | Nested-2 raised error \n   └──Nested-1 | Nested-1 raised error",
            {
                Error::pass_with("Nested-2", "Nested-2 raised error", Error::new("Nested-1", "Nested-1 raised error"))
            }
        ),
        (
            4,
            "Root | Root raised error \n   └──Nested-3 | \n      └──Nested-2 | \n         └──Nested-1 | Nested-1 raised error",
            {
                let err = {
                    let err = {
                        let err = {
                            let dbg = "Nested-1";
                            Error::new(dbg, "Nested-1 raised error")
                        };
                        let dbg = "Nested-2";
                        Error::pass(dbg, err)
                    };
                    let dbg = "Nested-3";
                    Error::pass(dbg, err)
                };
                let dbg = "Root";
                Error::pass_with(dbg, "Root raised error", err)
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
            false => Err(Error::new(dbg, "Returns error")),
        }
    }?;
    test_duration.exit();
    Ok(())
}
