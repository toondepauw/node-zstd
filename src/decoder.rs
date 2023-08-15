#![deny(clippy::all)]

use napi::{
  bindgen_prelude::{AsyncTask, Buffer},
  Env, Result, Task,
};
use std::io::Read;
use std::sync::Arc;
use zstd::dict::DecoderDictionary;

#[napi]
pub struct Decoder {
  dict: Option<Arc<DecoderDictionary<'static>>>,
}

#[napi]
impl Decoder {
  #[napi(constructor)]
  pub fn new(dictionary: Option<Buffer>) -> Result<Self> {
    let dict = match dictionary {
      Some(d) => Some(Arc::new(DecoderDictionary::copy(d.as_ref()))),
      None => None,
    };

    Ok(Decoder { dict })
  }

  #[napi(ts_return_type = "Promise<Buffer>")]
  pub fn decode(&self, data: Buffer) -> Result<AsyncTask<DecodeTask>> {
    let decode_task = DecodeTask {
      data: data,
      dict: self.dict.clone(),
    };

    Ok(AsyncTask::new(decode_task))
  }

  #[napi]
  pub fn decode_sync(&self, data: Buffer) -> Result<Buffer> {
    let output = decode(data.as_ref(), &self.dict)?;
    Ok(Buffer::from(output))
  }
}

pub struct DecodeTask {
  data: Buffer,
  dict: Option<Arc<DecoderDictionary<'static>>>,
}

#[napi]
impl Task for DecodeTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let output = decode(self.data.as_ref(), &self.dict)?;
    Ok(output)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Buffer> {
    Ok(Buffer::from(output))
  }

  fn finally(&mut self, _env: Env) -> Result<()> {
    Ok(())
  }
}

pub fn decode(data: &[u8], dict: &Option<Arc<DecoderDictionary<'static>>>) -> Result<Vec<u8>> {
  let mut output = Vec::<u8>::new();
  let mut decoder = match dict {
    Some(d) => zstd::stream::Decoder::with_prepared_dictionary(data, &d)?,
    None => zstd::stream::Decoder::with_buffer(data)?,
  };

  decoder.read_to_end(&mut output)?;
  decoder.finish();

  Ok(output)
}
