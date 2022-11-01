use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct JsResponseDto {
    pub js: String,
}

impl From<String> for JsResponseDto {
    fn from(content: String) -> Self {
        JsResponseDto { js: content }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from() {
        let content = "main() {console.log('test');}".to_owned();

        let result = JsResponseDto::from(content.clone());

        assert_eq!(result.js, content);
    }
}
