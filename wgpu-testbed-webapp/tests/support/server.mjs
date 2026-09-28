import { createServer } from "node:http";
import { readFile, access } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { resolve, extname, sep } from "node:path";

// Resolve from this file, not the checkout name or the caller's working directory.
const root = fileURLToPath(new URL("../../", import.meta.url));
const repository = resolve(root, "..");
const pkg = resolve(repository, "wgpu-testbed-lib/pkg-web");
const dist = resolve(root, "dist");
const fixture = resolve(root, "tests/fixtures/web.html");
for (const file of [
  fixture,
  resolve(pkg, "wgpu_testbed_lib.js"),
  resolve(dist, "index.html"),
]) {
  await access(file);
}

function within(directory, relative) {
  const file = resolve(directory, relative);
  if (!file.startsWith(directory + sep))
    throw new Error("Path outside asset root");
  return file;
}

function asset(path) {
  if (path === "/web/" || path === "/web/index.html") return fixture;
  if (path.startsWith("/pkg-web/"))
    return within(pkg, path.slice("/pkg-web/".length));
  if (path.startsWith("/web/shaders/") || path.startsWith("/web/resources/")) {
    return within(repository, path.slice("/web/".length));
  }
  if (path.startsWith("/bundler/"))
    return within(dist, path.slice("/bundler/".length) || "index.html");
  return null;
}

const types = {
  ".html": "text/html",
  ".js": "text/javascript",
  ".wasm": "application/wasm",
  ".png": "image/png",
};
const server = createServer(async (request, response) => {
  try {
    const path = decodeURIComponent(
      new URL(request.url, "http://localhost").pathname,
    );
    if (path === "/health") {
      response.end("ready");
      return;
    }
    if (path.endsWith("/favicon.ico")) {
      response.writeHead(204).end();
      return;
    }
    const file = asset(path);
    if (!file) {
      response.writeHead(404).end("Unknown test asset");
      return;
    }
    const content = await readFile(file);
    response.setHeader(
      "Content-Type",
      types[extname(file)] || "application/octet-stream",
    );
    response.end(content);
  } catch (error) {
    console.error(request.url, error.message);
    response.writeHead(404).end("Test asset not found");
  }
});
server.listen(5199, "127.0.0.1");
for (const signal of ["SIGTERM", "SIGINT"])
  process.on(signal, () => server.close());
