use std::{fs, path::Path};

extern crate napi_build;

#[path = "src/types.rs"]
mod types;

use types::OpenWebpageOptions;

fn main() {
  napi_build::setup();

  // Generate JSON schema
  let schema = schemars::schema_for!(OpenWebpageOptions);
  let json = serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");

  let out_path = Path::new("src-js/__generated__/OpenWebpageOptions.schema.json");
  fs::write(out_path, json.clone()).expect("Failed to write schema");

  // Generate TypeScript definitions

  let ts = format!(
    "import type {{ FromSchema }} from 'json-schema-to-ts';

export const OpenWebpageOptionsSchema = 
{} as const;

export type OpenWebpageOptions = FromSchema<typeof OpenWebpageOptionsSchema>;

// Generated...
",
    json
  );
  let ts_out_path = Path::new("src-js/__generated__/OpenWebpageOptions.ts");
  fs::write(ts_out_path, ts).expect("Failed to write schema");

  println!("cargo:rerun-if-changed=src/types.rs");
}
