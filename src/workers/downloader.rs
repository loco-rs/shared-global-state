use std::{sync::Arc, time::Duration};

use libvips::{ops, VipsApp, VipsImage};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::{app, models::users};

pub struct DownloadWorker {
    pub ctx: AppContext,
    pub vips: Arc<VipsApp>,
}

impl worker::AppWorker<DownloadWorkerArgs> for DownloadWorker {
    fn build(ctx: &AppContext) -> Self {
        Self {
            ctx: ctx.clone(),
            vips: app::VIPS.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DownloadWorkerArgs {}

#[async_trait]
impl worker::Worker<DownloadWorkerArgs> for DownloadWorker {
    async fn perform(&self, args: DownloadWorkerArgs) -> worker::Result<()> {
        // TODO: Some actual work goes here...
        println!("================================================");
        println!("rescaling image");

        // loads an image from file
        let image = VipsImage::new_from_file("test.png").unwrap();

        // will resize the image and return a new instance.
        // libvips works most of the time with immutable objects, so it will return a new object
        // the VipsImage struct implements Drop, which will free the memory
        let resized = ops::resize(&image, 0.5).unwrap();

        //optional parameters
        let options = ops::JpegsaveOptions {
            q: 90,
            background: vec![255.0],
            optimize_coding: true,
            optimize_scans: true,
            interlace: true,
            ..ops::JpegsaveOptions::default()
        };

        // alternatively you can use `jpegsave` that will use the default options
        match ops::jpegsave_with_opts(&resized, "output.jpeg", &options) {
            Err(_) => println!("error: {}", self.vips.error_buffer().unwrap()),
            Ok(_) => println!("Great Success!"),
        }

        println!("================================================");
        Ok(())
    }
}
