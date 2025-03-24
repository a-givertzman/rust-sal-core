///
/// for info debug:
/// ```ignore
/// struct Entity {
///     dbg: Dbg,
/// }
/// impl Entity {
///     pub fn new(parent: impl Into<String>) -> Self {
///         Self {
///             dbg: Dbg::new(parent.into(), "Entity"),
///         }
///     }
///     pub fn foo(&self) {
///         let result: Result<usize, &str> = Ok(173);
///         let result: Result<usize, &str> = Err("Was error");
///         match result {
///             Ok(val) => self.dbg.info("foo", format!("Result: {}", val)),   // "INFO: Parent/Entity.foo | Result: 173"
///             Err(err) => self.dbg.warn("foo", format!("Error: {}", err)),   // "WARN: Parent/Entity.foo | Error: Was error"
///         }
///     }
/// }
/// ```
#[derive(Clone)]
pub struct Dbg {
    me: String,
}
//
//
impl Dbg {
    ///
    /// Returns [Dbg] new instance
    /// - `parent` - The parent `Entity`
    /// - `me` - The name of `Entity` to be debuged
    /// - `parent` and `me` => "parent/me"
    pub fn new(parent: impl Into<String>, me: impl Into<String>) -> Self {
        Self {
            me: format!("{}/{}", parent.into(), me.into()),
        }
    }
    ///
    /// Logs a message at the `info` level.
    pub fn info(&self, caller: impl AsRef<str>, msg: impl AsRef<str>) {
        log::info!("{}.{} | {}", self.me, caller.as_ref(), msg.as_ref());
    }
    ///
    /// Logs a message at the `debug` level.
    pub fn debug(&self, caller: impl AsRef<str>, msg: impl AsRef<str>) {
        log::debug!("{}.{} | {}", self.me, caller.as_ref(), msg.as_ref());
    }
    ///
    /// Logs a message at the `warn` level.
    pub fn warn(&self, caller: impl AsRef<str>, msg: impl AsRef<str>) {
        log::warn!("{}.{} | {}", self.me, caller.as_ref(), msg.as_ref());
    }
    ///
    /// Logs a message at the `error` level.
    pub fn error(&self, caller: impl AsRef<str>, msg: impl AsRef<str>) {
        log::error!("{}.{} | {}", self.me, caller.as_ref(), msg.as_ref());
    }
}
//
//
impl std::fmt::Display for Dbg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the Display impl for `&str`:
        write!(f, "{}", self.me)
    }
}
//
//
impl From<Dbg> for String {
    fn from(value: Dbg) -> Self {
        String::from(value.me)
    }
}