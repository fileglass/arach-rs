#![feature(async_stream)]
#[cfg(test)]
pub mod test {
    use tokio::fs::File;
    use crate::Arachnid;
    use tokio;
    #[tokio::test]
    pub async fn test() {
        let client = Arachnid::new("KEY".to_string(), "https://URL/target".to_string());
        let f = File::open("test.jpg").await.unwrap();
        let res = client.check_file(f, "test.jpg".to_string(), "image/jpeg".to_string()).await.unwrap();
        dbg!(res);
    }
}