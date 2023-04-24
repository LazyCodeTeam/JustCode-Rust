#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct BucketObjectHead {
    pub name: String,
    pub prefix: String,
    pub mime: String,
    pub size: u64,
}

impl BucketObjectHead {
    pub fn key(&self) -> String {
        format!("{}{}", self.prefix, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        let bucket_object_head = BucketObjectHead {
            name: "name".to_owned(),
            prefix: "prefix/".to_owned(),
            mime: "mime".to_owned(),
            size: 0,
        };

        assert_eq!(bucket_object_head.key(), "prefix/name");
    }
}
