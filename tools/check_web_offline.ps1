<#
check_web_offline.ps1 - the game's page loads nothing, links nowhere, and is
filled through its five places, every block the file of src/ it says it is.

WHY: SD-13 in a browser, which Buffet keeps as the house does (docs/OMAKASE.md,
BD-5: no feed, no real market's data). The page, web/index.template.html
filled by tools/build_web.ps1, is one HTML file that runs from disk, and its
whole promise is that it reads nothing from the network and writes nothing
anywhere but a person's keymap and a run's unsaved work, kept in the browser,
and the files a person saves (Alonzo's ENGINE.23, ENGINE.5 and ENGINE.38).
Alonzo's check of the same name holds Alonzo's template to that at
every tag; this one holds the copy here to it whatever a later tag brings, and
the page this repository builds, whose blocks are Buffet's own texts. It is
Alonzo's tools/check_web_offline.ps1 at v0.1.3, adapted by the same copyright
holder under this repository's terms (docs/OMAKASE.md, HOUSE.4).

WHAT IT HOLDS, neither needing a host or the network:
  1. On the template and the three scripts the builder inlines into the page,
     web/viewport.js, web/fake.js and web/host.js, so that their text is the
     page's code: no reference that reaches out, of any kind (a script's src,
     a link, an image, a frame, a form, an anchor, video, audio, source,
     object, embed, a meta refresh, @import, url(, http://, https://, fetch,
     XMLHttpRequest, WebSocket, EventSource, sendBeacon, a dynamic import,
     importScripts); a charset declaration; the five places, {{VIEWPORT_JS}},
     {{FAKE_JS}}, {{HOST_JS}}, {{ALONZO_WASM}} and {{BLOCKS}}, once each;
     no place naming a file, {{KIND:name}}, since the template names no
     game's file and its builder writes every text at {{BLOCKS}} (ENGINE.15);
     and the stores, below: each name the page writes through, once in the
     template and in no script, and no other store named anywhere.
  2. When web/index.html stands beside the template, its markup holds to the
     same list of tags (no text the page holds may spell one), no place is
     left in it, and its blocks are src/'s files both ways. Each cartridge,
     src/<name>.vla beside its source src/<name>.source.vla, is a cartridge-
     block at that path with data-file <name>.vla, and its source a source-
     block at the source's path; every other .vla of src/ is a library-
     block at its own path, and each .vla of src/content/, a content sheet
     (MENU.1), a library-content- block at its path; each block's text is its
     file's with CRLF read as LF; and no block of a kind the page reads
     (cartridge, source, replay, golden, library, fixture) is there that no
     file is. So a page built before an edit of the game, or with a library
     at a path the source's includes do not resolve to, where Build in the
     drawer would refuse or write other bytes, fails here and not at a hand
     test.
  3. The session beside the cartridge (HOUSE.4's second slice, Alonzo's
     rules of v0.4.0): its replay and its golden, src/<name>.replay.vla and
     src/<name>.golden.vla, are replay- and golden- blocks at their paths,
     and its hand, src/<name>.hand.js, the hand- block, the one block that
     is a script, <script id="hand-NAME" data-path="PATH">, each owed both
     ways with its file's text as the others are. The hand's source is the
     page's code once its block runs, so it is held to the code tokens, makes
     no code of a text (eval, Function) and names no store. And the replay's
     header names its cartridge by the SHA-256 of src/<name>.vla's LF text,
     as tools/check_vendored.ps1 computes its digests, so that a page built
     over a replay the cartridge has moved under fails here before a
     browser plays it: cargo run --release --example replay restamps it.
Alonzo's list of the Web Audio calls its host never makes is not held here:
it is Alonzo's charter on its own host (its docs/CHARTER.md section 8), held
at the tag by Alonzo's check and here by the host's digest
(tools/check_vendored.ps1).

THE STORES, since the pin to alonzo 0.4.0 (2026-11-18, HOUSE.2), Alonzo's
rules at v0.4.0 carried whole. The page writes three things, each through
names the template holds once, in one function, and no script names: a
person's keymap, through localStorage, in browserStore() (Alonzo's ENGINE.23,
written to the page at alonzo 0.2.0 and not held here until this pin); a file
the person saves, through showSaveFilePicker, createObjectURL and
revokeObjectURL, in personFile() (ENGINE.5), whose object URL is no address on
any network, so those three names are set aside before 'url(' is looked for;
and a run's unsaved work, through indexedDB under navigator.locks, in
workStore() (ENGINE.38). Since the pin to alonzo 0.5.0 (2026-11-24), Alonzo's
rule at v0.5.0 beside them: a save of a file a cartridge reads is written into
the file a person opened, through the handle the browser's open picker gave
(ENGINE.49), so showOpenFilePicker is a way to a write too and is named once,
in personFile(), with the three. No source names another store: sessionStorage,
openDatabase, document.cookie, cookieStore, the Cache API's caches.,
navigator.storage or showDirectoryPicker. So a store a later tag adds fails
here by its name, as a reach out does.

-Control proves the check on scratch copies: the real template and scripts
pass, and so does a page tools/build_web.ps1 builds in the scratch folder
with -NoWasm; and twenty-nine mutants fail, each for its own reason. In the
template: a script's src appended, a fetch, the viewport's place removed, an
anchor, an image, a place naming a file, the blocks' place removed. In the
scripts: a fetch appended to the viewport's source, and one to the host's.
In the built page: the cartridge's block cut, a library block no file is,
the template standing as a built page, a library block at another path, the
source's block one character from its file, and a content sheet's block cut.
And the stores: the template naming sessionStorage, naming localStorage a
second time, the save picker a second time and indexedDB a second time; the
host's source naming localStorage, the save picker and navigator.locks.
And the session's (HOUSE.4's second slice): the built page without the
hand's block, and a replay block no file is; a hand calling fetch, one
calling eval, one naming localStorage and one naming indexedDB; and a replay
naming another cartridge. Each built mutant proves it applied before the
check is asked.

