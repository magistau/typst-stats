{
  craneLib,
}:
craneLib.buildPackage {
  pname = "typst-stats";
  version = "0.1.0";
  src = craneLib.cleanCargoSource ./.;
  strictDeps = true;
  cargoExtraArgs = "--target=wasm32-unknown-unknown";
  doCheck = false;
}
