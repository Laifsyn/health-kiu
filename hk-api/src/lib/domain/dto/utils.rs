macro_rules! id_wrapper {
    (
        $(#[$m:meta])*
        $vis:vis struct $name:ident($field_vis:vis $inner:ty)
    ) => {
        $(#[$m])*
        #[repr(transparent)]
        $vis struct $name($field_vis $inner);

        impl $name {
            #[allow(dead_code)]
            pub fn from_inner(inner: $inner) -> Self {
                $name(inner)
            }

            #[allow(dead_code)]
            pub fn from_inner_ref(inner: &$inner) -> &Self {
                unsafe {
                    std::mem::transmute::<&$inner, &$name>(inner)
                }
            }

            #[allow(dead_code)]
            pub fn from_inner_mut(inner: &mut $inner) -> &mut Self {
                unsafe {
                    std::mem::transmute::<&mut $inner, &mut $name>(inner)
                }
            }

            #[allow(dead_code)]
            pub fn try_from_inner<T>(inner: T) -> Result<Self, T::Error>
            where
                T: TryInto<$inner>,
            {
                Ok(Self(inner.try_into()?))
            }

            #[allow(dead_code)]
            pub fn into_inner(self) -> $inner {
                self.0
            }

            #[allow(dead_code)]
            pub fn as_inner_ref(&self) -> &$inner {
                &self.0
            }

            #[allow(dead_code)]
            pub fn as_inner_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::convert::From<$name> for $inner {
            fn from(id_wrapper: $name) -> Self {
                id_wrapper.into_inner()
            }
        }
    };
}

pub(crate) use id_wrapper;
