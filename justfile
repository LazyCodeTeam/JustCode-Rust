check:
  cargo clippy --all-targets --exclude gen --workspace --all-features -- -D warnings
  cargo nextest run
  cargo fmt --check
  cargo sort -w -c
  cargo machete

gen_dir := justfile_directory() / "backend" / "api" / "gen"
gen_apis_dir := gen_dir / "src" / "apis"
gen_lib_dir := gen_dir/ "src" / "lib.rs"
gen_cargo_dir := gen_dir / "Cargo.toml"
gen_cargo_lock_dir := gen_dir / "Cargo.lock"

gen:
  openapi-generator generate -i openapi/swagger_template.yaml -g rust -o {{gen_dir}} --library hyper --additional-properties=packageName=gen,packageVersion=0.1.0,preferUnsignedInt=true,bestFitInt=true

  rm -r {{gen_apis_dir}}

  echo "#[macro_use] extern crate serde; pub mod models;" > {{gen_lib_dir}}
  sed -i -e 's/serde = \(.*\)/serde = { workspace = true, features = ["derive"] }/g' {{gen_cargo_dir}}
  sed -i -e 's/uuid = \(.*\)/uuid = { workspace = true, features = ["serde"] }/g' {{gen_cargo_dir}}
  
  just format

format:
  cargo clippy --fix --allow-dirty 
  cargo fmt
  cargo sort -w
  cargo machete --fix || true

build:
  cargo xtask build-lambda --use-cross --target aarch64-unknown-linux-gnu

infra_dir := justfile_directory() / "infra"

publish env: 
  terraform -chdir={{infra_dir / env}} init
  terraform -chdir={{infra_dir / env}} apply -auto-approve -var-file {{infra_dir / env / "secret.tfvars"}}

bap env:
  just build
  just publish {{env}}
