#[macro_export]
macro_rules! generate_mapper_traits {
    () => {
        pub trait FromDto<T> {
            fn from_dto(dto: T) -> Self;
        }

        pub trait IntoModel<T> {
            fn into_model(self) -> T;
        }

        pub trait IntoDto<T> {
            fn into_dto(self) -> T;
        }

        pub trait FromModel<T> {
            fn from_model(model: T) -> Self;
        }

        impl<T, U> IntoModel<U> for T
        where
            U: FromDto<T>,
        {
            fn into_model(self) -> U {
                U::from_dto(self)
            }
        }

        impl<T, U> IntoDto<U> for T
        where
            U: FromModel<T>,
        {
            fn into_dto(self) -> U {
                U::from_model(self)
            }
        }
    };
}