House style (tools/check_*.ps1): PowerShell 5.1 and pwsh alike, host-free,
no network; exit 0 clean, exit 1 with every problem named.

Usage:  powershell -File tools\check_web_offline.ps1
        powershell -File tools\check_web_offline.ps1 -Control
#>
param(
    [switch]$Control
)
$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
$templatePath = Join-Path $repoRoot 'web/index.template.html'
$scriptPath = Join-Path $repoRoot 'web/viewport.js'
$fakePath = Join-Path $repoRoot 'web/fake.js'
$hostPath = Join-Path $repoRoot 'web/host.js'
$builtPath = Join-Path $repoRoot 'web/index.html'
$srcDir = Join-Path $repoRoot 'src'

# Markup that reaches out: held on the template and on a built page alike.
# There is no allowed anchor on this page, so '<a href' is forbidden outright.
$tagTokens = @('<script src', '<script type="module" src', '<link ', '<img ', '<iframe', '<form', '<a href', '<video', '<audio', '<source', '<object', '<embed', '<meta http-equiv="refresh"')
# Code and style that reach out: held on the template and on the three scripts the builder inlines.
$codeTokens = @('fetch(', 'XMLHttpRequest', 'WebSocket', 'EventSource', 'sendBeacon', 'import(', 'importScripts', '@import', 'url(', 'http://', 'https://')
# The stores (THE STORES above, Alonzo's at v0.5.0): no source names any of these. Compared without case.
$storageTokens = @('sessionStorage', 'openDatabase', 'document.cookie', 'cookieStore', 'caches.', 'navigator.storage', 'showDirectoryPicker')
# A person's keymap (ENGINE.23): named once, in the template's browserStore(), and nowhere else.
$keptStore = 'localStorage'
# A person's file (ENGINE.5): the save picker and the download's object URL, each named once, in the template's
# personFile(), and nowhere else; set aside before the code tokens are looked for, an object URL being no address.
# Since the pin to alonzo 0.5.0 the open picker with them, its handle a way to a write, a save into the file opened
# (ENGINE.49).
$keptFile = @('showSaveFilePicker', 'showOpenFilePicker', 'createObjectURL', 'revokeObjectURL')
# A run's unsaved work (ENGINE.38): the browser's database for pages and its Web Locks, each named once, in the
# template's workStore(), and nowhere else.
$keptWork = @('indexedDB', 'navigator.locks')
# The places the builder fills once each: the three scripts, the engine's module and the blocks.
$viewportPlaceholder = '{{VIEWPORT_JS}}'
$blocksPlaceholder = '{{BLOCKS}}'
$oncePlaceholders = @($viewportPlaceholder, '{{FAKE_JS}}', '{{HOST_JS}}', '{{ALONZO_WASM}}', $blocksPlaceholder)
# A place naming a file, which the template has no longer (ENGINE.15).
$namedPlace = '\{\{[A-Z]+:[^}]*\}\}'
# A block of a kind the page reads, its id, its attributes and its text.
$blockPattern = '<script type="text/plain" id="((cartridge|source|replay|golden|library|fixture)-[a-z0-9_-]+)"([^>]*)>([\s\S]*?)</script>'
# A hand's block (Alonzo's ENGINE.16): the one block that is a script, run where it stands.
$handPattern = '<script id="(hand-[a-z0-9_-]+)"([^>]*)>([\s\S]*?)</script>'
# A hand's source runs as the page's code: the code tokens, and no text made code.
$handEval = '\beval\s*\(|\bFunction\s*\('

