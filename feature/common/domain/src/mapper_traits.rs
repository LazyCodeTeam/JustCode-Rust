#[macro_export]
macro_rules! generate_mapper_traits {
    () => {
        pub trait MapFrom<T> {
            fn map_from(value: T) -> Self;
        }

        pub trait MapInto<T> {
            fn map_into(self) -> T;
        }

        impl<T, U> MapInto<U> for T
        where
            U: MapFrom<T>,
        {
            fn map_into(self) -> U {
                U::map_from(self)
            }
        }

        impl<T, U> MapFrom<Vec<T>> for Vec<U>
        where
            U: MapFrom<T>,
        {
            fn map_from(value: Vec<T>) -> Self {
                value.into_iter().map(MapInto::map_into).collect()
            }
        }

        impl<T, U> MapFrom<Option<T>> for Option<U>
        where
            U: MapFrom<T>,
        {
            fn map_from(value: Option<T>) -> Self {
                value.map(MapInto::map_into)
            }
        }

        pub trait TryMapFrom<T>: Sized {
            type Error;

            fn try_map_from(value: T) -> Result<Self, Self::Error>;
        }

        pub trait TryMapInto<T>: Sized {
            type Error;

            fn try_map_into(self) -> Result<T, Self::Error>;
        }

        impl<T, U> TryMapInto<U> for T
        where
            U: TryMapFrom<T>,
        {
            type Error = U::Error;

            fn try_map_into(self) -> Result<U, Self::Error> {
                U::try_map_from(self)
            }
        }
    };
}

#[cfg(test)]
mod test {
    generate_mapper_traits!();

    impl MapFrom<i32> for String {
        fn map_from(value: i32) -> Self {
            value.to_string()
        }
    }

    #[test]
    fn test_map_from() {
        let x = 1;
        let y = String::map_from(x);
        assert_eq!(y, "1".to_string());
    }

    #[test]
    fn test_map_into() {
        let x = 1;
        let y: String = x.map_into();
        assert_eq!(y, "1".to_string());
    }

    #[test]
    fn test_map_from_vec() {
        let x = vec![1, 2, 3];
        let y: Vec<String> = x.map_into();
        assert_eq!(y, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    fn test_map_from_option() {
        let x = Some(1);
        let y: Option<String> = x.map_into();
        assert_eq!(y, Some("1".to_string()));
    }

    #[test]
    fn test_map_from_option_none() {
        let x: Option<i32> = None;
        let y: Option<String> = x.map_into();
        assert_eq!(y, None::<String>);
    }

    impl TryMapFrom<i32> for String {
        type Error = ();

        fn try_map_from(value: i32) -> Result<Self, Self::Error> {
            Ok(value.to_string())
        }
    }

    #[test]
    fn test_try_map_from() {
        let x = 1;
        let y: Result<String, ()> = x.try_map_into();

        assert_eq!(y, Ok("1".to_owned()));
    }
}
