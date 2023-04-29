test:
  cargo nextest run

check: test
  cargo clippy --all-targets --exclude gen --workspace --all-features -- -D warnings
  cargo fmt --check
  cargo sort -w -c
  cargo machete

fix:
  cargo clippy --fix --allow-dirty 
  cargo fmt
  cargo sort -w
  cargo machete --fix || true

build:
  cargo xtask build-lambdas --use-cross --target aarch64-unknown-linux-gnu

gen_dir := justfile_directory() / "api" / "gen"
gen_apis_dir := gen_dir / "src" / "apis"
gen_lib_dir := gen_dir / "src" / "lib.rs"
gen_cargo_dir := gen_dir / "Cargo.toml"

gen:
  openapi-generator generate -i openapi/swagger_template.yaml -g rust -o {{gen_dir}} --library hyper --additional-properties=packageName=gen,packageVersion=0.1.0,preferUnsignedInt=true,bestFitInt=true

  rm -r {{gen_apis_dir}}

  echo "#[macro_use] extern crate serde; pub mod models;" > {{gen_lib_dir}}
  sed -i -e 's/serde = \(.*\)/serde = { workspace = true }/g' {{gen_cargo_dir}}
  sed -i -e 's/uuid = \(.*\)/uuid = { workspace = true }/g' {{gen_cargo_dir}}
  
  just fix

infra_dir := justfile_directory() / "infra"

publish env: 
  terraform -chdir={{infra_dir / env}} init
  terraform -chdir={{infra_dir / env}} apply -auto-approve -var-file {{infra_dir / env / "secret.tfvars"}}

deploy env:
  just build
  just publish {{env}}
