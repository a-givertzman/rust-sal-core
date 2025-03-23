#[cfg(test)]

use std::{sync::Once, time::Duration};
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

use crate::{domain::dbg::Dbg, Error};
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
fn new() -> Result<(), Box<dyn std::error::Error>> {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = "test_dbg";
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
            "Nested-2 | \
            \n   └──Nested-1 | Nested-1 raised error",
            {
                Error::pass("Nested-2", Error::new("Nested-1", "Nested-1 raised error"))
            }
        ),
        (
            3,
            "Nested-2 | Nested-2 raised error \
            \n   └──Nested-1 | Nested-1 raised error",
            {
                Error::pass_with("Nested-2", "Nested-2 raised error", Error::new("Nested-1", "Nested-1 raised error"))
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

    struct Entity {
        dbg: Dbg,
    }
    impl Entity {
        pub fn new(parent: impl Into<String>) -> Self {
            Self {
                dbg: Dbg::new(parent.into(), "Entity"),
            }
        }
        pub fn foo(&self) {
            let result: Result<usize, &str> = Ok(173);
            let result: Result<usize, &str> = Err("Was error");
            match result {
                Ok(val) => self.dbg.info("foo", format!("Result: {}", val)),   // "INFO: Parent/Entity | Result: 173"
                Err(err) => self.dbg.warn("foo", format!("Error: {}", err)),   // "WARN: Parent/Entity | Error: Was error"
            }
        }
    }
    let entity = Entity::new(dbg);
    entity.foo();
    let dbg = Dbg::new("test_dbg", "Me");
    dbg.info("new", "Info message");
    dbg.debug("new", "Debug message");
    dbg.warn("new", "Warning message");
    dbg.error("new", "Error message");
    test_duration.exit();
    Ok(())
}
