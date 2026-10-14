<#
check_layout.ps1 - the game's one folder, src/, held to the layout MISE.1 decided.

WHY: the game lives in one folder, src/ (the owner's word of 2026-10-14,
OMAKASE.md, MISE.1): the source a person edits, the cartridge the pass writes
beside it, Buffet's own library and the kit's copy, and later the content
sheets and the frozen courses. A path is the dearest thing to move later,
since the license map, the checks and the page's places name it, and a folder
that gains a file nobody named gains it silently. Rows are the engine's to
judge and not this script's: the oracle crate's tests load the cartridge
through alonzo, hold it to its source expanded and to its describe golden,
and hold every frame of it to the game written by hand, and CI's crate job
runs them on every push. So this is no port of Alonzo's cartridge check,
which reads every row because Alonzo's cartridges have no oracle crate
beside them: it holds what no row says, and needs no cargo to say it.

WHAT IT HOLDS, three things, none needing a host, cargo or the network:
  1. src/ holds exactly the files the baseline names, each of a kind: the
     source, its cartridge and the two libraries. Any other file under src/
     fails by name. Each kind is counted against the count pinned for it
     below: sources 1, cartridges 1, libraries 2, and content sheets (a .vla
     of src/content/) and courses (a folder of src/courses/) 0 until they
     exist, the first of each raising its pin with the rule its slice
     writes (MENU.1 the content rule, MISE.1's second slice the courses').
  2. The cartridge's second line is the pass's banner naming its source,
     word for word as alonzo::pass::banner writes it under the source's
     header line (check_spdx.ps1 holds that header): a cartridge written by
     hand, or expanded from another source, says so there.
  3. The source includes common.vla, then kitchen.vla, each by a line of its
     own as the pass reads an include, and kitchen.vla includes nothing
     (MISE.1's decision 2): the pass resolves every include against the
     source's own folder, an included file's too, so a library that included
     the kit would name it by a path true at one depth only, and a frozen
     course stands two folders deeper.

-Control proves the check on a scratch copy of the four files: the clean
copy passes; a stray file under src/, the cartridge missing, a banner naming
another source, an include in kitchen.vla, the source's includes out of
order, and a content sheet and a course before their pins are raised each
fail by name. Each mutant proves it applied before the check is asked.

House style (tools/check_*.ps1): PowerShell 5.1 and pwsh alike, host-free, a
hardcoded and reviewable baseline. Exit 0 clean, exit 1 with every failure
listed.

Usage:  powershell -File tools\check_layout.ps1
        powershell -File tools\check_layout.ps1 -Control
#>
param(
    [switch]$Control
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot

# ---- baseline: the files of src/ by kind, each kind's count pinned ----
# 2026-10-14, MISE.1's first slice: the source; its cartridge, the pass's
# <name>.vla beside <name>.source.vla; and the libraries the source includes,
# in the order it includes them, the kit's copy (HOUSE.3), then Buffet's own,
# which includes nothing. No content sheet and no course yet.
$sourceName = 'buffet.source.vla'
$cartridgeName = 'buffet.vla'
$includeOrder = @('common.vla', 'kitchen.vla')
$includesNothing = @('kitchen.vla')
$kinds = @(
    @{ Kind = 'source'; Plural = 'sources'; Pinned = 1; Names = @($sourceName)
       Missing = 'what a person edits, which the cartridge is expanded from'
       Raise = 'a file added or removed moves its name and this pin together, in this file' },
    @{ Kind = 'cartridge'; Plural = 'cartridges'; Pinned = 1; Names = @($cartridgeName)
       Missing = 'which the pass writes beside its source: cargo run --release --example expand'
       Raise = 'a file added or removed moves its name and this pin together, in this file' },
    @{ Kind = 'library'; Plural = 'libraries'; Pinned = 2; Names = $includeOrder
       Missing = 'which the source includes'
       Raise = 'a file added or removed moves its name and this pin together, in this file' },
    @{ Kind = 'content sheet'; Plural = 'content sheets'; Pinned = 0; Folder = 'content'; Unit = 'file'
       Raise = 'the content rule, written with the first sheet (MENU.1), raises this pin' },
    @{ Kind = 'course'; Plural = 'courses'; Pinned = 0; Folder = 'courses'; Unit = 'folder'
       Raise = 'the courses'' rule, MISE.1''s second slice (COURSE.1), raises this pin' }
)

# A file's name to its kind, case and all: a name that differs only in case
# is another file to Linux, where CI builds.
$named = New-Object System.Collections.Hashtable
foreach ($k in $kinds) { foreach ($n in @($k.Names)) { if ($n) { $named[$n] = $k.Kind } } }

# The banner the pass writes on a cartridge's second line, under the source's
# header line: alonzo::pass::banner, word for word.
function Get-Banner([string]$sourceFile) { return "; Expanded by the pass from ${sourceFile}: edit that file, then expand it again." }

# A file's lines, its CRLF read as LF.
function Get-LfLines([string]$path) {
    $text = [System.IO.File]::ReadAllText($path, (New-Object System.Text.UTF8Encoding($false)))
    $lines = $text.Replace("`r`n", "`n").Split([char]10)
    return ,$lines
}

# One line as the pass reads an include (alonzo::pass::include_name): its text
# trimmed opens "(include " and closes ")", ASCII case aside, and between
# them, trimmed, stands one quoted name with no quote in it. Returns the name;
# '' for a line that opens as an include does and is not one the pass reads;
# $null for a line that is no include, a comment among them.
function Read-Include([string]$line) {
    $t = $line.Trim()
    if (-not [regex]::IsMatch($t, '^\(include\s', 'IgnoreCase')) { return $null }
    if (-not $t.StartsWith('(include ', [System.StringComparison]::OrdinalIgnoreCase) -or -not $t.EndsWith(')')) { return '' }
    $inner = $t.Substring(9, $t.Length - 10).Trim()
    if ($inner.Length -lt 2 -or -not $inner.StartsWith('"') -or -not $inner.EndsWith('"')) { return '' }
    $name = $inner.Substring(1, $inner.Length - 2)
    if ($name.Length -eq 0 -or $name.Contains('"')) { return '' }
    return $name
}

# The includes of a file, in order: one record a line that opens as an include.
function Get-Includes([string]$path) {
    $list = New-Object System.Collections.Generic.List[object]
    $lines = Get-LfLines $path
    for ($i = 0; $i -lt $lines.Count; $i++) {
        $name = Read-Include $lines[$i]
        if ($null -ne $name) { $list.Add(@{ Line = $i + 1; Name = $name; Text = $lines[$i].Trim() }) }
    }
    return ,$list
}

# Names in order, as a failure says them.
function Format-Names($names) {
    $all = @($names)
    if ($all.Count -eq 0) { return 'nothing' }
    return (($all | ForEach-Object { if ($_ -eq '') { '(an include the pass cannot read)' } else { $_ } }) -join ', then ')
}

# Each kind's count, as the summary says it.
function Format-Found($found) {
    $parts = New-Object System.Collections.Generic.List[string]
    foreach ($k in $kinds) {
        $n = $found[$k.Kind].Count
        $word = if ($n -eq 1) { $k.Kind } else { $k.Plural }
        $parts.Add("$n $word")
    }
    return ($parts -join ', ')
}

# The check over a root. Returns one record: Failures (one line each), Found
# (each kind's files under src/, a course by its folder) and Files (every
# file under src/).
function Get-Report([string]$root) {
    $failures = New-Object System.Collections.Generic.List[string]
    $found = [ordered]@{}
    foreach ($k in $kinds) { $found[$k.Kind] = New-Object System.Collections.Generic.List[string] }
    $srcDir = Join-Path $root 'src'
    if (-not (Test-Path -LiteralPath $srcDir -PathType Container)) {
        $failures.Add('missing: src/, the game''s one folder (MISE.1)')
        return @{ Failures = $failures; Found = $found; Files = 0 }
    }

    # ---- 1. every file under src/ of a kind, and each kind at its pin ----
    $base = (Get-Item -LiteralPath $srcDir).FullName.TrimEnd('\', '/')
    $all = @(Get-ChildItem -LiteralPath $srcDir -Recurse -File -Force | Sort-Object FullName)
    foreach ($f in $all) {
        $rel = $f.FullName.Substring($base.Length + 1).Replace('\', '/')
        $parts = $rel.Split('/')
        $kind = $null
        $unit = 'src/' + $rel
        if ($parts.Count -eq 1) {
            if ($named.ContainsKey($rel)) { $kind = $named[$rel] }
        } else {
            foreach ($k in $kinds) {
                if (-not $k.Folder -or $parts[0] -cne $k.Folder) { continue }
                if ($k.Unit -eq 'file' -and $parts.Count -eq 2 -and $rel.EndsWith('.vla', [System.StringComparison]::Ordinal)) { $kind = $k.Kind }
                elseif ($k.Unit -eq 'folder' -and $parts.Count -ge 3) { $kind = $k.Kind; $unit = 'src/' + $parts[0] + '/' + $parts[1] + '/' }
            }
        }
        if ($null -eq $kind) {
            $failures.Add("stray: src/$rel is no file the layout names: the source, its cartridge, the two libraries, a content sheet (src/content/<name>.vla) or a course (src/courses/<name>/)")
        } elseif (-not $found[$kind].Contains($unit)) {
            $found[$kind].Add($unit)
        }
    }
    foreach ($k in $kinds) {
        $have = $found[$k.Kind]
        foreach ($n in @($k.Names)) {
            if ($n -and -not $have.Contains('src/' + $n)) { $failures.Add("missing: src/$n, the $($k.Kind), $($k.Missing)") }
        }
        if ($have.Count -ne $k.Pinned) {
            $list = ''
            if ($have.Count -gt 0) { $list = ' (' + ($have -join ', ') + ')' }
            $failures.Add("count: $($k.Plural), $($have.Count) under src/$list where the layout pins $($k.Pinned); $($k.Raise)")
        }
    }

    # ---- 2. the cartridge's second line, the pass's banner naming its source ----
    $cartPath = Join-Path $srcDir $cartridgeName
    if (Test-Path -LiteralPath $cartPath -PathType Leaf) {
        $lines = Get-LfLines $cartPath
        $second = ''
        if ($lines.Count -ge 2) { $second = $lines[1] }
        if ($second -cne (Get-Banner $sourceName)) {
            $failures.Add("banner: the second line of src/$cartridgeName is not the pass's banner naming $sourceName, but '$second'; expand the source again: cargo run --release --example expand")
        }
    }

    # ---- 3. the source includes the kit, then the library; the library includes nothing ----
    $sourcePath = Join-Path $srcDir $sourceName
    if (Test-Path -LiteralPath $sourcePath -PathType Leaf) {
        $inc = Get-Includes $sourcePath
        $names = New-Object System.Collections.Generic.List[string]
        foreach ($r in $inc) {
            $names.Add($r.Name)
            if ($r.Name -eq '') { $failures.Add("include: line $($r.Line) of src/$sourceName, '$($r.Text)', is not one include as the pass reads one, a quoted name alone on its line") }
        }
        if (($names -join '|') -cne ($includeOrder -join '|')) {
            $failures.Add("includes: src/$sourceName includes $(Format-Names $names), where the layout wants $(Format-Names $includeOrder), each by a line of its own")
        }
    }
    foreach ($lib in $includesNothing) {
        $p = Join-Path $srcDir $lib
        if (-not (Test-Path -LiteralPath $p -PathType Leaf)) { continue }
        foreach ($r in (Get-Includes $p)) {
            $failures.Add("include: line $($r.Line) of src/$lib, '$($r.Text)': the library includes nothing (MISE.1's decision 2), the source including the kit and the library each by its own line")
        }
    }
    return @{ Failures = $failures; Found = $found; Files = $all.Count }
}

if ($Control) {
    $tmp = Join-Path ([System.IO.Path]::GetTempPath()) ('buffet_layout_' + [System.IO.Path]::GetRandomFileName())
    $tsrc = Join-Path $tmp 'src'
    New-Item -ItemType Directory -Path $tsrc | Out-Null
    try {
        $utf8 = New-Object System.Text.UTF8Encoding($false)
        foreach ($n in (@($sourceName, $cartridgeName) + $includeOrder)) {
            $from = Join-Path (Join-Path $repoRoot 'src') $n
            if (-not (Test-Path -LiteralPath $from -PathType Leaf)) {
                Write-Host "FAIL: control: src/$n is not in this tree, so there is no clean copy to mutate; the plain check says why"
                exit 1
            }
            Copy-Item -LiteralPath $from -Destination (Join-Path $tsrc $n)
        }
        $sourceC = Join-Path $tsrc $sourceName
        $cartC = Join-Path $tsrc $cartridgeName
        $kitchenC = Join-Path $tsrc 'kitchen.vla'
        $source0 = [System.IO.File]::ReadAllBytes($sourceC)
        $cart0 = [System.IO.File]::ReadAllBytes($cartC)
        $kitchen0 = [System.IO.File]::ReadAllBytes($kitchenC)
        $stray = Join-Path $tsrc 'notes.txt'
        $sheet = Join-Path $tsrc 'content/jokes.vla'
        $course = Join-Path $tsrc 'courses/buffet0/buffet0.vla'
        $banner0 = Get-Banner $sourceName
        $banner1 = Get-Banner 'other.source.vla'
        $incKit = '(include "common.vla")'
        $incLib = '(include "kitchen.vla")'

        $verdicts = New-Object System.Collections.Generic.List[string]
        $ok = $true
        $clean = Get-Report $tmp
        if ($clean.Failures.Count -eq 0 -and $clean.Files -eq 4) {
            $verdicts.Add("the clean copy: passes, 4 files read ($(Format-Found $clean.Found))")
        } else {
            $ok = $false
            $verdicts.Add("the clean copy: $($clean.Failures.Count) problem(s), $($clean.Files) file(s) read; wanted 0 and 4: $($clean.Failures -join ' | ')")
        }

        $mutants = @(
            @{ Name = 'a stray file under src/'
               Apply = { [System.IO.File]::WriteAllText($stray, "a note`r`n", $utf8) }
               Applied = { Test-Path -LiteralPath $stray }
               Want = 'stray: src/notes.txt' },
            @{ Name = 'the cartridge missing'
               Apply = { Remove-Item -LiteralPath $cartC -Force }
               Applied = { -not (Test-Path -LiteralPath $cartC) }
               Want = "missing: src/$cartridgeName" },
            @{ Name = 'a banner naming another source'
               Apply = { $t = [System.IO.File]::ReadAllText($cartC, $utf8); $script:hits = [regex]::Matches($t, [regex]::Escape($banner0)).Count; [System.IO.File]::WriteAllText($cartC, $t.Replace($banner0, $banner1), $utf8) }
               Applied = { ($script:hits -eq 1) -and ((Get-LfLines $cartC)[1] -ceq $banner1) }
               Want = "banner: the second line of src/$cartridgeName" },
            @{ Name = 'an include in kitchen.vla'
               Apply = { $t = [System.IO.File]::ReadAllText($kitchenC, $utf8); [System.IO.File]::WriteAllText($kitchenC, $t.Insert($t.IndexOf("`n") + 1, $incKit + "`r`n"), $utf8) }
               Applied = { (Get-LfLines $kitchenC)[1] -ceq $incKit }
               Want = 'include: line 2 of src/kitchen.vla' },
            @{ Name = 'the source''s includes out of order'
               Apply = { $l = Get-LfLines $sourceC; $a = [Array]::IndexOf($l, $incKit); $b = [Array]::IndexOf($l, $incLib); if ($a -ge 0 -and $b -ge 0) { $l[$a] = $incLib; $l[$b] = $incKit }; [System.IO.File]::WriteAllText($sourceC, ($l -join "`r`n"), $utf8) }
               Applied = { $l = Get-LfLines $sourceC; $a = [Array]::IndexOf($l, $incKit); $b = [Array]::IndexOf($l, $incLib); ($a -ge 0) -and ($b -ge 0) -and ($b -lt $a) }
               Want = "includes: src/$sourceName includes kitchen.vla, then common.vla" },
            @{ Name = 'a content sheet before its pin is raised'
               Apply = { New-Item -ItemType Directory -Path (Split-Path -Parent $sheet) -Force | Out-Null; [System.IO.File]::WriteAllText($sheet, "; SPDX-License-Identifier: CC-BY-SA-4.0`r`n", $utf8) }
               Applied = { Test-Path -LiteralPath $sheet }
               Want = 'count: content sheets, 1 under src/ (src/content/jokes.vla)' },
            @{ Name = 'a course before its pin is raised'
               Apply = { New-Item -ItemType Directory -Path (Split-Path -Parent $course) -Force | Out-Null; [System.IO.File]::WriteAllText($course, "; a frozen course`r`n", $utf8) }
               Applied = { Test-Path -LiteralPath $course }
               Want = 'count: courses, 1 under src/ (src/courses/buffet0/)' }
        )
        foreach ($mu in $mutants) {
            $null = & $mu.Apply
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
            [System.IO.File]::WriteAllBytes($sourceC, $source0)
            [System.IO.File]::WriteAllBytes($cartC, $cart0)
            [System.IO.File]::WriteAllBytes($kitchenC, $kitchen0)
            foreach ($extra in @($stray, (Join-Path $tsrc 'content'), (Join-Path $tsrc 'courses'))) {
                if (Test-Path -LiteralPath $extra) { Remove-Item -LiteralPath $extra -Recurse -Force }
            }
        }
        foreach ($v in $verdicts) { Write-Host ('control: ' + $v) }
        if ($ok) { Write-Host 'OK: control: the clean copy passes and each of the seven mutants fails for its own reason'; exit 0 }
        Write-Host 'FAIL: control: the check did not behave as the header says'
        exit 1
    } finally {
        Remove-Item -LiteralPath $tmp -Recurse -Force -ErrorAction SilentlyContinue
    }
}

$report = Get-Report $repoRoot
Write-Host "check_layout: src/ holds $($report.Files) file(s): $(Format-Found $report.Found)"
if ($report.Failures.Count -gt 0) {
    Write-Host "FAIL: $($report.Failures.Count) problem(s)"
    $report.Failures | ForEach-Object { Write-Host "  - $_" }
    exit 1
}
Write-Host "OK: src/ holds exactly the files the layout names, each kind at its pin; the cartridge's second line is the banner naming $sourceName; the source includes $(Format-Names $includeOrder), and $($includesNothing -join ', ') includes nothing"
exit 0
