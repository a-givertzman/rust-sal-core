///
/// for info debug:
/// ```ignore
/// struct Entity {
///     dbg: Dbg,
/// }
/// impl Entity {
///     pub fn new(parent: impl Into<String>) -> Self {
///         let name = format!("{}/Entity", parent);
///         Self {
///             dbg: Dbg::new(name) 
///         }
///     }
///     pub fn foo(&self) {
///         let result = Ok(173);
///         let result = Err("Was error");
///         match result {
///             Ok(val) => self.dbg.info(format!("Result: {}", val)),   // "INFO: Parent/Entity | Result: 173"
///             Err(err) => self.dbg.warn(format!("Error: {}", err)),   // "WARN: Parent/Entity | Error: Was error"
///         }
///     }
/// }
/// ```
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
    pub fn debug(&self, msg: impl AsRef<str>) {
        log::debug!("{} | {}", self.me, msg.as_ref());
    }
    ///
    /// Logs a message at the `warn` level.
    pub fn warn(&self, msg: impl AsRef<str>) {
        log::warn!("{} | {}", self.me, msg.as_ref());
    }
    ///
    /// Logs a message at the `error` level.
    pub fn error(&self, msg: impl AsRef<str>) {
        log::error!("{} | {}", self.me, msg.as_ref());
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
        todo!()
    }
}