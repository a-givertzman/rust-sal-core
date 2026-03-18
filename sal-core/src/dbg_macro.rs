#[macro_export]
macro_rules! define_dbg {
    ( $(#[$meta:meta])* pub struct $name:ident { $( $vis:vis $fname:ident : $fty:ty ),* $(,)? } ) => {
        $(#[$meta])*
        pub struct $name {
            pub dbg: $crate::dbg::Dbg,
            $( $vis $fname : $fty, )*
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name)).field("dbg", &self.dbg).finish()
            }
        }

        impl $name {
            pub fn new(parent: impl Into<String>, $( $fname : $fty ),* ) -> Self {
                Self {
                    dbg: $crate::dbg::Dbg::new(parent, stringify!($name)),
                    $( $fname, )*
                }
            }
            #[inline]
            pub fn scope<'a>(&'a self, area: &'a str) -> $crate::EntityScope<'a> {
                $crate::EntityScope { dbg: &self.dbg, area }
            }
        }
    };
}

// Вспомогательная структура для ошибок (вынесена из макроса для чистоты)
pub struct EntityScope<'a> {
    pub dbg: &'a crate::dbg::Dbg,
    pub area: &'a str,
}

impl<'a> EntityScope<'a> {
    #[allow(dead_code)]
    pub fn err(&self, msg: impl AsRef<str>) -> crate::error::Error {
        crate::error::Error::new(self.dbg, self.area).pass(msg.as_ref())
    }
    #[allow(dead_code)]
    pub fn err_with<E: std::fmt::Display>(&self, msg: impl AsRef<str>, cause: E) -> crate::error::Error {
        let cause_err = crate::error::Error::new(self.dbg, self.area).pass(cause.to_string());
        crate::error::Error::new(self.dbg, self.area).pass_with(msg.as_ref(), cause_err)
    }
    #[allow(dead_code)] pub fn info(&self, msg: impl AsRef<str>) { self.dbg.info(self.area, msg.as_ref()); }
    #[allow(dead_code)] pub fn warn(&self, msg: impl AsRef<str>) { self.dbg.warn(self.area, msg.as_ref()); }
}