# The blocks src/ owes a page, by id: the file each is read from, its data-path and its data-file.
function Get-OwedBlocks([string]$src) {
    $owed = @{}
    if (-not (Test-Path -LiteralPath $src)) { return $owed }
    $vla = @(Get-ChildItem -LiteralPath $src -File | Where-Object { $_.Extension -eq '.vla' } | Sort-Object Name)
    $carts = New-Object System.Collections.Generic.List[string]
    foreach ($f in $vla) { if ($f.Name.EndsWith('.source.vla')) { $carts.Add($f.Name.Substring(0, $f.Name.Length - '.source.vla'.Length)) } }
    $session = New-Object System.Collections.Generic.List[string]
    foreach ($n in $carts) {
        $owed['cartridge-' + $n] = @{ File = (Join-Path $src ($n + '.vla')); Path = ('src/' + $n + '.vla'); DataFile = ($n + '.vla') }
        $owed['source-' + $n] = @{ File = (Join-Path $src ($n + '.source.vla')); Path = ('src/' + $n + '.source.vla'); DataFile = '' }
        # The session beside it (HOUSE.4's second slice): its replay, its golden and its hand, each where it stands.
        foreach ($kind in @(@('replay', '.replay.vla'), @('golden', '.golden.vla'), @('hand', '.hand.js'))) {
            $file = Join-Path $src ($n + $kind[1])
            if (Test-Path -LiteralPath $file) { $owed[$kind[0] + '-' + $n] = @{ File = $file; Path = ('src/' + $n + $kind[1]); DataFile = '' } }
        }
        $session.Add($n + '.replay.vla'); $session.Add($n + '.golden.vla')
    }
    foreach ($f in $vla) {
        if ($f.Name.EndsWith('.source.vla')) { continue }
        if ($session -contains $f.Name) { continue }
        $base = [System.IO.Path]::GetFileNameWithoutExtension($f.Name)
        if ($carts -contains $base) { continue }
        $owed['library-' + $base] = @{ File = $f.FullName; Path = ('src/' + $f.Name); DataFile = '' }
    }
    # The content sheets (MENU.1): each .vla of src/content/, a library block at its path, its id library-content-<name>.
    $content = Join-Path $src 'content'
    if (Test-Path -LiteralPath $content) {
        foreach ($f in @(Get-ChildItem -LiteralPath $content -File | Where-Object { $_.Extension -eq '.vla' } | Sort-Object Name)) {
            $owed['library-content-' + [System.IO.Path]::GetFileNameWithoutExtension($f.Name)] = @{ File = $f.FullName; Path = ('src/content/' + $f.Name); DataFile = '' }
        }
    }
    return $owed
}

function Test-Template([string]$path, [string]$script, [string]$fake = $fakePath, [string]$hostjs = $hostPath) {
    $problems = @()
    $text = [System.IO.File]::ReadAllText($path)
    $lower = $text.ToLowerInvariant()
    $scanned = $lower
    foreach ($k in $keptFile) { $scanned = $scanned.Replace($k.ToLowerInvariant(), '') }
    foreach ($t in ($tagTokens + $codeTokens)) {
        if ($scanned.Contains($t.ToLowerInvariant())) { $problems += ("the template reaches out: '{0}'" -f $t) }
    }
    # A person's file, written through each of its names once, in personFile() (ENGINE.5).
    foreach ($k in $keptFile) {
        $nk = ([regex]::Matches($lower, [regex]::Escape($k.ToLowerInvariant()))).Count
        if ($nk -ne 1) { $problems += ("the template names {0} {1} time(s), where the page writes a person's file through it in one place, personFile() (ENGINE.5)" -f $k, $nk) }
    }
    # A run's unsaved work, kept through each of its names once, in workStore() (ENGINE.38).
    foreach ($k in $keptWork) {
        $nk = ([regex]::Matches($lower, [regex]::Escape($k.ToLowerInvariant()))).Count
        if ($nk -ne 1) { $problems += ("the template names {0} {1} time(s), where the page keeps a run's unsaved work through it in one place, workStore() (ENGINE.38)" -f $k, $nk) }
    }
    if (-not $lower.Contains('<meta charset="utf-8">')) { $problems += 'no <meta charset="utf-8">' }
    # The keymap's store named once (ENGINE.23); no other store named.
    foreach ($t in $storageTokens) {
        if ($lower.Contains($t.ToLowerInvariant())) { $problems += ("the template names '{0}', a store the page never writes" -f $t) }
    }
    $kept = ([regex]::Matches($lower, [regex]::Escape($keptStore.ToLowerInvariant()))).Count
    if ($kept -ne 1) { $problems += ("the template names {0} {1} time(s), where the page writes a person's keymap through it in one place, browserStore() (ENGINE.23)" -f $keptStore, $kept) }
    foreach ($ph in $oncePlaceholders) {
        $n = ([regex]::Matches($text, [regex]::Escape($ph))).Count
        if ($n -ne 1) { $problems += ("placeholder {0} appears {1} time(s), and the template has one place for it" -f $ph, $n) }
    }
    foreach ($m in [regex]::Matches($text, $namedPlace)) { $problems += ("the template names a file at {0}; every text the page holds is a block its builder writes at {1}" -f $m.Value, $blocksPlaceholder) }
    # The scripts' sources: the code tokens, since each is the page's code once inlined.
    foreach ($pair in @(@("the viewport's source", $script), @("the fake module's source", $fake), @("the host's source", $hostjs))) {
        if (-not (Test-Path -LiteralPath $pair[1])) { $problems += ("no file for {0} at {1}" -f $pair[0], $pair[1]); continue }
        $js = [System.IO.File]::ReadAllText($pair[1]).ToLowerInvariant()
        foreach ($t in $codeTokens) {
            if ($js.Contains($t.ToLowerInvariant())) { $problems += ("{0} reaches out: '{1}'" -f $pair[0], $t) }
        }
        foreach ($t in ($storageTokens + @($keptStore) + $keptFile + $keptWork)) {
            if ($js.Contains($t.ToLowerInvariant())) { $problems += ("{0} names '{1}', and the page writes nothing but a person's keymap, through the template's one {2} (ENGINE.23), a person's file, through its one personFile() (ENGINE.5), and a run's unsaved work, through its one workStore() (ENGINE.38)" -f $pair[0], $t, $keptStore) }
        }
    }
    # Returned without the comma, as Alonzo's returns it: the callers wrap the result in @().
    return $problems
}

