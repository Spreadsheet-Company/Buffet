<#
build_web.ps1 - the game's page: Alonzo's page template, vendored unchanged at
the tag the pin names, filled with its three scripts, the engine's module and
the game's texts, is web/index.html.

WHAT IT DOES: reads web/index.template.html and fills its five places, each
once: {{VIEWPORT_JS}} with web/viewport.js, {{FAKE_JS}} with web/fake.js and
{{HOST_JS}} with web/host.js, each inlined whole; {{ALONZO_WASM}} with the
engine's module as base64, target/wasm32-unknown-unknown/release/alonzo.wasm
(or -Wasm), which `cargo build --release --locked -p alonzo --target
wasm32-unknown-unknown` writes from the registry crate under the workspace's
release profile; and {{BLOCKS}} with every text the page holds, each a block
of its own, <script type="text/plain" id="KIND-NAME" data-path="PATH">, the
path the file's from the repository's root (Alonzo's web/README.md, "A game's
page", ENGINE.15). The blocks are written in this order: the page's name
(page-name, Buffet); each cartridge, src/<name>.vla beside its source
src/<name>.source.vla, the cartridge's block with data-file, the name a load
passes on, and then its source's, since Build in the drawer runs the pass
over the texts the page holds (cartridge-, source-); and every other .vla of
src/, a library at its own path (library-): today src/common.vla, the kit,
and src/kitchen.vla, Buffet's own, the paths the source's includes resolve
to against the source's folder, so that Build writes the bytes `cargo run
--release --example expand` writes. Every text is inlined with its line
endings normalized to LF, so that the page holds the bytes the checks pin,
and a text holding '</script' would end its block in the page and is
refused. A template with a place naming a file, {{KIND:name}}, is refused,
since the page reads every text from its blocks. The page is written UTF-8
without a byte-order mark to web/index.html, one file that runs from disk
and fetches nothing: a build artifact (.gitignore); the template, the scripts
and the game's files are the source.

WHY A BUILDER OF BUFFET'S OWN: the template names no game's file, so a game
vendors it and the three scripts unchanged and fills them with a builder of
its own (ENGINE.15; docs/OMAKASE.md, HOUSE.4). This is Alonzo's
tools/build_web.ps1 at v0.1.3, adapted by the same copyright holder under
this repository's terms, as oracle/examples/expand.rs was: one game's texts
from src/ in place of Alonzo's cartridge folders, no order list since the
picker offers one cartridge, and no records, since the bench's modes are
Alonzo's own and refuse on a game's page. And the module is required: a
game's page without it plays nothing, the double, web/fake.js, reading a
cartridge's manifest and no row after it and holding no pass. -NoWasm builds
a page without the module anyway, for the ratchets job, which has no cargo,
and for tools/check_web_offline.ps1's control. The replay and golden blocks
come with COURSE.0's tasting, with their files; MENU.1's content sheets,
under src/content/, will want library blocks at their paths, and the rule
above widens then. tools/check_web_offline.ps1 holds the page this writes:
its markup, and each block its file's path and text, both ways.

House style (tools/*.ps1): PowerShell 5.1 and pwsh alike, host-free, no
network; exit 0 with the output named, exit 1 with the reason.

Usage:  powershell -File tools\build_web.ps1
        powershell -File tools\build_web.ps1 -Out C:\somewhere\index.html
        powershell -File tools\build_web.ps1 -NoWasm
#>
param(
    [string]$Out = '',
    [string]$Wasm = '',
    [switch]$NoWasm
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
function Resolve-Arg([string]$value, [string]$default) {
    if ($value -eq '') { return (Join-Path $root $default) }
    if ([System.IO.Path]::IsPathRooted($value)) { return $value }
    return (Join-Path $root $value)
}
$template = Join-Path $root 'web/index.template.html'
$viewportJs = Join-Path $root 'web/viewport.js'
$fakeJs = Join-Path $root 'web/fake.js'
$hostJs = Join-Path $root 'web/host.js'
$srcDir = Join-Path $root 'src'
$Wasm = Resolve-Arg $Wasm 'target/wasm32-unknown-unknown/release/alonzo.wasm'
$Out = Resolve-Arg $Out 'web/index.html'
foreach ($p in @($template, $viewportJs, $fakeJs, $hostJs)) { if (-not (Test-Path -LiteralPath $p)) { Write-Output ("FAIL: no file at {0}" -f $p); exit 1 } }
if (-not (Test-Path -LiteralPath $srcDir)) { Write-Output ("FAIL: no folder at {0}" -f $srcDir); exit 1 }

$html = [System.IO.File]::ReadAllText($template)

# The places: the scripts, the module and the blocks, each once; and none naming a file.
foreach ($ph in @('{{VIEWPORT_JS}}', '{{FAKE_JS}}', '{{HOST_JS}}', '{{ALONZO_WASM}}', '{{BLOCKS}}')) {
    $n = ([regex]::Matches($html, [regex]::Escape($ph))).Count
    if ($n -ne 1) { Write-Output ("FAIL: the template has {0} {1} time(s), not once" -f $ph, $n); exit 1 }
}
$named = [regex]::Match($html, '\{\{[A-Z]+:[^}]*\}\}')
if ($named.Success) { Write-Output ("FAIL: the template names a file at {0}; every text the page holds is a block this builder writes at {{{{BLOCKS}}}} (ENGINE.15)" -f $named.Value); exit 1 }

# The game's texts: each src/<name>.source.vla a cartridge's source, its cartridge src/<name>.vla beside
# it; every other .vla of src/ a library, at the path the source's includes resolve to.
$vla = @(Get-ChildItem -LiteralPath $srcDir -File | Where-Object { $_.Extension -eq '.vla' } | Sort-Object Name)
$cartridges = New-Object System.Collections.Generic.List[string]
foreach ($f in $vla) { if ($f.Name.EndsWith('.source.vla')) { $cartridges.Add($f.Name.Substring(0, $f.Name.Length - '.source.vla'.Length)) } }
if ($cartridges.Count -eq 0) { Write-Output ("FAIL: {0} holds no <name>.source.vla, so the page would hold no game" -f $srcDir); exit 1 }
foreach ($name in $cartridges) {
    if (-not (Test-Path -LiteralPath (Join-Path $srcDir ($name + '.vla')))) { Write-Output ("FAIL: src/{0}.source.vla has no cartridge beside it, src/{0}.vla: expand it first (cargo run --release --example expand)" -f $name); exit 1 }
}
$libraries = @($vla | Where-Object { -not $_.Name.EndsWith('.source.vla') -and $cartridges -notcontains [System.IO.Path]::GetFileNameWithoutExtension($_.Name) })

# The texts, '</script' refused in every one.
function Read-Inlined([string]$path, [bool]$normalize) {
    $t = [System.IO.File]::ReadAllText($path)
    if ($normalize) { $t = $t -replace "`r`n", "`n" }
    if ($t -match '(?i)</script') { throw ("{0} contains '</script', which would end its block in the page" -f $path) }
    return $t
}
# A block: one text, its id the kind and the name, its path the file's from the repository's root.
function New-Block([string]$id, [string]$path, [string]$text, [string]$file = '') {
    $attrs = ''
    if ($file -ne '') { $attrs += ' data-file="' + $file + '"' }
    if ($path -ne '') { $attrs += ' data-path="' + $path + '"' }
    return '<script type="text/plain" id="' + $id + '"' + $attrs + '>' + $text + '</script>'
}
try {
    $scripts = @{ '{{VIEWPORT_JS}}' = (Read-Inlined $viewportJs $false); '{{FAKE_JS}}' = (Read-Inlined $fakeJs $false); '{{HOST_JS}}' = (Read-Inlined $hostJs $false) }
    $blocks = New-Object System.Collections.Generic.List[string]
    $blocks.Add((New-Block 'page-name' '' 'Buffet'))
    foreach ($name in $cartridges) {
        $blocks.Add((New-Block ('cartridge-' + $name) ('src/' + $name + '.vla') (Read-Inlined (Join-Path $srcDir ($name + '.vla')) $true) ($name + '.vla')))
        $blocks.Add((New-Block ('source-' + $name) ('src/' + $name + '.source.vla') (Read-Inlined (Join-Path $srcDir ($name + '.source.vla')) $true)))
    }
    foreach ($lib in $libraries) {
        $blocks.Add((New-Block ('library-' + [System.IO.Path]::GetFileNameWithoutExtension($lib.Name)) ('src/' + $lib.Name) (Read-Inlined $lib.FullName $true)))
    }
} catch { Write-Output ("FAIL: " + $_.Exception.Message); exit 1 }

