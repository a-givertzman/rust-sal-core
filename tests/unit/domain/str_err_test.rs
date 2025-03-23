#[cfg(test)]

mod str_err {
    use std::{sync::Once, time::{Duration, Instant}};
    use sal_core::StrErr;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
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
        let dbg = "str_err";
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let err = StrErr::new(dbg, "first error");
        log::debug!("{}", err);
        let err = StrErr::pass(dbg, err);
        log::debug!("{}", err);
        let err = StrErr::pass_with(dbg, "Locally raised error", err);
        log::debug!("{}", err);
        {
            Err(StrErr::new(dbg, "Returns error"))
        }?;
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
        Ok(())
    }
}
