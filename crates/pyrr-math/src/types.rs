use std::mem::{size_of, transmute_copy, ManuallyDrop};

// macro_rules! count_tts {
//     () => { 0 };
//     ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
//     ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
// }

macro_rules! decl_vector {{
    $(#[$type_meta:meta])*
    $vis:vis struct $Type:ident {$(
        $(#[$field_meta:meta])*
        $field:ident
    ),* $(,)?}
} => {
    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    $(#[$type_meta])*
    $vis struct $Type<T> {$(
        $(#[$field_meta])*
        $vis $field: T,
    )*}

    impl<T> $Type<T> {
        pub const fn new($($field: T),*) -> Self {
            Self { $($field),* }
        }
    }
}}

decl_vector! {
    /// A 2-dimensional vector.
    ///
    /// | coordinate space | conventional `x` | conventional `y` |
    /// | ---------------- | ---------------- | ---------------- |
    /// | side-on          | forward (right)  | up               |
    /// | top-down         | east (right)     | north (up)       |
    /// | texture space    | right            | down             |
    pub struct Vector2 { x, y }
}

decl_vector! {
    /// A 3-dimensional vector.
    ///
    /// | coordinate space | conventional `x` | conventional `y` | conventional `z` |
    /// | ---------------- | ---------------- | ---------------- | ---------------- |
    /// | local space      | forward          | left             | up               |
    /// | world space      | north            | west             | up               |
    /// | view space       | right            | up               | closer           |
    /// | clip space       | right ∈ [-1, 1]  | up ∈ [-1, 1]     | further ∈ [0, 1] |
    ///
    /// <div class="warning">Note that while Pyrr uses a right-handed coordinate system,
    /// rendering ends up left-handed due to the use of a reverse-z depth. However, this
    /// should only ever matter when doing clip-space work in shaders.</div>
    pub struct Vector3 { x, y, z }
}

macro_rules! impl_vector {{
    const $N:ident: usize;
    type $Vector:ident;
    $($items:item)*
} => {
    const _: () = {
        const $N: usize = 2;
        type $Vector<T> = Vector2<T>;
        $($items)*
    };
    const _: () = {
        const $N: usize = 3;
        type $Vector<T> = Vector3<T>;
        $($items)*
    };
}}

impl_vector!(
    const N: usize;
    type Vector;

    impl<T> Vector<T> {
        #[allow(unsafe_code)]
        pub const fn as_array(&self) -> &[T; N] {
            // SAFETY: Vector is repr(C) with N fields of type T
            unsafe { &*(self as *const Self).cast() }
        }

        #[allow(unsafe_code)]
        pub fn as_mut_array(&mut self) -> &mut [T; N] {
            // SAFETY: Vector is repr(C) with N fields of type T
            unsafe { &mut *(self as *mut Self).cast() }
        }

        pub const fn as_slice(&self) -> &[T] {
            self.as_array().as_slice()
        }

        pub fn as_mut_slice(&mut self) -> &mut [T] {
            self.as_mut_array()
        }
    }

    impl<T> From<[T; N]> for Vector<T> {
        #[allow(unsafe_code)]
        fn from(v: [T; N]) -> Self {
            assert_eq!(size_of::<Vector<T>>(), size_of::<[T; N]>());
            // SAFETY: Vector is repr(C) with N fields of type T
            unsafe { transmute_copy(&ManuallyDrop::new(v)) }
        }
    }

    impl<T> From<Vector<T>> for [T; N] {
        #[allow(unsafe_code)]
        fn from(v: Vector<T>) -> Self {
            assert_eq!(size_of::<Vector<T>>(), size_of::<[T; N]>());
            // SAFETY: Vector is repr(C) with N fields of type T
            unsafe { transmute_copy(&ManuallyDrop::new(v)) }
        }
    }

    impl<'a, T> From<&'a Vector<T>> for &'a [T; N] {
        fn from(v: &'a Vector<T>) -> Self {
            v.as_array()
        }
    }

    impl<'a, T> From<&'a mut Vector<T>> for &'a mut [T; N] {
        fn from(v: &'a mut Vector<T>) -> Self {
            v.as_mut_array()
        }
    }

    impl<T> AsRef<[T; N]> for Vector<T> {
        #[allow(unsafe_code)]
        fn as_ref(&self) -> &[T; N] {
            self.as_array()
        }
    }

    impl<T> AsMut<[T; N]> for Vector<T> {
        fn as_mut(&mut self) -> &mut [T; N] {
            self.as_mut_array()
        }
    }

    impl<T> From<Vector<T>> for glm::TVec<T, N> {
        fn from(v: Vector<T>) -> Self {
            glm::TVec::from_array_storage(na::base::ArrayStorage([v.into(); 1]))
        }
    }

    impl<T> From<glm::TVec<T, N>> for Vector<T> {
        fn from(v: glm::TVec<T, N>) -> Self {
            let (a,) = v.data.0.into();
            a.into()
        }
    }
);

/// A 2-dimensional vector of `f32`s.
pub type Vec2f = Vector2<f32>;
