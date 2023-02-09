mod activity;
mod bodies;
mod builder;
mod event;
mod name;

pub use self::activity::ActivityType;
pub use self::activity::ActivityTypes;
pub use self::event::Event;
pub use self::name::EventName;

pub enum Error {
    CannotUseStringDeclaration(EventName),
    DoesNotAcceptType(EventName, ActivityType),
    UnknownEvent(EventName),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        use Error::*;
        match self {
            CannotUseStringDeclaration(evt) => {
                format!("Cannot use string only declaration for {evt}")
            }
            DoesNotAcceptType(evt, act) => format!("{evt} does not accept activity {act}"),
            UnknownEvent(evt) => format!("Possum doesn't know about event {evt}"),
        }
    }
}
