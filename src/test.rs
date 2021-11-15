#[cfg(test)]
pub mod test {
    use tokio;
    use tokio::fs::File;
    use arach_rs::Arachnid;
    #[tokio::test]
    pub async fn test() {
        let client = Arachnid::new("KEY".to_string(), "URL".to_string());
        let f = File::open("test.jpg").await.unwrap();
        let res = client
            .check_file(f, "test.jpg".to_string(), "image/jpeg".to_string())
            .await
            .unwrap();
        dbg!(res);
    }
}
