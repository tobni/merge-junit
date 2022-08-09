const { Binary } = require("binary-install");
const os = require("os");

const getPlatform = () => {
  const type = os.type();
  const arch = os.arch();

  if (type === "Linux" && arch === "x64") {
    return "x86_64-unknown-linux-musl";
  }
  if (type === "Darwin" && (arch === "x64")) {
    return "x86_64-apple-darwin";
  }

  throw new Error(`Unsupported platform: ${type} ${arch}`);
};


const getBinary = () => {
  const platform = getPlatform();
  const version = require("../package.json").version;
  const name = "merge-junit";
  const url = `https://github.com/tobni/${name}/releases/download/v${version}/${name}-v${version}-${platform}.tar.gz`;
  return new Binary(name, url);
};

module.exports = { getBinary }