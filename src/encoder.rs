#![deny(clippy::all)]

use napi::{
  bindgen_prelude::{AsyncTask, Buffer},
  Env, Result, Task,
};
use std::io::Write;
use std::sync::Arc;
use zstd::dict::EncoderDictionary;

#[napi]
pub struct Encoder {
  level: i32,
  dict: Option<Arc<EncoderDictionary<'static>>>,
}

#[napi]
impl Encoder {
  #[napi(constructor)]
  pub fn new(level: i32, dictionary: Option<Buffer>) -> Result<Self> {
    let dict = match dictionary {
      Some(d) => Some(Arc::new(EncoderDictionary::copy(d.as_ref(), level))),
      None => None,
    };

    Ok(Encoder {
      level: level,
      dict: dict,
    })
  }

  #[napi(ts_return_type = "Promise<Buffer>")]
  pub fn encode(&self, data: Buffer) -> Result<AsyncTask<EncodeTask>> {
    let encode_task = EncodeTask {
      data: data,
      level: self.level,
      dict: self.dict.clone(),
    };

    Ok(AsyncTask::new(encode_task))
  }

  #[napi]
  pub fn encode_sync(&self, data: Buffer) -> Result<Buffer> {
    let output = encode(data.as_ref(), self.level, &self.dict)?;
    Ok(Buffer::from(output))
  }
}

pub struct EncodeTask {
  data: Buffer,
  level: i32,
  dict: Option<Arc<EncoderDictionary<'static>>>,
}

#[napi]
impl Task for EncodeTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let output = encode(self.data.as_ref(), self.level, &self.dict)?;
    Ok(output)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Buffer> {
    Ok(Buffer::from(output))
  }

  fn finally(&mut self, _env: Env) -> Result<()> {
    Ok(())
  }
}

fn encode(
  data: &[u8],
  level: i32,
  dict: &Option<Arc<EncoderDictionary<'static>>>,
) -> Result<Vec<u8>> {
  let mut output = Vec::<u8>::new();
  let mut encoder = match dict {
    Some(d) => zstd::stream::Encoder::with_prepared_dictionary(&mut output, &d)?,
    None => zstd::stream::Encoder::new(&mut output, level)?,
  };

  encoder.write_all(data)?;
  encoder.finish()?;

  Ok(output)
}
