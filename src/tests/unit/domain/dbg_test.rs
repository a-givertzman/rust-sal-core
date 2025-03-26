#[cfg(test)]

use std::{sync::Once, time::Duration};
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use crate::domain::dbg::Dbg;
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
/// Testing `new()`
#[test]
fn new() -> Result<(), Box<dyn std::error::Error>> {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = "test_dbg";
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
    test_duration.run().unwrap();
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
            for result in [Ok(173), Err("Was error")] {
                match result {
                    Ok(val) => self.dbg.info("foo", format!("Result: {}", val)),   // "INFO: Parent/Entity | Result: 173"
                    Err(err) => self.dbg.warn("foo", format!("Error: {}", err)),   // "WARN: Parent/Entity | Error: Was error"
                }
            }
        }
    }
    let entity = Entity::new(dbg);
    entity.foo();
    let dbg = Dbg::new("test_dbg", "Me");
    let fn_name = "new";
    dbg.info(fn_name, "Info message");
    dbg.debug(fn_name, "Debug message");
    dbg.warn(fn_name, "Warning message");
    dbg.error(fn_name, "Error message");
    test_duration.exit();
    Ok(())
}
///
/// Testing `to_string()`
#[test]
fn to_string() -> Result<(), Box<dyn std::error::Error>> {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = "test_dbg";
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
    test_duration.run().unwrap();
    let test_data = [
        (1, format!("{dbg}/"), Dbg::new(dbg, "")),
        (1, format!("/Me"), Dbg::new("", "Me")),
        (1, format!("{dbg}/Me"), Dbg::new(dbg, "Me")),
    ];
    for (step, target, value) in test_data {
        let result = value.to_string();
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
    Ok(())
}

mod dbg {
    macro_rules! dbg {
        () => {{
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            &name[..name.len() - 3]
        }}
    }
}