# A built page: its markup, no place left, and its blocks src/'s files both ways, each at its path with its text.
function Test-Built([string]$path, [string]$src = $srcDir) {
    $problems = @()
    $text = [System.IO.File]::ReadAllText($path)
    $lower = $text.ToLowerInvariant()
    foreach ($t in $tagTokens) {
        if ($lower.Contains($t.ToLowerInvariant())) { $problems += ("the built page reaches out: '{0}'" -f $t) }
    }
    foreach ($ph in $oncePlaceholders) {
        if ($text.Contains($ph)) { $problems += ("the built page has {0} left in it" -f $ph) }
    }
    foreach ($m in [regex]::Matches($text, $namedPlace)) { $problems += ("the built page has {0} left in it" -f $m.Value) }
    $held = @{}
    foreach ($m in [regex]::Matches($text, $blockPattern)) {
        $id = $m.Groups[1].Value
        if ($held.ContainsKey($id)) { $problems += ("the built page holds the block {0} twice" -f $id); continue }
        $attrs = $m.Groups[3].Value
        $p = [regex]::Match($attrs, 'data-path="([^"]*)"')
        $df = [regex]::Match($attrs, 'data-file="([^"]*)"')
        $held[$id] = @{ Path = $(if ($p.Success) { $p.Groups[1].Value } else { '' }); DataFile = $(if ($df.Success) { $df.Groups[1].Value } else { '' }); Text = $m.Groups[4].Value }
    }
    foreach ($m in [regex]::Matches($text, $handPattern)) {
        $id = $m.Groups[1].Value
        if ($held.ContainsKey($id)) { $problems += ("the built page holds the block {0} twice" -f $id); continue }
        $p = [regex]::Match($m.Groups[2].Value, 'data-path="([^"]*)"')
        $held[$id] = @{ Path = $(if ($p.Success) { $p.Groups[1].Value } else { '' }); DataFile = ''; Text = $m.Groups[3].Value }
    }
    $owed = Get-OwedBlocks $src
    foreach ($id in @($owed.Keys | Sort-Object)) {
        $o = $owed[$id]
        if (-not $held.ContainsKey($id)) { $problems += ("src/ owes the built page the block {0}, {1}, and the page holds no such block: build it again (tools/build_web.ps1)" -f $id, $o.Path); continue }
        $h = $held[$id]
        if ($h.Path -cne $o.Path) { $problems += ("the block {0} stands at data-path ""{1}"", and its file is {2}: Build in the drawer finds a text by its path" -f $id, $h.Path, $o.Path) }
        if ($h.DataFile -cne $o.DataFile) { $problems += ("the block {0} has data-file ""{1}"", where it is ""{2}""" -f $id, $h.DataFile, $o.DataFile) }
        if (-not (Test-Path -LiteralPath $o.File)) { $problems += ("the block {0} is owed by {1}, and no file is there" -f $id, $o.Path); continue }
        $want = ([System.IO.File]::ReadAllText($o.File)).Replace("`r`n", "`n")
        if (-not [string]::Equals($h.Text, $want, [System.StringComparison]::Ordinal)) { $problems += ("the block {0} is not {1}'s text: build the page again (tools/build_web.ps1)" -f $id, $o.Path) }
    }
    foreach ($id in @($held.Keys | Sort-Object)) { if (-not $owed.ContainsKey($id)) { $problems += ("the built page holds the block {0}, and no file of src/ is it" -f $id) } }
    return $problems
}

# The hands' sources (HOUSE.4's second slice; Alonzo's ENGINE.16), src/<name>.hand.js: each is the page's code once
# its block runs, so each is held to the code tokens, makes no code of a text and names no store.
function Test-Hands([string]$src = $srcDir) {
    $problems = @()
    if (-not (Test-Path -LiteralPath $src)) { return $problems }
    foreach ($f in @(Get-ChildItem -LiteralPath $src -File | Where-Object { $_.Name.EndsWith('.hand.js') } | Sort-Object Name)) {
        $n = $f.Name.Substring(0, $f.Name.Length - '.hand.js'.Length)
        $js = [System.IO.File]::ReadAllText($f.FullName)
        $lower = $js.ToLowerInvariant()
        foreach ($t in $codeTokens) { if ($lower.Contains($t.ToLowerInvariant())) { $problems += ("the hand of {0} reaches out: '{1}'" -f $n, $t) } }
        if ([regex]::IsMatch($js, $handEval)) { $problems += ("the hand of {0} makes code of a text: '{1}'" -f $n, [regex]::Match($js, $handEval).Value) }
        foreach ($t in ($storageTokens + @($keptStore) + $keptFile + $keptWork)) { if ($lower.Contains($t.ToLowerInvariant())) { $problems += ("the hand of {0} names '{1}', and the page writes nothing but a person's keymap, file and unsaved work, each through the template's one place" -f $n, $t) } }
    }
    return $problems
}

# The SHA-256 of a file's text with CRLF read as LF, upper case, as tools/check_vendored.ps1 computes its digests.
function Get-LfSha256([string]$path) {
    $text = [System.IO.File]::ReadAllText($path, (New-Object System.Text.UTF8Encoding($false)))
    $bytes = (New-Object System.Text.UTF8Encoding($false)).GetBytes($text.Replace("`r`n", "`n"))
    $sha = [System.Security.Cryptography.SHA256]::Create()
    try { $hash = $sha.ComputeHash($bytes) } finally { $sha.Dispose() }
    return (($hash | ForEach-Object { $_.ToString('X2') }) -join '')
}

