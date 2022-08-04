#[macro_export]
/// Produces documentation for utility enumerations.
macro_rules! utility_enum {
    ($($tree:tt)+) => {
        #[derive(Debug, Clone)]
        #[doc="An enumeration used for utility purposes. You shouldn't ever need to construct this"]
        #[doc="directly. Instead you should use one of its member types. These all implement `Into`"]
        #[doc="for this type. You can then pass them to a method which accepts and type `T` such"]
        #[doc="that `T` satisfies `Into` the type of this utility enum (this is an automatically"]
        #[doc="generated doc comment which is why it doesn't say `Into<type of this enum>`.)"]
        #[doc="\n"]
        $($tree)+
    }
}
