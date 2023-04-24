#[macro_export]
macro_rules! define_repo {
    (
        $v:vis struct $name:ident$(<$($gen:tt),+>)? {
            $($fv:vis $fname:ident: $ft:tt$(<$($fgen:tt),+>)?($($types:ident: $args:ty),*) -> $fout:ty as $fgt:ty),* $(,)?
        }
    ) => {
        common_domain::paste::paste! {
            $(
                define_repo!($fv $fname: $ft$(<$($fgen),+>)?($($types: $args),*) -> $fout as $fgt);
            )*

            $v struct $name$(<$($gen),+>)? {
                $(
                    $fv $fname: [<$fname:camel>]<$fgt>,
                )*
            }
            $(
                define_repo!(@mocks $fname: $ft$(<$($fgen),+>)?($($types: $args),*) -> $fout);
            )*
        }
    };
    ($fv:vis $fname:ident: fn($($types:ident: $args:ty),*) -> $fout:ty as $fgt:ty) => {
        common_domain::paste::paste! {
            $fv trait [<$fname:camel Type>]: std::future::Future<Output = $fout> + std::marker::Send + 'static {}

            impl<T> [<$fname:camel Type>] for T where T: std::future::Future<Output = $fout> + std::marker::Send + 'static {}

            type [<$fname:camel>]<$fgt> = fn($($args),*) -> $fgt;
        }
    };
    ($fv:vis $fname:ident: $ft:tt$(<$($fgen:tt),+>)?($($types:ident: $args:ty),*) -> $fout:ty as $fgt:ty) => {
        common_domain::paste::paste! {
            define_repo!(@type $fv $fname$(<$($fgen),+>)?);

            type [<$fname:camel>]<$fgt> = $fgt;

            $fv trait [<_ $fname:camel>]$(<$($fgen),+>)?: $ft($($args),*) -> Self::OutputFuture {
                type OutputFuture: std::future::Future<Output = $fout>;
            }

            impl<$($($fgen),+,)?F, FUT> [<_ $fname:camel>]$(<$($fgen),+>)? for F
            where
                F: $ft($($args),*) -> FUT,
                FUT: std::future::Future<Output = $fout>,
            {
                type OutputFuture = FUT;
            }
        }
    };
    (@type $vf:vis $fname:ident$(<$($lt:lifetime),+>)?) => {
        common_domain::paste::paste! {
            $vf trait [<$fname:camel Type>]: $(for<$($lt),+>)? [<_ $fname:camel>]$(<$($lt),+>)? {}

            impl<T> [<$fname:camel Type>] for T where T: $(for<$($lt),+>)? [<_ $fname:camel>]$(<$($lt),+>)? {}
        }

    };

    (@type $vf:vis $fname:ident<$($lt:lifetime),+,$($gen:ident),+>) => {
        common_domain::paste::paste! {
            $vf trait [<$fname:camel Type>]<$($gen),+>: for<$($lt),+> [<_ $fname:camel>]<$($lt),+, $($gen),+> {}

            impl<T> [<$fname:camel Type>]<$($gen),+> for T where T: for<$($lt),+> [<_ $fname:camel>]<$($lt),+, $($gen),+> {}
        }

    };
    (@type $vf:vis $fname:ident<$($gen:ident),+>) => {
        common_domain::paste::paste! {
            $vf trait [<$fname:camel Type>]<$($gen),+>: [<_ $fname:camel>]<$($gen),+> {}

            impl<T> [<$fname:camel Type>]<$($gen),+> for T where T: [<_ $fname:camel>]<$($gen),+> {}
        }

    };
    (@mocks $fname:ident: $ft:tt$(<$($lt:lifetime),+>)?($($types:ident: $args:ty),*) -> $fout:ty) => {
        common_domain::paste::paste! {
            #[cfg(test)]
            mod [<mock_ $fname>] {
                use super::*;
                common_domain::lazy_static::lazy_static! {
                    static ref [<$fname:snake:upper>]: common_domain::tokio::sync::Mutex<()> = common_domain::tokio::sync::Mutex::new(());
                }

                pub use [<mock_ $fname:snake _internal>]::call;

                pub async fn ctx() -> ([<mock_ $fname:snake _internal>]::__call::Context, common_domain::tokio::sync::MutexGuard<'static, ()>) {
                    let lock = [<$fname:snake:upper>].lock().await;
                    let ctx = [<mock_ $fname:snake _internal>]::call_context();
                    (ctx, lock)

                }

                #[mockall::automock]
                mod [<$fname:snake _internal>] {
                    use super::*;

                    pub async fn call$(<$($lt),+>)?($([<_ $types>]: $args),*) -> $fout {
                        unimplemented!()
                    }
                }
            }
        }
    };
    (@mocks $fname:ident: $ft:tt$(<$($gen:tt),+>)?($($types:ident: $args:ty),*) -> $fout:ty) => {};
}
