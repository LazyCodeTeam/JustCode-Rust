#[macro_export]
macro_rules! define_port {
    ($id:ident = $f:tt<$($gen:tt),+>($($types:ident: $args:ty),*) -> $($out:tt)+) => {
        paste::paste! {
            pub trait $id<$($gen),+>: $f($($args),*) -> Self::OutputFuture {
                type OutputFuture: std::future::Future<Output = $($out)+>;
            }

            impl<$($gen),+, F, FUT> $id<$($gen),+> for F
            where
                F: $f($($args),*) -> FUT,
                FUT: std::future::Future<Output = $($out)+>,
            {
                type OutputFuture = FUT;
            }


            #[mockall::automock]
            pub mod [<$id:snake>] {
                use super::*;

                pub async fn call<$($gen),+>($([<_ $types>]: $args),*) -> $($out)+ {
                    unimplemented!()
                }
            }
        }
    };
}
