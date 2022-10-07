#[macro_export]
macro_rules! define_port {
    ($id:ident = $f:tt<$($gen:tt),+>($($args:tt)*) -> $($out:tt)+) => {
        pub trait $id<$($gen),+>: $f($($args)*) -> Self::OutputFuture {
            type OutputFuture: std::future::Future<Output = $($out)+>;
        }

        impl<$($gen),+, F, FUT> $id<$($gen),+> for F
        where
            F: $f($($args)*) -> FUT,
            FUT: std::future::Future<Output = $($out)+>,
        {
            type OutputFuture = FUT;
        }
    };
}