# The engine's module, base64; required unless -NoWasm.
$wasmText = ''
$wasmNote = 'no engine module built in (-NoWasm)'
if (-not $NoWasm) {
    if (-not (Test-Path -LiteralPath $Wasm)) {
        Write-Output ("FAIL: no engine module at {0}, and a game's page without it plays nothing: build it with cargo build --release --locked -p alonzo --target wasm32-unknown-unknown, or pass -NoWasm for a page that carries none" -f $Wasm)
        exit 1
    }
    $bytes = [System.IO.File]::ReadAllBytes($Wasm)
    $wasmText = [System.Convert]::ToBase64String($bytes)
    $wasmNote = ("the engine module, {0:N0} bytes" -f $bytes.Length)
}

$page = $html
foreach ($ph in $scripts.Keys) { $page = $page.Replace($ph, $scripts[$ph]) }
$page = $page.Replace('{{ALONZO_WASM}}', $wasmText)
$page = $page.Replace('{{BLOCKS}}', ($blocks -join "`n"))
foreach ($ph in @('{{VIEWPORT_JS}}', '{{FAKE_JS}}', '{{HOST_JS}}', '{{ALONZO_WASM}}', '{{BLOCKS}}')) {
    if ($page.Contains($ph)) { Write-Output ("FAIL: {0} is left in the page" -f $ph); exit 1 }
}
$outDir = Split-Path -Parent $Out
if ($outDir -ne '' -and -not (Test-Path -LiteralPath $outDir)) { New-Item -ItemType Directory -Path $outDir | Out-Null }
[System.IO.File]::WriteAllText($Out, $page, (New-Object System.Text.UTF8Encoding($false)))
Write-Output ("OK: wrote {0} ({1:N0} characters): the viewport, the fake module and the host, {2} block(s): the page's name, {3} cartridge(s) with their source(s) and {4} librar{5}, {6}" -f $Out, $page.Length, $blocks.Count, $cartridges.Count, $libraries.Count, $(if ($libraries.Count -eq 1) { 'y' } else { 'ies' }), $wasmNote)
exit 0
