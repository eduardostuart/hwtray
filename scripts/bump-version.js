/**
 * Version management for Cargo.toml, package.json, and tauri.conf.json.
 *
 * Usage:
 *   node scripts/bump-version.js check            # fail if files disagree
 *   node scripts/bump-version.js current          # print the source-of-truth version
 *   node scripts/bump-version.js set <version>    # bump all files
 *
 * Regex-based replacement preserves existing formatting (indentation, array
 * layout, trailing newline) that a JSON or TOML round-trip would rewrite.
 */

import { readFileSync, writeFileSync } from 'node:fs'
import { resolve, dirname, relative } from 'node:path'
import { fileURLToPath } from 'node:url'

const SEMVER = /^\d+\.\d+\.\d+(-[\w.]+)?$/

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..')
const rel = (p) => relative(root, p)

const tomlCodec = (key) => {
  const re = new RegExp(`^${key} = "([^"]+)"`, 'm')
  return {
    read: (raw) => raw.match(re)?.[1],
    write: (raw, v) => raw.replace(re, `${key} = "${v}"`),
  }
}

const jsonCodec = () => {
  const re = /"version":\s*"([^"]+)"/
  return {
    read: (raw) => raw.match(re)?.[1],
    write: (raw, v) => raw.replace(re, `"version": "${v}"`),
  }
}

// Cargo.toml is the source of truth for the workspace version.
const FILES = [
  { path: resolve(root, 'Cargo.toml'), codec: tomlCodec('version') },
  { path: resolve(root, 'package.json'), codec: jsonCodec() },
  { path: resolve(root, 'crates/homewizard-desktop/tauri.conf.json'), codec: jsonCodec() },
]

function readVersions() {
  return FILES.map(({ path, codec }) => {
    const raw = readFileSync(path, 'utf8')
    const version = codec.read(raw)
    if (!version) {
      throw new Error(`No version found in ${rel(path)}`)
    }
    return { path, version }
  })
}

function die(message) {
  console.error(message)
  process.exit(1)
}

const commands = {
  check() {
    const versions = readVersions()
    for (const { path, version } of versions) {
      console.log(`${rel(path)}: ${version}`)
    }
    const source = versions[0].version
    const mismatched = versions.filter((v) => v.version !== source)
    if (mismatched.length > 0) {
      const names = mismatched.map((v) => rel(v.path)).join(', ')
      die(`Version mismatch: ${names} differ from ${rel(versions[0].path)}`)
    }
  },

  current() {
    const [source] = readVersions()
    process.stdout.write(`${source.version}\n`)
  },

  set(version) {
    if (!version) {
      die('Usage: node scripts/bump-version.js set <version>')
    }
    if (!SEMVER.test(version)) {
      die(`Invalid semver: ${version}`)
    }
    for (const { path, codec } of FILES) {
      const raw = readFileSync(path, 'utf8')
      const previous = codec.read(raw)
      writeFileSync(path, codec.write(raw, version))
      console.log(`${rel(path)}: ${previous} → ${version}`)
    }
  },
}

const [name, ...args] = process.argv.slice(2)
const command = commands[name]
if (!command) {
  die('Usage: node scripts/bump-version.js <check|current|set <version>>')
}
command(...args)
