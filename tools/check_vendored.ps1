<#
check_vendored.ps1 - what Buffet copies from Alonzo, held to the files it
copies, at the tag the pin names.

WHY: Buffet consumes the engine and the language from crates.io, and the
engine's package carries neither its kit nor its page, so two things are
copied from Alonzo's repository instead, byte for byte, under the terms they
came with. The kit, cartridges/common.vla, the library of template macros
every Alonzo cartridge is written in, is copied to src/common.vla beside the
game's source (docs/OMAKASE.md, HOUSE.3), its name kept so that it diffs
against Alonzo's by name, under its 0BSD. The page, the template
web/index.template.html and the three scripts it inlines, web/host.js,
web/fake.js and web/viewport.js, is copied to the paths Alonzo gives them
(HOUSE.4), under its Apache-2.0, and tools/build_web.ps1 fills it with the
game. A copy is worth what its last comparison was worth. An edit made here
would fork the kit or the page silently, and a pin moved without the copies
would put the engine of one release beside the kit and the page of another:
HOUSE.2 moved the pin to 0.1.1 and left the kit at v0.1.0, and the check of
the day, check_kit.ps1, which this one grew from, could not see it. So every
copy is held to the SHA-256 of its file at one tag, read off the tag once and
written below, and the tag is held to Cargo.toml's pin of alonzo. A pattern
Buffet wants in the kit, or a change it wants in the page, goes to Alonzo by
a pull request, and the copies and this baseline follow at the next pin,
never ahead of it; a pattern of Buffet's own goes in src/kitchen.vla, and
the page is never edited here.

WHAT IT HOLDS, four things, none needing a host or the network:
  1. Each file the baseline names is there.
  2. Its SHA-256, over its text with CRLF read as LF, and its length in LF
     bytes, equal the ones pinned for it. A .vla is checked out CRLF and the
     page's files LF (.gitattributes), and the tag stores both LF, so the
     digest is of the LF text, as a replay's digest of its cartridge is; the
     length says how far a copy that fails is from the file.
  3. The tag the baseline names is Cargo.toml's pin of alonzo: alonzo =
     "=0.5.0" is the tag v0.5.0. A pin moved without the copies fails here,
     and so do copies moved without the pin.
  4. web/ holds the page's four files and nothing else but the page the
     builder writes there, web/index.html (.gitignore). REUSE.toml gives
     everything under web/ Apache-2.0, the terms the page came with, so a
     file of Buffet's own put there would take the wrong terms.

-Control proves the check on a scratch tree, a built page standing in it:
the clean copies pass; a kit with one byte changed, a host with one byte
changed, a copy missing, the pin moved and not the tag, and a stray file
under web/ each fail for its own reason. Each mutant proves it applied
before the check is asked.

House style (tools/check_*.ps1): PowerShell 5.1 and pwsh alike, host-free, a
hardcoded and reviewable baseline. Exit 0 clean, exit 1 with every failure
listed.

Usage:  powershell -File tools\check_vendored.ps1
        powershell -File tools\check_vendored.ps1 -Control
