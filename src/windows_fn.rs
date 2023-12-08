pub trait WindowsFnPtr: private::SealedWindowsFnPtr {
    const ASSERT_PTR_SIZE: () = [()][core::mem::size_of::<usize>()];
}

mod private {
    pub trait SealedWindowsFnPtr: Copy {}
}

impl<F: private::SealedWindowsFnPtr> WindowsFnPtr for F {}

#[repr(transparent)]
pub struct WindowsFn<F>(pub F);

impl<F: Copy> WindowsFn<F> {
    pub unsafe fn as_fn(&self) -> F {
        self.0
    }
}

impl<F: WindowsFnPtr> From<*const ()> for WindowsFn<F> {
    fn from(value: *const ()) -> Self {
        let _ = F::ASSERT_PTR_SIZE;
        unsafe { core::mem::transmute_copy(&value) }
    }
}

macro_rules! impl_unsafe_fn {
    (@recurse $first:ident $( , $rest:ident )*) => {
        impl_unsafe_fn!($( $rest ),*);
    };

    (@recurse) => {};

    ($( $param:ident ),*) => {
        impl<Output, $( $param ),*> private::SealedWindowsFnPtr for unsafe extern "C" fn($( $param ),*) -> Output {}

        impl<Output, $( $param ),*> WindowsFn<unsafe extern "C" fn($( $param ),*) -> Output> {
            #[allow(non_snake_case)]
            #[inline(always)]
            pub unsafe fn call(&self, $( $param: $param ),*) -> Output {
                (self.0)($( $param ),*)
            }
        }

        impl_unsafe_fn!(@recurse $( $param ),*);
    };
}

impl_unsafe_fn!(A, B, C, D, E, F, G, H, I, J, K, L, M);
