///
/// Represents a nested error sequence,
/// - As a string may looks like
/// ```log
/// Root | Root raised error 
///     └──Nested-3 | 
///        └──Nested-2 | 
///           └──Nested-1 | Nested-1 raised error
/// ```
#[derive(Clone)]
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
    /// - `me` - Module/Class/Test where error was raised
    /// - `area` - Method where error was raised
    /// 
    /// Note
    /// - use `err(msg)` - To create error with message and specified `me` & `area`
    /// - use pass(child) to wrap incoming error
    pub fn new(me: impl Into<String>, area: impl Into<String>) -> Self {
        let area = area.into();
        Self {
            me: format!(
                "{}{}",
                me.into(),
                if area.is_empty() {String::new()} else {format!(".{area}")},
            ),
            msg: None,
            child: None,
        }
    }
    ///
    /// Passes received error 
    /// - `err` - Incoming error to be passed
    pub fn pass(&self, err: impl Into<Self>) -> Self {
        Self {
            me: self.me.clone(),
            msg: None,
            child: Some(Box::new(err.into())),
        }
    }
    ///
    /// Passes incoming error with additional message
    /// - `msg` - Additional message
    /// - `err` - Incoming error to be passed
    pub fn pass_with(&self, msg: impl Into<String>, err: impl Into<Self>) -> Self {
        Self {
            me: self.me.clone(),
            msg: Some(msg.into()),
            child: Some(Box::new(err.into())),
        }
    }
    ///
    /// Returns [Error] new instance with the message
    pub fn err(&self, msg: impl Into<String>) -> Self {
        Self {
            me: self.me.clone(),
            msg: Some(msg.into()),
            child: None,
        }
    }
    //
    // Combines all nested errors into single string
    fn join(&self, depth: usize) -> String {
        let tab = "   ".repeat(depth + 1);
        match (&self.msg, &self.child) {
            (Some(msg), Some(child)) => format!("{} | {} \n{tab}└──{}", self.me, msg, child.join(depth + 1)),
            (Some(msg), None) => format!("{} | {}", self.me, msg),
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
//
//
impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::new("", "").err(msg)
    }
}
//
//
impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::new("", "").err(msg)
    }
}