# Each replay's header (HOUSE.4's second slice): it names the cartridge beside it by the SHA-256 of its LF text,
# which the page's player requires of the file it loads.
function Test-Session([string]$src = $srcDir) {
    $problems = @()
    if (-not (Test-Path -LiteralPath $src)) { return $problems }
    foreach ($f in @(Get-ChildItem -LiteralPath $src -File | Where-Object { $_.Name.EndsWith('.replay.vla') } | Sort-Object Name)) {
        $n = $f.Name.Substring(0, $f.Name.Length - '.replay.vla'.Length)
        $cart = Join-Path $src ($n + '.vla')
        $m = [regex]::Match([System.IO.File]::ReadAllText($f.FullName), '(?m)^\(replay "[^"]*" \(spec \d+\) \(cartridge "([0-9A-F]{64})"\) \(seed \d+\)\)\r?$')
        if (-not $m.Success) { $problems += ("src/{0} has no replay's header, (replay ""<name>"" (spec N) (cartridge ""<SHA-256>"") (seed S))" -f $f.Name); continue }
        if (-not (Test-Path -LiteralPath $cart)) { $problems += ("src/{0} stands beside no cartridge, src/{1}.vla" -f $f.Name, $n); continue }
        $digest = Get-LfSha256 $cart
        if ($m.Groups[1].Value -cne $digest) { $problems += ("src/{0} names the cartridge {1}, and src/{2}.vla is {3}: restamp it, cargo run --release --example replay" -f $f.Name, $m.Groups[1].Value, $n, $digest) }
    }
    return $problems
}

