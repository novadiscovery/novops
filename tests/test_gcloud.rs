mod test_utils;


#[cfg(test)]
mod tests {
    use crate::test_utils;
    use log::{info};


    #[tokio::test]
    async fn test_gcloud_secretmanager() -> Result<(), anyhow::Error> {

        init_test();

        let outputs = test_utils::load_env_for("gcloud_secretmanager", "dev").await?;

        info!("test_gcloud_secretmanager: Found variables: {:?}", outputs.variables);
        info!("test_gcloud_secretmanager: Found files: {:?}", outputs.files);

        assert_eq!(outputs.variables.get("SECRETMANAGER_VAR_STRING").unwrap().value, "S3cret!");
        assert_eq!(outputs.files.get("/tmp/gcloud_SECRETMANAGER_VAR_FILE").unwrap().content, "S3cret!".as_bytes());

        Ok(())
    }

    fn init_test(){
        test_utils::init_log();
    }
}