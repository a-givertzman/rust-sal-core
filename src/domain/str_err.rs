///
/// Represents a nested error sequence,
/// - As string looks like
/// ```log
/// Root | Root raised error 
///     └──Nested-3 | 
///        └──Nested-2 | 
///           └──Nested-1 | Nested-1 raised error
/// ```
// #[repr(transparent)]
pub struct Error {
    me: String,
    msg: Option<String>,
    child: Option<Box<Error>>,
}
//
//
impl Error {
    ///
    /// Returns [StrErr] new root instance, without child
    /// 
    /// use pass(child) to wrap incoming error
    pub fn new(me: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            me: me.into(),
            msg: Some(msg.into()),
            child: None,
        }
    }
    ///
    /// Passes an error with the self name, but without error message 
    pub fn pass(me: impl Into<String>, child: Self) -> Self {
        Self {
            me: me.into(),
            msg: None,
            child: Some(Box::new(child)),
        }
    }
    ///
    /// Passes an error with the self name and with the error message 
    pub fn pass_with(me: impl Into<String>, msg: impl Into<String>, child: Self) -> Self {
        Self {
            me: me.into(),
            msg: Some(msg.into()),
            child: Some(Box::new(child)),
        }
    }
    //
    // Combines all nested errors into single string
    fn join(&self, depth: usize) -> String {
        let tab = "   ".repeat(depth + 1);
        match (&self.msg, &self.child) {
            (Some(err), Some(child)) => format!("{} | {} \n{tab}└──{}", self.me, err, child.join(depth + 1)),
            (Some(err), None) => format!("{} | {}", self.me, err),
            (None, Some(child)) => format!("{} | \n{tab}└──{}", self.me, child.join(depth + 1)),
            (None, None) => format!("{}", self.me),
        }
    }
}
// Error doesn't require you to implement any methods, but
// your type must also implement Debug and Display.
impl std::error::Error for Error {}
//
//
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the Display impl for `&str`:
        write!(f, "{}", self.join(0))
    }
}
//
//
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the Display impl for `&str`:
        std::fmt::Display::fmt(&self, f)
    }
}
