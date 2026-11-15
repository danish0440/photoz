# MCP

`photoz-automation` implements an MCP server over stdio with three backends:

- **Headless:** an in-process `Headless` session owns the engine and performs file I/O.
- **Bridge:** `BridgeClient` forwards tools to a desktop application started with `photoz --control <port>`.
- **The open window:** plain `photoz-cli mcp` finds the desktop application through the connection file it publishes while the person lets agents use it (AppZ › Settings › AI agents), before every tool call; while the app is closed it starts it, serves a headless session, or reports an error, as chosen in Preferences › AI Integrations. Without that consent, or with `--headless`, it is the headless session above.

The simplest setup needs no terminal: turn on AppZ › Settings › AI agents and click *Add* next to the AI app; that one entry reaches every Craft app. See [`docs/mcp.md`](../../../docs/mcp.md#connecting-the-desktop-app-no-terminal).

For a source checkout, first [install the CLI on PATH and register the server](../../../docs/mcp.md#install-and-register).
The installed binary survives `cargo clean`; a symlink into `target/release` does not.

Start the headless server with (without `--headless`, `mcp` works in the open PhotoZ window
while the AppZ's agents switch is on):

```sh
photoz-cli mcp --headless --automation-read-root /work/project --automation-write-root /work/project
```

Bridge to a running desktop application with the same private token file used by the app:

```sh
photoz-cli mcp --bridge 127.0.0.1:7878 --control-token-file /private/path/control.token
```

Use absolute root paths in MCP client configurations, since a relative root is resolved from the
client's launch directory. In bridge mode the roots are configured on the desktop app, and root
flags on `photoz-cli mcp --bridge` are rejected. Explicit `--control-token` and
`--control-token-file` flags require `--bridge` (or `PHOTOZ_CONTROL_PORT`) and are mutually
exclusive; bridge credentials can also come from `PHOTOZ_CONTROL_TOKEN` or
`PHOTOZ_CONTROL_TOKEN_FILE`. Each launch option
except `--help`/`-h` takes one non-empty value and cannot be repeated. Stray positional arguments,
missing values and options for the wrong mode are usage errors (exit 2) reported on stderr before
the server starts.

Tools cover session/document operations, command discovery and execution, batching, and—when bridged—UI inspection and control. Tool schemas improve correctness but are not authorization boundaries.

## From an installed release

The release packages ship `photoz-cli` alongside the desktop app, so no build is needed:

| Install | CLI |
|---|---|
| Windows (MSI) | `%LOCALAPPDATA%\Programs\ArtCraft\PhotoZ\photoz-cli.exe` by default (`C:\Program Files\ArtCraft\PhotoZ\photoz-cli.exe` for an all-users install, under `Program Files (x86)` instead for the 32-bit build, or wherever you installed it otherwise), not on `PATH` |
| Linux (deb, rpm) | `/usr/bin/photoz-cli` |
| macOS | the separate `photoz-cli-<version>-macos-<arch>.zip` release asset (the `.app` holds only the desktop app) |

```sh
# Windows (PowerShell), default install folder
claude mcp add photoz -- "$env:LOCALAPPDATA\Programs\ArtCraft\PhotoZ\photoz-cli.exe" mcp --automation-read-root <dir> --automation-write-root <dir>
# Linux, or macOS with the CLI unzipped onto PATH
claude mcp add photoz -- photoz-cli mcp --automation-read-root <dir> --automation-write-root <dir>
```

## Security notes

Stdio MCP does not open a network listener. Its client receives only the read and write roots
explicitly granted at launch; omitting a root omits that authority. Bridge mode authenticates its
loopback control connection with a bearer token before invoking any control method and uses the
roots configured on the desktop process.

The launcher's switch (off by default) opens the same loopback, token-authenticated control
listener as `--control`, with a token the app keeps in its settings folder (`agent-control.token`,
user-only on Unix; delete it for a new one). The folders chosen in Preferences › AI Integrations
are the desktop process's roots, and the roots of a `mcp` headless session only when agents may
use the app and *When PhotoZ is closed* is set to work without a window.

The current server does not issue scoped sessions or distinguish trusted tools from high-impact tools. Authentication proves possession of the token; it does not restrict an authenticated client to document-only access. Do not expose either transport through an untrusted broker, shell, proxy, or remote service without adding an external policy boundary.

Filesystem capabilities are implemented. General tool and method capabilities remain
[proposed](../security/automation-security.md).

The five shared core tools are `command_list`, `command_run`, `command_batch`, `doc_inspect`
and `render_preview`. Existing documented tools, including `doc_render_preview`, remain
listed. `photoz://document` and `photoz://commands` expose live JSON. Long commands
that are waited on report progress and stop when the request is cancelled, headless and in
bridge mode (where they may also outlast the bridge's reply timeout), and direct headless
Render Video does too, using the granted write root; see
[`docs/mcp.md`](../../../docs/mcp.md) for arguments, cleanup and transport behavior.