#>
param(
    [switch]$Control
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot

# ---- baseline: the tag, and each file copied at it with its digest ----
# 2026-10-14, HOUSE.3: the kit at v0.1.0, the tag of alonzo 0.1.0.
# 2026-10-15, HOUSE.4: the pin moved to alonzo 0.1.3; the kit copied again,
# its 22 lines of comments and docstrings ("SPEC.md" become "docs/SPEC.md",
# Alonzo's 2e687f2), and the page's four files copied, all at v0.1.3
# (78aa82e). 2026-10-15, the pin moved to alonzo 0.1.4, whose page carries
# ENGINE.16 to ENGINE.20 (the views on a game's sheets among them, FRONT.1's
# gate): the page's four files copied again at v0.1.4 (1ba83c9); the kit is
# unchanged since v0.1.3, its digest the same. 2026-10-16, the pin moved to
# alonzo 0.1.5, whose page carries ENGINE.7's edit and continue: the template
# and host.js copied again at v0.1.5 (fb3b2a4); fake.js, viewport.js and the
# kit unchanged since v0.1.4, their digests the same. 2026-10-17, the pin
# moved to alonzo 0.2.0 and the language to vla-lang 0.10.0, whose page
# carries the command palette, the keymap and a cartridge's own commands
# (ENGINE.22 to ENGINE.25): the template, host.js and fake.js copied again at
# v0.2.0 (275f1e0); viewport.js unchanged since v0.1.4 and the kit since
# v0.1.3, their digests the same. 2026-10-30, the pin moved to alonzo 0.3.0 and
# the language to vla-lang 0.12.0, whose page carries version 6, a person's
# edits from the grid and a format's appearance drawn (SPEC.5, CART.9,
# ENGINE.28), and whose kit's verbs take expressions (KIT.5, KIT.6's first
# slice): the kit and the page's four files copied again at v0.3.0 (6816d65).
# 2026-11-18, the pin moved to alonzo 0.4.0 and the language to vla-lang
# 0.14.0, whose page saves and opens a game and keeps a run's unsaved work
# (ENGINE.5, ENGINE.38, ENGINE.40), draws a sheet's conditional formats
# (ENGINE.37) and shows the region it was viewed through (ENGINE.43), and
# whose kit takes a put's pieces by name and has no walkers (KIT.6's second
# and third slices and its held step): the kit and the page's four files
# copied again at v0.4.0 (6e7b659). 2026-11-24, the pin moved to alonzo 0.5.0
# and the language to vla-lang 0.15.0, whose page goes on as a game went from a
# save or kept work opened where its Write sheet waits (ENGINE.30's decision 13,
# Buffet's ask), copies, cuts and pastes in the grid (ENGINE.39) and saves a
# file a cartridge reads into the file opened (CART.16, ENGINE.49), and whose
# kit gains file-lines: the kit, the template and host.js copied again at
# v0.5.0 (5aab4c6); fake.js and viewport.js unchanged since v0.4.0, their
# digests the same. Each digest and length
# is `git show <tag>:<path> | sha256sum` and `| wc -c` in Alonzo's tree, the
# blobs being stored LF.
$tag = 'v0.5.0'
$baseline = @(
    @{ File = 'src/common.vla'; From = 'cartridges/common.vla'; Sha256 = '6680CA14029EC4849D7FA0C784318D5AE83202FB1B4828F13E362EF4DA59EB8F'; Bytes = 31592 },
    @{ File = 'web/index.template.html'; From = 'web/index.template.html'; Sha256 = 'EEBBCE851EBE70F03241E18AEED3E71423696A9DB5C5998D26D4A7C63BFF77FA'; Bytes = 1093414 },
    @{ File = 'web/host.js'; From = 'web/host.js'; Sha256 = 'CB70470B611F3146D12B29AE5EC2A51D553FA1D789A230A27743E323B25378CD'; Bytes = 237358 },
    @{ File = 'web/fake.js'; From = 'web/fake.js'; Sha256 = '088EDC0579941B1B1F6039A0CFDD47CB092C22B9836D8D83FA7FC8F1DF054593'; Bytes = 49732 },
    @{ File = 'web/viewport.js'; From = 'web/viewport.js'; Sha256 = '686226D40B42FFD32F1AFE38389D95BE2974701976486230644C5227C331A8EA'; Bytes = 158424 }
)
# The one file web/ may hold beside the copies: the page tools/build_web.ps1 writes, a build artifact.
$builtPage = 'web/index.html'

function Get-LfDigest([string]$path) {
    $text = [System.IO.File]::ReadAllText($path, (New-Object System.Text.UTF8Encoding($false)))
    $bytes = (New-Object System.Text.UTF8Encoding($false)).GetBytes($text.Replace("`r`n", "`n"))
    $sha = [System.Security.Cryptography.SHA256]::Create()
    try { $hash = $sha.ComputeHash($bytes) } finally { $sha.Dispose() }
    return @{ Sha256 = (($hash | ForEach-Object { $_.ToString('X2') }) -join ''); Bytes = $bytes.Length }
}

# The tag Cargo.toml's pin of alonzo names, v<version>, or '' when no exact pin can be read.
function Get-PinnedTag([string]$root) {
    $manifest = Join-Path $root 'Cargo.toml'
    if (-not (Test-Path -LiteralPath $manifest)) { return '' }
    $m = [regex]::Match([System.IO.File]::ReadAllText($manifest), '(?m)^alonzo\s*=\s*"=([0-9]+\.[0-9]+\.[0-9]+)"')
    if (-not $m.Success) { return '' }
    return 'v' + $m.Groups[1].Value
}

function Get-Report([string]$root) {
    $failures = New-Object System.Collections.Generic.List[string]
    $read = 0
    # 1 and 2: each copy there, and the file it copies
    foreach ($b in $baseline) {
        $from = "Alonzo's $($b.From) at $tag"
        $p = Join-Path $root $b.File
        if (-not (Test-Path -LiteralPath $p)) { $failures.Add("missing: $($b.File), the copy of $from"); continue }
        $got = Get-LfDigest $p
        $read++
        if ($got.Sha256 -ne $b.Sha256) {
            $failures.Add("changed: $($b.File) is not $($from): SHA-256 $($got.Sha256) over $($got.Bytes) LF bytes, where the copy is $($b.Sha256) over $($b.Bytes); copy the file again from the tag, or move the pin and this baseline together")
        } elseif ($got.Bytes -ne $b.Bytes) {
            $failures.Add("changed: $($b.File) has the copy's digest and $($got.Bytes) LF bytes, where the baseline says $($b.Bytes); the baseline's two numbers disagree")
        }
    }
    # 3: the tag the pin names
    $pinned = Get-PinnedTag $root
    if ($pinned -eq '') {
        $failures.Add("pin: Cargo.toml holds no exact pin of alonzo, alonzo = ""=X.Y.Z"", so the tag the copies must be at cannot be read")
    } elseif ($pinned -ne $tag) {
        $failures.Add("pin: the copies are at $tag and Cargo.toml pins alonzo at $($pinned.Substring(1)), the tag $($pinned): copy every file again from $pinned and write its digests here, or move the pin back")
    }
    # 4: web/ the page's copies and its built page, and nothing else
    $web = Join-Path $root 'web'
    if (Test-Path -LiteralPath $web) {
        $allowed = @($baseline | Where-Object { $_.File.StartsWith('web/') } | ForEach-Object { $_.File }) + @($builtPage)
        $webFull = (Get-Item -LiteralPath $web -Force).FullName.TrimEnd('\', '/')
        foreach ($f in @(Get-ChildItem -LiteralPath $web -Recurse -File -Force | Sort-Object FullName)) {
            $rel = 'web/' + $f.FullName.Substring($webFull.Length + 1).Replace('\', '/')
            if ($allowed -notcontains $rel) { $failures.Add("stray: $rel is no copy of Alonzo's page and not the page the builder writes; REUSE.toml gives everything under web/ Apache-2.0, so a file of Buffet's own belongs elsewhere") }
        }
    }
    return @{ Failures = $failures; Read = $read; Pinned = $pinned }
}

if ($Control) {
    $tmp = Join-Path ([System.IO.Path]::GetTempPath()) ('buffet_vendored_' + [System.IO.Path]::GetRandomFileName())
    New-Item -ItemType Directory -Path (Join-Path $tmp 'src') | Out-Null
    New-Item -ItemType Directory -Path (Join-Path $tmp 'web') | Out-Null
    try {
        foreach ($b in $baseline) { Copy-Item -LiteralPath (Join-Path $repoRoot $b.File) -Destination (Join-Path $tmp $b.File) }
        Copy-Item -LiteralPath (Join-Path $repoRoot 'Cargo.toml') -Destination (Join-Path $tmp 'Cargo.toml')
        # a built page stands beside the copies, as it may in a working tree
        [System.IO.File]::WriteAllText((Join-Path $tmp $builtPage), "<!doctype html>`n")
        $kit = Join-Path $tmp 'src/common.vla'
        $hostCopy = Join-Path $tmp 'web/host.js'
        $fake = Join-Path $tmp 'web/fake.js'
        $manifest = Join-Path $tmp 'Cargo.toml'
        $stray = Join-Path $tmp 'web/buffet.css'
        $kit0 = [System.IO.File]::ReadAllBytes($kit)
        $host0 = [System.IO.File]::ReadAllBytes($hostCopy)
        $fake0 = [System.IO.File]::ReadAllBytes($fake)
        $manifest0 = [System.IO.File]::ReadAllText($manifest)
        $pinLine = '(?m)^alonzo\s*=\s*"=([0-9.]+)"'
        if ([regex]::Matches($manifest0, $pinLine).Count -ne 1) { Write-Host "FAIL: control: Cargo.toml's pin of alonzo is not one line"; exit 1 }

        function Set-OneByte([string]$path, [byte[]]$orig) {
            $b = [byte[]]$orig.Clone(); $i = [Array]::IndexOf($b, [byte][char]'0'); $b[$i] = [byte][char]'1'; [System.IO.File]::WriteAllBytes($path, $b)
        }
        function Test-OneByte([string]$path, [byte[]]$orig) {
            $now = [System.IO.File]::ReadAllBytes($path)
            ($now.Length -eq $orig.Length) -and (@(for ($j = 0; $j -lt $now.Length; $j++) { if ($now[$j] -ne $orig[$j]) { $j } }).Count -eq 1)
        }

        $verdicts = New-Object System.Collections.Generic.List[string]
        $ok = $true
        $clean = Get-Report $tmp
        if ($clean.Failures.Count -eq 0 -and $clean.Read -eq $baseline.Count) {
            $verdicts.Add("the clean copies: pass, $($clean.Read) files read, the pin's tag $($clean.Pinned), a built page beside them")
        } else {
            $ok = $false
            $verdicts.Add("the clean copies: $($clean.Failures.Count) problem(s), $($clean.Read) read; wanted 0 and $($baseline.Count): $($clean.Failures -join ' | ')")
        }

        $mutants = @(
            @{ Name = 'a kit with one byte changed'
               Apply = { Set-OneByte $kit $kit0 }
               Applied = { Test-OneByte $kit $kit0 }
               Want = 'changed: src/common.vla is not' },
            @{ Name = 'a host with one byte changed'
               Apply = { Set-OneByte $hostCopy $host0 }
               Applied = { Test-OneByte $hostCopy $host0 }
               Want = 'changed: web/host.js is not' },
            @{ Name = 'a copy missing'
               Apply = { Remove-Item -LiteralPath $fake -Force }
               Applied = { -not (Test-Path -LiteralPath $fake) }
               Want = 'missing: web/fake.js' },
            @{ Name = 'the pin moved and not the tag'
               Apply = { [System.IO.File]::WriteAllText($manifest, [regex]::Replace($manifest0, $pinLine, 'alonzo = "=0.1.99"')) }
               Applied = { ([System.IO.File]::ReadAllText($manifest)).Contains('alonzo = "=0.1.99"') }
               Want = 'pins alonzo at 0.1.99' },
            @{ Name = 'a stray file under web/'
               Apply = { [System.IO.File]::WriteAllText($stray, "body { color: #a080e0; }`n") }
               Applied = { Test-Path -LiteralPath $stray }
               Want = 'stray: web/buffet.css' }
        )
        foreach ($mu in $mutants) {
            & $mu.Apply
            $applied = & $mu.Applied
            if (-not $applied) {
                $ok = $false
                $verdicts.Add("$($mu.Name): the mutation did not apply")
            } else {
                $r = Get-Report $tmp
                $hit = $false
                foreach ($x in $r.Failures) { if ($x.Contains($mu.Want)) { $hit = $true } }
                if ($hit) { $verdicts.Add("$($mu.Name): fails, as it should ($($r.Failures.Count): $($r.Failures[0]))") }
                elseif ($r.Failures.Count -gt 0) { $ok = $false; $verdicts.Add("$($mu.Name): fails, but without '$($mu.Want)': $($r.Failures -join ' | ')") }
                else { $ok = $false; $verdicts.Add("$($mu.Name): PASSED but should fail") }
            }
            # put the tree back
            [System.IO.File]::WriteAllBytes($kit, $kit0)
            [System.IO.File]::WriteAllBytes($hostCopy, $host0)
            [System.IO.File]::WriteAllBytes($fake, $fake0)
            [System.IO.File]::WriteAllText($manifest, $manifest0)
            if (Test-Path -LiteralPath $stray) { Remove-Item -LiteralPath $stray -Force }
        }
        foreach ($v in $verdicts) { Write-Host ('control: ' + $v) }
        if ($ok) { Write-Host 'OK: control: the clean copies pass and each of the five mutants fails for its own reason'; exit 0 }
        Write-Host 'FAIL: control: the check did not behave as the header says'
        exit 1
    } finally {
        Remove-Item -LiteralPath $tmp -Recurse -Force -ErrorAction SilentlyContinue
    }
}

$report = Get-Report $repoRoot
Write-Host "check_vendored: $($report.Read) of $($baseline.Count) file(s) copied from Alonzo at $tag read against their digests; Cargo.toml pins the tag $($report.Pinned)"
if ($report.Failures.Count -gt 0) {
    Write-Host "FAIL: $($report.Failures.Count) problem(s)"
    foreach ($f in $report.Failures) { Write-Host "  $f" }
    exit 1
}
Write-Host "OK: the kit and the page's four files are Alonzo's at $tag, the tag Cargo.toml's pin names, and web/ holds nothing else but the built page"
exit 0