if ($Control) {
    $tmp = Join-Path ([System.IO.Path]::GetTempPath()) ('buffet_weboff_' + [System.IO.Path]::GetRandomFileName())
    New-Item -ItemType Directory -Path $tmp | Out-Null
    try {
        $utf8 = New-Object System.Text.UTF8Encoding($false)
        $text = [System.IO.File]::ReadAllText($templatePath)
        $js = [System.IO.File]::ReadAllText($scriptPath)
        $real = @(Test-Template $templatePath $scriptPath)
        $a = Join-Path $tmp 'a.html'; [System.IO.File]::WriteAllText($a, ($text + "`n<script src=""x.js""></script>`n"), $utf8)
        $b = Join-Path $tmp 'b.html'; [System.IO.File]::WriteAllText($b, ($text + "`n<script>fetch('x')</script>`n"), $utf8)
        $c = Join-Path $tmp 'c.html'; [System.IO.File]::WriteAllText($c, $text.Replace($viewportPlaceholder, ''), $utf8)
        $d = Join-Path $tmp 'd.html'; [System.IO.File]::WriteAllText($d, ($text + "`n<a href=""x.html"">x</a>`n"), $utf8)
        $e = Join-Path $tmp 'e.html'; [System.IO.File]::WriteAllText($e, ($text + "`n<img src=""data:image/png;base64,AAAA"">`n"), $utf8)
        $f = Join-Path $tmp 'f.html'; [System.IO.File]::WriteAllText($f, ($text + "`n{{CARTRIDGE:buffet}}`n"), $utf8)
        $gjs = Join-Path $tmp 'g.js'; [System.IO.File]::WriteAllText($gjs, ($js + "`nfetch('x');`n"), $utf8)
        if (([regex]::Matches($text, [regex]::Escape($blocksPlaceholder))).Count -ne 1) { Write-Output "FAIL: the place $blocksPlaceholder is not in the template exactly once"; exit 1 }
        $h = Join-Path $tmp 'h.html'; [System.IO.File]::WriteAllText($h, $text.Replace($blocksPlaceholder, ''), $utf8)
        $ijs = Join-Path $tmp 'i.js'; [System.IO.File]::WriteAllText($ijs, ([System.IO.File]::ReadAllText($hostPath) + "`nfetch('x');`n"), $utf8)
        $ra = @(Test-Template $a $scriptPath); $rb = @(Test-Template $b $scriptPath); $rc = @(Test-Template $c $scriptPath)
        $rd = @(Test-Template $d $scriptPath); $re = @(Test-Template $e $scriptPath)
        $rf = @(Test-Template $f $scriptPath); $rg = @(Test-Template $templatePath $gjs); $rh = @(Test-Template $h $scriptPath)
        $ri = @(Test-Template $templatePath $scriptPath $fakePath $ijs)
        $fHit = @($rf | Where-Object { $_ -like '*names a file at {{CARTRIDGE:buffet}}*' }).Count
        $hHit = @($rh | Where-Object { $_ -like '*{{BLOCKS}} appears 0 time(s)*' }).Count
        $iHit = @($ri | Where-Object { $_ -like "the host's source reaches out*" }).Count
        # The stores (ENGINE.23, ENGINE.5, ENGINE.38): a template naming sessionStorage, and one naming localStorage, the
        # save picker or indexedDB a second time; a host source naming localStorage, the save picker or navigator.locks;
        # each failing by its words
        $so = Join-Path $tmp 'so.html'; [System.IO.File]::WriteAllText($so, ($text + "`n<script>sessionStorage.setItem('x', '1');</script>`n"), $utf8)
        $sp = Join-Path $tmp 'sp.html'; [System.IO.File]::WriteAllText($sp, ($text + "`n<script>window.localStorage.setItem('x', '1');</script>`n"), $utf8)
        $sqjs = Join-Path $tmp 'sq.js'; [System.IO.File]::WriteAllText($sqjs, ([System.IO.File]::ReadAllText($hostPath) + "`nwindow.localStorage.setItem('x', '1');`n"), $utf8)
        $fp = Join-Path $tmp 'fp.html'; [System.IO.File]::WriteAllText($fp, ($text + "`n<script>window.showSaveFilePicker();</script>`n"), $utf8)
        $fqjs = Join-Path $tmp 'fq.js'; [System.IO.File]::WriteAllText($fqjs, ([System.IO.File]::ReadAllText($hostPath) + "`nwindow.showSaveFilePicker();`n"), $utf8)
        $wr = Join-Path $tmp 'wr.html'; [System.IO.File]::WriteAllText($wr, ($text + "`n<script>window.indexedDB.open('x');</script>`n"), $utf8)
        $wsjs = Join-Path $tmp 'ws.js'; [System.IO.File]::WriteAllText($wsjs, ([System.IO.File]::ReadAllText($hostPath) + "`nnavigator.locks.request('x', function () {});`n"), $utf8)
        $soHit = @(@(Test-Template $so $scriptPath) | Where-Object { $_ -like "the template names 'sessionStorage'*" }).Count
        $spHit = @(@(Test-Template $sp $scriptPath) | Where-Object { $_ -like 'the template names localStorage 2 time(s)*' }).Count
        $sqHit = @(@(Test-Template $templatePath $scriptPath $fakePath $sqjs) | Where-Object { $_ -like "the host's source names 'localStorage'*" }).Count
        $fpHit = @(@(Test-Template $fp $scriptPath) | Where-Object { $_ -like 'the template names showSaveFilePicker 2 time(s)*' }).Count
        $fqHit = @(@(Test-Template $templatePath $scriptPath $fakePath $fqjs) | Where-Object { $_ -like "the host's source names 'showSaveFilePicker'*" }).Count
        $wrHit = @(@(Test-Template $wr $scriptPath) | Where-Object { $_ -like 'the template names indexedDB 2 time(s)*' }).Count
        $wsHit = @(@(Test-Template $templatePath $scriptPath $fakePath $wsjs) | Where-Object { $_ -like "the host's source names 'navigator.locks'*" }).Count
        # A page built in the scratch folder, which passes, and five built pages that must not.
        $shell = if (Get-Command powershell.exe -ErrorAction SilentlyContinue) { 'powershell.exe' } else { 'pwsh' }
        $page = Join-Path $tmp 'page.html'
        $prev = $ErrorActionPreference
        $ErrorActionPreference = 'Continue'
        try { $null = & $shell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'build_web.ps1') -NoWasm -Out $page 2>&1; $code = $LASTEXITCODE } finally { $ErrorActionPreference = $prev }
        if ($code -ne 0 -or -not (Test-Path -LiteralPath $page)) { Write-Output 'FAIL: tools/build_web.ps1 could not build the control''s page'; exit 1 }
        $built = [System.IO.File]::ReadAllText($page)
        $realBuilt = @(Test-Built $page)
        function Remove-Block([string]$html, [string]$id) { return [regex]::Replace($html, '<script type="text/plain" id="' + [regex]::Escape($id) + '"[^>]*>[\s\S]*?</script>\n?', '') }
        $j = Join-Path $tmp 'j.html'; [System.IO.File]::WriteAllText($j, (Remove-Block $built 'cartridge-buffet'), $utf8)
        $k = Join-Path $tmp 'k.html'; [System.IO.File]::WriteAllText($k, $built.Replace('<script type="text/plain" id="alonzo-wasm">', '<script type="text/plain" id="library-nowhere" data-path="src/nowhere.vla">x</script>' + "`n" + '<script type="text/plain" id="alonzo-wasm">'), $utf8)
        $l = Join-Path $tmp 'l.html'; [System.IO.File]::WriteAllText($l, $text, $utf8)
        $kitchenAt = 'id="library-kitchen" data-path="src/kitchen.vla"'
        $m = Join-Path $tmp 'm.html'; [System.IO.File]::WriteAllText($m, $built.Replace($kitchenAt, 'id="library-kitchen" data-path="src/courses/kitchen.vla"'), $utf8)
        # one character more in the source's block alone: a space in its include of the library
        $include = '(include "kitchen.vla")'
        $srcBlock = [regex]::Match($built, '<script type="text/plain" id="source-buffet"[^>]*>([\s\S]*?)</script>')
        $at = if ($srcBlock.Success) { $srcBlock.Groups[1].Value.IndexOf($include) } else { -1 }
        $nPage = $built
        if ($at -ge 0) { $cut = $srcBlock.Groups[1].Index + $at + '(include'.Length; $nPage = $built.Substring(0, $cut) + ' ' + $built.Substring($cut) }
        $n = Join-Path $tmp 'n.html'; [System.IO.File]::WriteAllText($n, $nPage, $utf8)
        # a content sheet's block cut (MENU.1): the page built before the sheet existed
        $o = Join-Path $tmp 'o.html'; [System.IO.File]::WriteAllText($o, (Remove-Block $built 'library-content-disclosures'), $utf8)
        $jApplied = ([System.IO.File]::ReadAllText($j)).Length -lt $built.Length
        $kApplied = ([System.IO.File]::ReadAllText($k)).Contains('id="library-nowhere"')
        $mApplied = ([regex]::Matches($built, [regex]::Escape($kitchenAt))).Count -eq 1 -and ([System.IO.File]::ReadAllText($m)).Contains('data-path="src/courses/kitchen.vla"')
        $nApplied = $at -ge 0 -and ([System.IO.File]::ReadAllText($n)).Length -eq $built.Length + 1 -and ([System.IO.File]::ReadAllText($n)).Contains('(include  "kitchen.vla")')
        $oApplied = $built.Contains('id="library-content-disclosures" data-path="src/content/disclosures.vla"') -and -not ([System.IO.File]::ReadAllText($o)).Contains('id="library-content-disclosures"')
        $rj = @(Test-Built $j); $rk = @(Test-Built $k); $rl = @(Test-Built $l); $rm = @(Test-Built $m); $rn = @(Test-Built $n); $ro = @(Test-Built $o)
        # The session (HOUSE.4's second slice): the page without the hand's block, and a replay block no file is; a
        # scratch folder whose hand calls fetch, calls eval, names localStorage or names indexedDB; and one whose replay
        # names another cartridge. The real hands and session pass first.
        $realHands = @(Test-Hands)
        $realSession = @(Test-Session)
        $hb = Join-Path $tmp 'hb.html'; [System.IO.File]::WriteAllText($hb, [regex]::Replace($built, '<script id="hand-buffet"[^>]*>[\s\S]*?</script>\n?', ''), $utf8)
        $hbApplied = $built.Contains('<script id="hand-buffet"') -and -not ([System.IO.File]::ReadAllText($hb)).Contains('<script id="hand-buffet"')
        $hbHit = @(@(Test-Built $hb) | Where-Object { $_ -like '*owes the built page the block hand-buffet*' }).Count
        $rs = Join-Path $tmp 'rs.html'; [System.IO.File]::WriteAllText($rs, $built.Replace('<script type="text/plain" id="alonzo-wasm">', '<script type="text/plain" id="replay-nowhere" data-path="src/nowhere.replay.vla">x</script>' + "`n" + '<script type="text/plain" id="alonzo-wasm">'), $utf8)
        $rsApplied = ([System.IO.File]::ReadAllText($rs)).Contains('id="replay-nowhere"')
        $rsHit = @(@(Test-Built $rs) | Where-Object { $_ -like '*holds the block replay-nowhere, and no file*' }).Count
        $handText = [System.IO.File]::ReadAllText((Join-Path $srcDir 'buffet.hand.js'))
        $handHits = New-Object System.Collections.Generic.List[string]
        $handsOk = $true
        foreach ($pl in @(@('fetch', "`nfetch('x');", "the hand of buffet reaches out: 'fetch('"), @('eval', "`neval('1');", "the hand of buffet makes code of a text: 'eval('"), @('localStorage', "`nwindow.localStorage.setItem('x', '1');", "the hand of buffet names 'localStorage'"), @('indexedDB', "`nindexedDB.open('x');", "the hand of buffet names 'indexedDB'"))) {
            $copy = Join-Path $tmp ('hand_' + $pl[0])
            New-Item -ItemType Directory -Path $copy | Out-Null
            [System.IO.File]::WriteAllText((Join-Path $copy 'buffet.hand.js'), ($handText + $pl[1] + "`n"), $utf8)
            $hApplied = ([System.IO.File]::ReadAllText((Join-Path $copy 'buffet.hand.js'))).Contains($pl[1].Trim())
            $hHit = @(@(Test-Hands $copy) | Where-Object { $_ -like ($pl[2] + '*') }).Count
            $handHits.Add(("{0} {1}" -f $pl[0], $(if ($hApplied -and $hHit -ge 1) { 'failed' } else { 'NOT failed' })))
            if (-not ($hApplied -and $hHit -ge 1)) { $handsOk = $false }
        }
        $other = Join-Path $tmp 'other'
        New-Item -ItemType Directory -Path $other | Out-Null
        Copy-Item -LiteralPath (Join-Path $srcDir 'buffet.vla') -Destination (Join-Path $other 'buffet.vla')
        $replayText = [System.IO.File]::ReadAllText((Join-Path $srcDir 'buffet.replay.vla'))
        $stamp = [regex]::Match($replayText, '\(cartridge "([0-9A-F]{64})"\)')
        $otherText = if ($stamp.Success) { $replayText.Replace($stamp.Groups[1].Value, ('0' * 64)) } else { $replayText }
        [System.IO.File]::WriteAllText((Join-Path $other 'buffet.replay.vla'), $otherText, $utf8)
        $otApplied = $stamp.Success -and ([System.IO.File]::ReadAllText((Join-Path $other 'buffet.replay.vla'))).Contains('(cartridge "' + ('0' * 64) + '")')
        $otHit = @(@(Test-Session $other) | Where-Object { $_ -like '*names the cartridge 0000*restamp it*' }).Count
        $jHit = @($rj | Where-Object { $_ -like '*owes the built page the block cartridge-buffet*' }).Count
        $kHit = @($rk | Where-Object { $_ -like '*holds the block library-nowhere, and no file*' }).Count
        $lHit = @($rl | Where-Object { $_ -like '*has {{BLOCKS}} left in it*' }).Count
        $mHit = @($rm | Where-Object { $_ -like '*block library-kitchen stands at data-path*' }).Count
        $nHit = @($rn | Where-Object { $_ -like "*block source-buffet is not src/buffet.source.vla's text*" }).Count
        $oHit = @($ro | Where-Object { $_ -like '*owes the built page the block library-content-disclosures*' }).Count
        Write-Output ("control: the session: the real hands {0} problem(s) and the replay's header {1}; built, the hand's block cut ({2}) {3}, a stray replay block ({4}) {5}; the hands: {6}; a replay naming another cartridge ({7}) {8}" -f $realHands.Count, $realSession.Count, $hbApplied, $hbHit, $rsApplied, $rsHit, ($handHits -join ', '), $otApplied, $otHit)
        Write-Output ("control: the real set has {0} problem(s) and the page built from it {1}; the script src {2}, the fetch {3}, the missing placeholder {4}, the anchor {5}, the image {6}, the place naming a file {7}, the blocks' place removed {8}, the fetch in the viewport {9}, the fetch in the host {10}; built, the cartridge's block cut ({11}) {12}, the stray library block ({13}) {14}, the template as a page {15}, the library at another path ({16}) {17}, the source one character off ({18}) {19}, the content sheet's block cut ({20}) {21}; the stores: sessionStorage in the template {22}, localStorage there a second time {23} and in the host {24}, the save picker there a second time {25} and in the host {26}, the work's store there a second time {27} and its locks in the host {28}" -f $real.Count, $realBuilt.Count, $ra.Count, $rb.Count, $rc.Count, $rd.Count, $re.Count, $fHit, $hHit, $rg.Count, $iHit, $jApplied, $jHit, $kApplied, $kHit, $lHit, $mApplied, $mHit, $nApplied, $nHit, $oApplied, $oHit, $soHit, $spHit, $sqHit, $fpHit, $fqHit, $wrHit, $wsHit)
        if ($real.Count -eq 0 -and $realBuilt.Count -eq 0 -and $ra.Count -ge 1 -and $rb.Count -ge 1 -and $rc.Count -ge 1 -and $rd.Count -ge 1 -and $re.Count -ge 1 -and $fHit -ge 1 -and $hHit -ge 1 -and $rg.Count -ge 1 -and $iHit -ge 1 -and $jApplied -and $jHit -ge 1 -and $kApplied -and $kHit -ge 1 -and $lHit -ge 1 -and $mApplied -and $mHit -ge 1 -and $nApplied -and $nHit -ge 1 -and $oApplied -and $oHit -ge 1 -and $soHit -ge 1 -and $spHit -ge 1 -and $sqHit -ge 1 -and $fpHit -ge 1 -and $fqHit -ge 1 -and $wrHit -ge 1 -and $wsHit -ge 1 -and $realHands.Count -eq 0 -and $realSession.Count -eq 0 -and $hbApplied -and $hbHit -ge 1 -and $rsApplied -and $rsHit -ge 1 -and $handsOk -and $otApplied -and $otHit -ge 1) {
            Write-Output 'OK: the check passes the template, the three scripts, the hand, the replay''s header and a page built from them, and fails each of the twenty-nine mutants'
            exit 0
        }
        Write-Output 'FAIL: the control did not behave'
        foreach ($p in $real) { Write-Output ('  real: ' + $p) }
        foreach ($p in $realBuilt) { Write-Output ('  real page: ' + $p) }
        foreach ($p in $realHands) { Write-Output ('  real hand: ' + $p) }
        foreach ($p in $realSession) { Write-Output ('  real session: ' + $p) }
        exit 1
    } finally {
        Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
    }
}

