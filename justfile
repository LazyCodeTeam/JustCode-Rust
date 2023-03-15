check:
  cargo clippy --all-targets --exclude gen --workspace --all-features -- -D warnings
  cargo nextest run
  cargo fmt --check
  cargo sort -w -c
  cargo machete

gen_dir := justfile_directory() / "backend" / "api" / "gen"
gen_cargo_dir := gen_dir / "Cargo.toml"
gen_cargo_lock_dir := gen_dir / "Cargo.lock"

gen:
  openapi-generator generate -i openapi/swagger_template.yaml -g rust -o {{gen_dir}} --additional-properties packageName=gen,packageVersion=0.1.0,preferUnsignedInt=true,bestFitInt=true

  sed -i -e 's/^features = \[\(.*\)\]/features = \[\1, \"native-tls-vendored\"\]/g' {{gen_cargo_dir}}
  
  just format

format:
  cargo sort -w
  cargo fmt
  cargo machete --fix || true

build:
  just gen
  cargo xtask build-lambda --use-cross --target aarch64-unknown-linux-gnu

dev_dir := justfile_directory() / "infra" / "dev"
dev_secrets_dir := dev_dir / "secret.tfvars"

publish_dev:
  terraform -chdir={{dev_dir}} apply -auto-approve -var-file {{dev_secrets_dir}}

bap_dev:
  just build
  just publish_dev
