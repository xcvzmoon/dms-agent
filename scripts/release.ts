import type { VersionBumpRelease } from 'bumpp';
import type { GitCommit, SemverBumpType } from 'changelogen';
import { versionBump } from 'bumpp';
import {
  determineSemverChange,
  generateMarkDown,
  getCurrentGitStatus,
  getGitDiff,
  loadChangelogConfig,
  parseCommits,
} from 'changelogen';
import { consola } from 'consola';
import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { createInterface } from 'node:readline/promises';

const FORCEABLE_RELEASE_TYPES = [
  'major',
  'premajor',
  'minor',
  'preminor',
  'patch',
  'prepatch',
  'prerelease',
] as const satisfies readonly SemverBumpType[];

type ForceableReleaseType = (typeof FORCEABLE_RELEASE_TYPES)[number];

type CliFlags = {
  dryRun: boolean;
  yes: boolean;
  help: boolean;
  releaseType: ForceableReleaseType | undefined;
  preid: string | undefined;
};

class ReleaseError extends Error {
  readonly code: string;

  constructor(code: string, message: string) {
    super(message);
    this.name = 'ReleaseError';
    this.code = code;
  }
}

function isForceableReleaseType(value: string): value is ForceableReleaseType {
  return FORCEABLE_RELEASE_TYPES.some((type) => type === value);
}

function printHelp(): void {
  consola.info(`Usage: node scripts/release.ts [options]

  Determines the next release from Conventional Commits (via changelogen),
  bumps the package version (via bumpp), updates CHANGELOG.md, creates a
  release commit and tag, and pushes both to trigger the release workflow.

  Options:
    --type <${FORCEABLE_RELEASE_TYPES.join('|')}>
                       Force the release type instead of detecting it from commits.
    --preid <id>       Prerelease identifier (e.g. "beta", "rc") for pre* release types.
    --dry-run          Preview the release without changing any files or git state.
    -y, --yes          Skip the confirmation prompt.
    -h, --help         Show this help message.
  `);
}

function parseCliFlags(args: string[]): CliFlags {
  const flags: CliFlags = {
    dryRun: false,
    yes: false,
    help: false,
    releaseType: undefined,
    preid: undefined,
  };

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];

    switch (arg) {
      case '--dry-run':
        flags.dryRun = true;
        break;
      case '-y':
      case '--yes':
        flags.yes = true;
        break;
      case '-h':
      case '--help':
        flags.help = true;
        break;
      case '--type': {
        const value = args[index + 1];

        if (value === undefined || !isForceableReleaseType(value)) {
          throw new ReleaseError(
            'INVALID_RELEASE_TYPE',
            `--type must be one of: ${FORCEABLE_RELEASE_TYPES.join(', ')}`,
          );
        }

        flags.releaseType = value;
        index += 1;
        break;
      }
      case '--preid': {
        const value = args[index + 1];

        if (value === undefined) {
          throw new ReleaseError('MISSING_PREID', '--preid requires a value');
        }

        flags.preid = value;
        index += 1;
        break;
      }
      default:
        throw new ReleaseError('UNKNOWN_FLAG', `Unknown flag: ${arg}`);
    }
  }

  return flags;
}

function filterRelevantCommits(commits: GitCommit[], types: Record<string, unknown>): GitCommit[] {
  return commits.filter((commit) => {
    return !types[commit.type]
      ? false
      : !(commit.type === 'chore' && commit.scope === 'deps' && !commit.isBreaking);
  });
}

async function confirmPrompt(message: string): Promise<boolean> {
  const rl = createInterface({ input: process.stdin, output: process.stdout });

  try {
    const answer = await rl.question(`${message} (y/N) `);
    return /^y(es)?$/i.test(answer.trim());
  } finally {
    rl.close();
  }
}

function runGit(args: string[], cwd: string): void {
  execFileSync('git', args, { cwd, stdio: 'pipe' });
}