if (-not (Test-Path -LiteralPath $templatePath)) { Write-Output ("FAIL: no template at {0}" -f $templatePath); exit 1 }
$problems = @(Test-Template $templatePath $scriptPath)
$problems += @(Test-Hands)
$problems += @(Test-Session)
$builtNote = 'no built page beside it (tools/build_web.ps1 writes one)'
if (Test-Path -LiteralPath $builtPath) {
    $problems += @(Test-Built $builtPath)
    $owedCount = (Get-OwedBlocks $srcDir).Count
    $builtNote = ("the built page ({0:N0} characters) holds too, its {1} blocks src/'s files both ways, each at its path with its text" -f (Get-Item -LiteralPath $builtPath).Length, $owedCount)
}
if ($problems.Count -eq 0) {
    Write-Output ("OK: web/index.template.html, web/viewport.js, web/fake.js, web/host.js and src/'s hand load nothing and link nowhere, no hand makes code of a text, each replay names the cartridge beside it, nothing names a store but the template's one localStorage, a person's keymap's, its one personFile(), a person's file's, and its one workStore(), a run's unsaved work's, the scripts', the module's and the blocks' places each in its one place, and no place naming a file; {0}" -f $builtNote)
    exit 0
}
Write-Output ("FAIL: the web page has {0} problem(s):" -f $problems.Count)
foreach ($p in $problems) { Write-Output ('  ' + $p) }
exit 1
