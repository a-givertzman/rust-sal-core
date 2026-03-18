use crate::error::Error;

/// Трейт для записи данных в контекст
pub trait ContextWrite<T> {
    fn write(self, value: T) -> Result<Self, Error> where Self: Sized;
}

/// Трейт для чтения данных по ссылке (для полей типа InitialCtx или Option<TestingCtx>)
pub trait ContextReadRef<T> {
    fn read_ref(&self) -> &T;
}

/// Трейт для чтения данных через клон (для большинства Ctx)
pub trait ContextRead<T> {
    fn read(&self) -> T;
}
