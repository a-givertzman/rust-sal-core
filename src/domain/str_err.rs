///
/// 
// #[repr(transparent)]
pub struct StrErr {
    me: String,
    err: Option<String>,
    child: Option<Box<StrErr>>,
}
//
//
impl StrErr {
    ///
    /// Returns [StrErr] new root instance, without child
    /// 
    /// use pass(child) to wrap incoming error
    pub fn new(me: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            me: me.into(),
            err: Some(msg.into()),
            child: None,
        }
    }
    ///
    /// Passes an error with the self name, but without error message 
    pub fn pass(me: impl Into<String>, child: Self) -> Self {
        Self {
            me: me.into(),
            err: None,
            child: Some(Box::new(child)),
        }
    }
    ///
    /// Passes an error with the self name and with the error message 
    pub fn pass_with(me: impl Into<String>, err: impl Into<String>, child: Self) -> Self {
        Self {
            me: me.into(),
            err: Some(err.into()),
            child: Some(Box::new(child)),
        }
    }
}
// Error doesn't require you to implement any methods, but
// your type must also implement Debug and Display.
impl std::error::Error for StrErr {}
//
//
impl std::fmt::Display for StrErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the Display impl for `&str`:
        match (&self.err, &self.child) {
            (Some(err), Some(child)) => write!(f, "{} | {} ->\n\t{}", self.me, err, **child),
            (Some(err), None) => write!(f, "{} | {}", self.me, err),
            (None, Some(child)) => write!(f, "{} | -> \n\t{}", self.me, **child),
            (None, None) => write!(f, "{} |", self.me),
        }
    }
}
//
//
impl std::fmt::Debug for StrErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the Display impl for `&str`:
        std::fmt::Display::fmt(&self, f)
    }
}
