use rusoto_core::{Region, RusotoError};
use rusoto_s3::{ListObjectsV2Output, ListObjectsV2Request, S3Client, S3};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let region = Region::default();
    let s3_client = S3Client::new(region);
    let bucket_name = "my-bucket";

    let mut rt = Runtime::new()?;
    let res: Result<ListObjectsV2Output, RusotoError<ListObjectsV2Output>> = rt.block_on(s3_client.list_objects_v2(ListObjectsV2Request {
        bucket: bucket_name.to_string(),
        ..Default::default()
    }));

    match res {
        Ok(result) => {
            if let Some(contents) = result.contents {
                for object in contents {
                    println!("Object key: {}", object.key.as_ref().unwrap());
                }
            } else {
                println!("No objects found in bucket.");
            }
        },
        Err(e) => {
            println!("Error listing objects: {:?}", e);
        }
    }

    Ok(())
}
