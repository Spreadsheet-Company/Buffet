<#
check_web_offline.ps1 - the game's page loads nothing, links nowhere, and is
filled through its five places, every block the file of src/ it says it is.

WHY: SD-13 in a browser, which Buffet keeps as the house does (docs/OMAKASE.md,
BD-5: no feed, no real market's data). The page, web/index.template.html
filled by tools/build_web.ps1, is one HTML file that runs from disk, and its
whole promise is that it reads nothing from the network and writes nothing
anywhere. Alonzo's check of the same name holds Alonzo's template to that at
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
     {{FAKE_JS}}, {{HOST_JS}}, {{ALONZO_WASM}} and {{BLOCKS}}, once each; and
     no place naming a file, {{KIND:name}}, since the template names no
     game's file and its builder writes every text at {{BLOCKS}} (ENGINE.15).
  2. When web/index.html stands beside the template, its markup holds to the
     same list of tags (no text the page holds may spell one), no place is
     left in it, and its blocks are src/'s files both ways. Each cartridge,
     src/<name>.vla beside its source src/<name>.source.vla, is a cartridge-
     block at that path with data-file <name>.vla, and its source a source-
     block at the source's path; every other .vla of src/ is a library-
     block at its own path; each block's text is its file's with CRLF read as
     LF; and no block of a kind the page reads (cartridge, source, replay,
     golden, library, fixture) is there that no file is. So a page built
     before an edit of the game, or with a library at a path the source's
     includes do not resolve to, where Build in the drawer would refuse or
     write other bytes, fails here and not at a hand test.
Alonzo's list of the Web Audio calls its host never makes is not held here:
it is Alonzo's charter on its own host (its docs/CHARTER.md section 8), held
at the tag by Alonzo's check and here by the host's digest
(tools/check_vendored.ps1).

-Control proves the check on scratch copies: the real template and scripts
pass, and so does a page tools/build_web.ps1 builds in the scratch folder
with -NoWasm; and fourteen mutants fail, each for its own reason. In the
template: a script's src appended, a fetch, the viewport's place removed, an
anchor, an image, a place naming a file, the blocks' place removed. In the
scripts: a fetch appended to the viewport's source, and one to the host's.
In the built page: the cartridge's block cut, a library block no file is,
the template standing as a built page, a library block at another path, and
the source's block one character from its file. Each built mutant proves it
applied before the check is asked.

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
# The places the builder fills once each: the three scripts, the engine's module and the blocks.
$viewportPlaceholder = '{{VIEWPORT_JS}}'
$blocksPlaceholder = '{{BLOCKS}}'
$oncePlaceholders = @($viewportPlaceholder, '{{FAKE_JS}}', '{{HOST_JS}}', '{{ALONZO_WASM}}', $blocksPlaceholder)
# A place naming a file, which the template has no longer (ENGINE.15).
$namedPlace = '\{\{[A-Z]+:[^}]*\}\}'
# A block of a kind the page reads, its id, its attributes and its text.
$blockPattern = '<script type="text/plain" id="((cartridge|source|replay|golden|library|fixture)-[a-z0-9_-]+)"([^>]*)>([\s\S]*?)</script>'

# The blocks src/ owes a page, by id: the file each is read from, its data-path and its data-file.
function Get-OwedBlocks([string]$src) {
    $owed = @{}
    if (-not (Test-Path -LiteralPath $src)) { return $owed }
    $vla = @(Get-ChildItem -LiteralPath $src -File | Where-Object { $_.Extension -eq '.vla' } | Sort-Object Name)
    $carts = New-Object System.Collections.Generic.List[string]
    foreach ($f in $vla) { if ($f.Name.EndsWith('.source.vla')) { $carts.Add($f.Name.Substring(0, $f.Name.Length - '.source.vla'.Length)) } }
    foreach ($n in $carts) {
        $owed['cartridge-' + $n] = @{ File = (Join-Path $src ($n + '.vla')); Path = ('src/' + $n + '.vla'); DataFile = ($n + '.vla') }
        $owed['source-' + $n] = @{ File = (Join-Path $src ($n + '.source.vla')); Path = ('src/' + $n + '.source.vla'); DataFile = '' }
    }
    foreach ($f in $vla) {
        if ($f.Name.EndsWith('.source.vla')) { continue }
        $base = [System.IO.Path]::GetFileNameWithoutExtension($f.Name)
        if ($carts -contains $base) { continue }
        $owed['library-' + $base] = @{ File = $f.FullName; Path = ('src/' + $f.Name); DataFile = '' }
    }
    return $owed
}

function Test-Template([string]$path, [string]$script, [string]$fake = $fakePath, [string]$hostjs = $hostPath) {
    $problems = @()
    $text = [System.IO.File]::ReadAllText($path)
    $lower = $text.ToLowerInvariant()
    foreach ($t in ($tagTokens + $codeTokens)) {
        if ($lower.Contains($t.ToLowerInvariant())) { $problems += ("the template reaches out: '{0}'" -f $t) }
    }
    if (-not $lower.Contains('<meta charset="utf-8">')) { $problems += 'no <meta charset="utf-8">' }
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
        $jApplied = ([System.IO.File]::ReadAllText($j)).Length -lt $built.Length
        $kApplied = ([System.IO.File]::ReadAllText($k)).Contains('id="library-nowhere"')
        $mApplied = ([regex]::Matches($built, [regex]::Escape($kitchenAt))).Count -eq 1 -and ([System.IO.File]::ReadAllText($m)).Contains('data-path="src/courses/kitchen.vla"')
        $nApplied = $at -ge 0 -and ([System.IO.File]::ReadAllText($n)).Length -eq $built.Length + 1 -and ([System.IO.File]::ReadAllText($n)).Contains('(include  "kitchen.vla")')
        $rj = @(Test-Built $j); $rk = @(Test-Built $k); $rl = @(Test-Built $l); $rm = @(Test-Built $m); $rn = @(Test-Built $n)
        $jHit = @($rj | Where-Object { $_ -like '*owes the built page the block cartridge-buffet*' }).Count
        $kHit = @($rk | Where-Object { $_ -like '*holds the block library-nowhere, and no file*' }).Count
        $lHit = @($rl | Where-Object { $_ -like '*has {{BLOCKS}} left in it*' }).Count
        $mHit = @($rm | Where-Object { $_ -like '*block library-kitchen stands at data-path*' }).Count
        $nHit = @($rn | Where-Object { $_ -like "*block source-buffet is not src/buffet.source.vla's text*" }).Count
        Write-Output ("control: the real set has {0} problem(s) and the page built from it {1}; the script src {2}, the fetch {3}, the missing placeholder {4}, the anchor {5}, the image {6}, the place naming a file {7}, the blocks' place removed {8}, the fetch in the viewport {9}, the fetch in the host {10}; built, the cartridge's block cut ({11}) {12}, the stray library block ({13}) {14}, the template as a page {15}, the library at another path ({16}) {17}, the source one character off ({18}) {19}" -f $real.Count, $realBuilt.Count, $ra.Count, $rb.Count, $rc.Count, $rd.Count, $re.Count, $fHit, $hHit, $rg.Count, $iHit, $jApplied, $jHit, $kApplied, $kHit, $lHit, $mApplied, $mHit, $nApplied, $nHit)
        if ($real.Count -eq 0 -and $realBuilt.Count -eq 0 -and $ra.Count -ge 1 -and $rb.Count -ge 1 -and $rc.Count -ge 1 -and $rd.Count -ge 1 -and $re.Count -ge 1 -and $fHit -ge 1 -and $hHit -ge 1 -and $rg.Count -ge 1 -and $iHit -ge 1 -and $jApplied -and $jHit -ge 1 -and $kApplied -and $kHit -ge 1 -and $lHit -ge 1 -and $mApplied -and $mHit -ge 1 -and $nApplied -and $nHit -ge 1) {
            Write-Output 'OK: the check passes the template, the three scripts and a page built from them, and fails each of the fourteen mutants'
            exit 0
        }
        Write-Output 'FAIL: the control did not behave'
        foreach ($p in $real) { Write-Output ('  real: ' + $p) }
        foreach ($p in $realBuilt) { Write-Output ('  real page: ' + $p) }
        exit 1
    } finally {
        Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
    }
}

if (-not (Test-Path -LiteralPath $templatePath)) { Write-Output ("FAIL: no template at {0}" -f $templatePath); exit 1 }
$problems = @(Test-Template $templatePath $scriptPath)
$builtNote = 'no built page beside it (tools/build_web.ps1 writes one)'
if (Test-Path -LiteralPath $builtPath) {
    $problems += @(Test-Built $builtPath)
    $owedCount = (Get-OwedBlocks $srcDir).Count
    $builtNote = ("the built page ({0:N0} characters) holds too, its {1} blocks src/'s files both ways, each at its path with its text" -f (Get-Item -LiteralPath $builtPath).Length, $owedCount)
}
if ($problems.Count -eq 0) {
    Write-Output ("OK: web/index.template.html, web/viewport.js, web/fake.js and web/host.js load nothing and link nowhere, the scripts', the module's and the blocks' places each in its one place, and no place naming a file; {0}" -f $builtNote)
    exit 0
}
Write-Output ("FAIL: the web page has {0} problem(s):" -f $problems.Count)
foreach ($p in $problems) { Write-Output ('  ' + $p) }
exit 1
