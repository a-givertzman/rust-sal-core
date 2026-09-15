use std::sync::Arc;

///
/// for info debug:
/// ```ignore
/// struct Entity {
///     dbg: Dbg,
/// }
/// impl Entity {
///     pub fn new(parent: impl AsRef<str>) -> Self {
///         Self {
///             dbg: Dbg::new(parent, "Entity"),
///         }
///     }
///     pub fn foo(&self) {
///         let result: Result<usize, &str> = Ok(173);
///         let result: Result<usize, &str> = Err("Was error");
///         match result {
///             Ok(val) => self.dbg.info("foo", format!("Result: {}", val)),   // "INFO: Parent/Entity.foo | Result: 173"
///             Err(err) => self.dbg.warn("foo", format!("Error: {}", err)),   // "WARN: Parent/Entity.foo | Error: Was error"
///         }
///         match result {
///             Ok(val) => log::info!("{dbg}.foo | Result: {:?}", val),        // "INFO: Parent/Entity.foo | Result: 173"
///             Err(err) => log::warn!("{dbg}.foo | Error: {:?}", err),       // "WARN: Parent/Entity.foo | Error: Was error"
///         }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dbg {
    me: Arc<str>,
}
//
//
impl Dbg {
    ///
    /// Returns [Dbg] new instance
    /// - `parent` - The parent `Entity`
    /// - `me` - The name of `Entity` to be debuged
    /// - `parent` and `me` => "parent/me"
    pub fn new(parent: impl AsRef<str>, me: impl AsRef<str>) -> Self {
        let parent = parent.as_ref();
        let me = if parent.is_empty() {
            Arc::from(format!("/{}", me.as_ref()))
        } else  {
            Arc::from(format!("{}/{}", parent, me.as_ref()))
        };
        Self { me }
    }
    ///
    /// Returns [Dbg] new instance without parent
    /// - `me` - The name of `Entity` to be debuged
    pub fn own(me: impl AsRef<str>) -> Self {
        Self {
            me: me.as_ref().into(),
        }
    }
    ///
    /// Logs a message at the `info` level.
    /// - `area` - code block from which message `msg` will be logged
    pub fn info(&self, area: impl AsRef<str>, msg: impl AsRef<str>) {
        log::info!("{}.{} | {}", self.me, area.as_ref(), msg.as_ref());
    }
    ///
    /// Logs a message at the `debug` level.
    /// - `area` - code block from which message `msg` will be logged
    pub fn debug(&self, area: impl AsRef<str>, msg: impl AsRef<str>) {
        log::debug!("{}.{} | {}", self.me, area.as_ref(), msg.as_ref());
    }
    ///
    /// Logs a message at the `warn` level.
    /// - `area` - code block from which message `msg` will be logged
    pub fn warn(&self, area: impl AsRef<str>, msg: impl AsRef<str>) {
        log::warn!("{}.{} | {}", self.me, area.as_ref(), msg.as_ref());
    }
    ///
    /// Logs a message at the `error` level.
    /// - `area` - code block from which message `msg` will be logged
    pub fn error(&self, area: impl AsRef<str>, msg: impl AsRef<str>) {
        log::error!("{}.{} | {}", self.me, area.as_ref(), msg.as_ref());
    }
}
//
impl Default for Dbg {
    fn default() -> Self {
        Self { me: Arc::from(String::default()) }
    }
}
//
impl std::fmt::Display for Dbg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.me)
    }
}
//
//
impl std::fmt::Debug for Dbg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.me)
    }
}
//
//
impl From<Dbg> for String {
    fn from(value: Dbg) -> Self {
        String::from(&*value.me)
    }
}
//
//
impl From<&Dbg> for String {
    fn from(value: &Dbg) -> Self {
        String::from(&*value.me)
    }
}
//
//
impl AsRef<str> for Dbg {
    fn as_ref(&self) -> &str {
        &self.me
    }
}