async function writeChangelog(outputPath: string, entry: string): Promise<void> {
  const existingContent = existsSync(outputPath)
    ? await readFile(outputPath, 'utf8')
    : '# Changelog\n\n';
  const lastEntryMatch = existingContent.match(/^###?\s+.*$/m);
  const nextContent =
    lastEntryMatch && typeof lastEntryMatch.index === 'number'
      ? `${existingContent.slice(0, lastEntryMatch.index)}${entry}\n\n${existingContent.slice(lastEntryMatch.index)}`
      : `${existingContent}\n${entry}\n\n`;

  await writeFile(outputPath, nextContent);
}

async function readPackageVersion(cwd: string): Promise<string> {
  const raw = await readFile(join(cwd, 'package.json'), 'utf8');
  const pkg: unknown = JSON.parse(raw);

  if (
    typeof pkg !== 'object' ||
    pkg === null ||
    !('version' in pkg) ||
    typeof pkg.version !== 'string'
  ) {
    throw new ReleaseError(
      'MISSING_PACKAGE_VERSION',
      'package.json has no readable "version" field.',
    );
  }

  return pkg.version;
}

async function updateTauriConfigVersion(cwd: string, newVersion: string): Promise<string> {
  const relativePath = join('src-tauri', 'tauri.conf.json');
  const configPath = join(cwd, relativePath);
  const raw = await readFile(configPath, 'utf8');
  const updated = raw.replace(/"version":\s*"[^"]*"/, `"version": "${newVersion}"`);

  if (updated === raw) {
    throw new ReleaseError(
      'MISSING_TAURI_CONFIG_VERSION',
      `${relativePath} has no "version" field.`,
    );
  }

  await writeFile(configPath, updated);
  return relativePath;
}

async function updateCargoTomlVersion(
  cwd: string,
  newVersion: string,
): Promise<{ relativePath: string; packageName: string }> {
  const relativePath = join('src-tauri', 'Cargo.toml');
  const cargoTomlPath = join(cwd, relativePath);
  const raw = await readFile(cargoTomlPath, 'utf8');
  const packageSectionPattern = /^\[package\]\n(?:(?!^\[)[\s\S])*?/m;
  const nameMatch = raw.match(new RegExp(`${packageSectionPattern.source}^name = "([^"]*)"`, 'm'));

  if (!nameMatch) {
    throw new ReleaseError('MISSING_CARGO_NAME', `${relativePath} has no "[package]" name field.`);
  }

  const packageName = nameMatch[1];
  const updated = raw.replace(
    new RegExp(`(${packageSectionPattern.source}^version = ")[^"]*(")`, 'm'),
    `$1${newVersion}$2`,
  );

  if (updated === raw) {
    throw new ReleaseError(
      'MISSING_CARGO_VERSION',
      `${relativePath} has no "[package]" version field.`,
    );
  }

  await writeFile(cargoTomlPath, updated);
  return { relativePath, packageName };
}

async function updateCargoLockVersion(
  cwd: string,
  packageName: string,
  newVersion: string,
): Promise<string> {
  const relativePath = join('src-tauri', 'Cargo.lock');
  const cargoLockPath = join(cwd, relativePath);
  const raw = await readFile(cargoLockPath, 'utf8');
  const pattern = new RegExp(
    `(\\[\\[package\\]\\]\\nname = "${packageName}"\\nversion = ")[^"]*(")`,
  );
  const updated = raw.replace(pattern, `$1${newVersion}$2`);

  if (updated === raw) {
    throw new ReleaseError(
      'MISSING_CARGO_LOCK_VERSION',
      `${relativePath} has no entry for package "${packageName}".`,
    );
  }

  await writeFile(cargoLockPath, updated);
  return relativePath;
}

async function syncTauriVersion(cwd: string, newVersion: string): Promise<string[]> {
  const tauriConfigPath = await updateTauriConfigVersion(cwd, newVersion);
  const { relativePath: cargoTomlPath, packageName } = await updateCargoTomlVersion(
    cwd,
    newVersion,
  );
  const cargoLockPath = await updateCargoLockVersion(cwd, packageName, newVersion);
  return [tauriConfigPath, cargoTomlPath, cargoLockPath];
}

async function main(): Promise<void> {
  const flags = parseCliFlags(process.argv.slice(2));

  if (flags.help) {
    printHelp();
    return;
  }

  const cwd = process.cwd();
  const dirtyStatus = await getCurrentGitStatus(cwd);

  if (dirtyStatus && !flags.dryRun) {
    throw new ReleaseError(
      'DIRTY_WORKTREE',
      'Working tree has uncommitted changes. Commit or stash them before releasing.',
    );
  }

  const config = await loadChangelogConfig(cwd);
  const rawCommits = await getGitDiff(config.from, config.to, config.cwd);
  const commits = filterRelevantCommits(
    parseCommits(rawCommits, config).map((commit) => {
      commit.type = commit.type.toLowerCase();
      return commit;
    }),
    config.types,
  );

  const releaseType: SemverBumpType | null =
    flags.releaseType ?? determineSemverChange(commits, config);

  if (releaseType === null) {
    const from = config.from || 'the beginning of history';
    consola.info(`No relevant commits since ${from}. Nothing to release.`);
    return;
  }

  const currentVersion = await readPackageVersion(cwd);

  consola.info(`Current version : v${currentVersion}`);
  consola.info(`Release type    : ${releaseType}`);
  consola.info(`Commits included: ${commits.length}`);

  if (flags.dryRun) {
    const markdown = await generateMarkDown(commits, config);
    consola.info('\n--- Changelog preview (dry run) ---\n');
    consola.info(markdown);
    consola.info('\nDry run complete. No files or git state were changed.');
    return;
  }

  if (!flags.yes && !(await confirmPrompt(`Proceed with a "${releaseType}" release?`))) {
    consola.info('Release cancelled.');
    return;
  }

  const release: VersionBumpRelease = releaseType;
  const bumpResult = await versionBump({
    release,
    currentVersion,
    preid: flags.preid,
    commit: false,
    tag: false,
    push: false,
    install: false,
    confirm: false,
    interface: false,
    cwd,
  });

  config.newVersion = bumpResult.newVersion;

  const tauriFiles = await syncTauriVersion(cwd, bumpResult.newVersion);
  const markdown = await generateMarkDown(commits, config);
  const filesToStage = ['package.json', ...tauriFiles];

  if (typeof config.output === 'string') {
    await writeChangelog(config.output, markdown);
    filesToStage.push(config.output);
  }

  const commitMessage =
    config.templates.commitMessage?.replaceAll('{{newVersion}}', bumpResult.newVersion) ??
    `chore(release): v${bumpResult.newVersion}`;
  const tagName = `v${bumpResult.newVersion}`;
  const tagMessage =
    config.templates.tagMessage?.replaceAll('{{newVersion}}', bumpResult.newVersion) ?? tagName;

  runGit(['add', ...filesToStage], cwd);
  runGit(['commit', '-m', commitMessage], cwd);
  runGit(['tag', '-a', tagName, '-m', tagMessage], cwd);
  runGit(['push', '--follow-tags'], cwd);

  consola.info(`Released and pushed ${tagName}.`);
}

try {
  await main();
} catch (error) {
  if (error instanceof ReleaseError) {
    consola.error(`[${error.code}] ${error.message}`);
  } else {
    consola.error(error instanceof Error ? error.message : error);
  }

  process.exit(1);
}
