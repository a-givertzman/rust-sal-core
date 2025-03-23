///
/// 
pub struct StrErr {
    name: String,
    msg: String,
    child: Option<Box<StrErr>>,
}
//
//
impl StrErr {
    ///
    /// Returns [StrErr] new root instance, without child
    /// 
    /// use pass(child) to wrap incoming error
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            child: None,
        }
    }
    ///
    /// 
    pub fn pass(child: Self) -> Self {
        Self {
            msg: String::new(),
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
        self.0.fmt(f)
    }
}
impl std::fmt::Debug for StrErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the Display impl for `&str`:
        std::fmt::Display::fmt(&self, f)
    }
}
