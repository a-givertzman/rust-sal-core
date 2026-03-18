// Макрос для контекста
#[macro_export]
macro_rules! define_context {
    (
        pub struct $name:ident {
            parameters: $pt:ty,
            $( $(@[$m:ident])? $f:ident : $t:ty ),* $(,)?
        }
    ) => {
        #[derive(Clone)] 
        pub struct $name {
            pub parameters: $pt,
            $( pub $f: $crate::impl_field_type!( $(@[$m])? $t ), )*
        }

        // Автоматическая реализация для поля parameters
        impl $crate::ContextReadRef<$pt> for $name {
            fn read_ref(&self) -> &$pt {
                &self.parameters
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    parameters: <$pt>::default(),
                    $( $f: Default::default(), )*
                }
            }
        }

        $( $crate::impl_context_traits!($name, $f, $t $(, @[$m])? ); )*
    };
}

#[macro_export]
macro_rules! impl_field_type {
    (@[raw] $t:ty) => { $t };
    (@[opt_ref] $t:ty) => { Option<$t> };
    ($t:ty) => { Option<$t> };
}

#[macro_export]
macro_rules! impl_context_traits {
    ($ctx:ident, $f:ident, $t:ty, @[raw]) => {
        impl $crate::ContextWrite<$t> for $ctx {
            fn write(mut self, v: $t) -> Result<Self, $crate::error::Error> { self.$f = v; Ok(self) }
        }
        impl $crate::ContextReadRef<$t> for $ctx {
            fn read_ref(&self) -> &$t { &self.$f }
        }
    };
    ($ctx:ident, $f:ident, $t:ty, @[opt_ref]) => {
        impl $crate::ContextWrite<$t> for $ctx {
            fn write(mut self, v: $t) -> Result<Self, $crate::error::Error> { self.$f = Some(v); Ok(self) }
        }
        impl $crate::ContextReadRef<Option<$t>> for $ctx {
            fn read_ref(&self) -> &Option<$t> { &self.$f }
        }
    };
    ($ctx:ident, $f:ident, $t:ty) => {
        impl $crate::ContextWrite<$t> for $ctx {
            fn write(mut self, v: $t) -> Result<Self, $crate::error::Error> { self.$f = Some(v); Ok(self) }
        }
        impl $crate::ContextRead<$t> for $ctx {
            fn read(&self) -> $t { self.$f.clone().expect(stringify!($f)) }
        }
    };
}

