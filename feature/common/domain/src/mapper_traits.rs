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

        impl<T, U> FromModel<Vec<T>> for Vec<U>
        where
            U: FromModel<T>,
        {
            fn from_model(models: Vec<T>) -> Self {
                models.into_iter().map(|m| m.into_dto()).collect()
            }
        }

        impl<T, U> FromModel<Option<T>> for Option<U>
        where
            U: FromModel<T>,
        {
            fn from_model(model: Option<T>) -> Self {
                model.map(|m| m.into_dto())
            }
        }

        impl<T, U> FromDto<Vec<T>> for Vec<U>
        where
            U: FromDto<T>,
        {
            fn from_dto(dtos: Vec<T>) -> Self {
                dtos.into_iter().map(|m| m.into_model()).collect()
            }
        }

        impl<T, U> FromDto<Option<T>> for Option<U>
        where
            U: FromDto<T>,
        {
            fn from_dto(dto: Option<T>) -> Self {
                dto.map(|m| m.into_model())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    generate_mapper_traits!();

    #[derive(Debug, PartialEq)]
    struct Dto;

    #[derive(Debug, PartialEq)]
    struct Model;

    impl FromDto<Dto> for Model {
        fn from_dto(_dto: Dto) -> Self {
            Model
        }
    }

    impl FromModel<Model> for Dto {
        fn from_model(_model: Model) -> Self {
            Dto
        }
    }

    #[test]
    fn into_model() {
        let dto = Dto;
        let model: Model = dto.into_model();
        assert_eq!(model, Model);
    }

    #[test]
    fn into_dto() {
        let model = Model;
        let dto: Dto = model.into_dto();
        assert_eq!(dto, Dto);
    }

    #[test]
    fn into_model_vec() {
        let dtos = vec![Dto, Dto];
        let models: Vec<Model> = dtos.into_model();
        assert_eq!(models, vec![Model, Model]);
    }

    #[test]
    fn into_dto_vec() {
        let models = vec![Model, Model];
        let dtos: Vec<Dto> = models.into_dto();
        assert_eq!(dtos, vec![Dto, Dto]);
    }

    #[test]
    fn into_model_option() {
        let dto = Some(Dto);
        let model: Option<Model> = dto.into_model();
        assert_eq!(model, Some(Model));
    }

    #[test]
    fn into_dto_option() {
        let model = Some(Model);
        let dto: Option<Dto> = model.into_dto();
        assert_eq!(dto, Some(Dto));
    }
}
