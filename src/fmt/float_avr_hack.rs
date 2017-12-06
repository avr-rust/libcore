use fmt::{Formatter, Result, Display, Debug, UpperExp, LowerExp};

macro_rules! floating {
    ($ty:ident) => (
        #[stable(feature = "rust1", since = "1.0.0")]
        impl Debug for $ty {
            fn fmt(&self, fmt: &mut Formatter) -> Result {
                unimplemented!();
            }
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl Display for $ty {
            fn fmt(&self, fmt: &mut Formatter) -> Result {
                unimplemented!();
            }
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl LowerExp for $ty {
            fn fmt(&self, fmt: &mut Formatter) -> Result {
                unimplemented!();
            }
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl UpperExp for $ty {
            fn fmt(&self, fmt: &mut Formatter) -> Result {
                unimplemented!();
            }
        }
    )
}

floating! { f32 }
floating! { f64 }
