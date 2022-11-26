#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::use_self,
    clippy::must_use_candidate
)]

use free_storage::FileId;
use napi::{bindgen_prelude::AsyncTask, Result, Task};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
}

#[napi(js_name = "FileId")]
pub struct JsFileId(FileId);

pub struct UploadFile(String, Vec<u8>, String, String);
impl Task for UploadFile {
    type Output = FileId;
    type JsValue = JsFileId;

    fn compute(&mut self) -> Result<Self::Output> {
        static RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());
        RT.block_on(FileId::upload(&self.0, &*self.1, &self.2, &self.3))
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("{e:?}")))
    }
    fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(JsFileId(output))
    }
}

#[napi]
impl JsFileId {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Result<Self> {
        Err(napi::Error::new(
            napi::Status::GenericFailure,
            String::from("`new` should never be called. Use the `upload` method to create an instance instead."),
        ))
    }

    #[napi]
    #[allow(clippy::needless_pass_by_value)]
    pub fn upload(
        file_name: String,
        file_data: Vec<u8>,
        repo: String,
        token: String,
    ) -> AsyncTask<UploadFile> {
        AsyncTask::new(UploadFile(file_name, file_data, repo, token))
    }

    #[napi]
    pub async fn get(&self, token: Option<String>) -> Result<File> {
        let (data, name) = self
            .0
            .get(token)
            .await
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("{e:?}")))?;

        Ok(File { name, data })
    }

    #[napi(js_name = "__debug")]
    pub fn debug(&self) {
        println!("{:#?}", self.0);
    }
}
