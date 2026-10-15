<#
check_layout.ps1 - the game's one folder, src/, held to the layout MISE.1 decided.

WHY: the game lives in one folder, src/ (the owner's word of 2026-10-14,
docs/OMAKASE.md, MISE.1): the source a person edits, the cartridge the pass writes
beside it, Buffet's own library and the kit's copy, the content sheets under
src/content/, and later the frozen courses. A path is the dearest thing to move
later, since the license map, the checks and the page's places name it, and a
folder that gains a file nobody named gains it silently. Rows are the engine's to
judge and not this script's: the oracle crate's tests load the cartridge
through alonzo, hold it to its source expanded and to its describe golden,
and hold every frame of it to the game written by hand, and CI's crate job
runs them on every push. So this is no port of Alonzo's cartridge check,
which reads every row because Alonzo's cartridges have no oracle crate
beside them: it holds what no row says, and needs no cargo to say it.

WHAT IT HOLDS, four things, none needing a host, cargo or the network:
  1. src/ holds exactly the files the baseline names, each of a kind: the
     source, its cartridge, the two libraries, the content sheets and their
     how-to, src/content/README.md. Any other file under src/ fails by name.
     Each kind is counted against the count pinned for it below: sources 1,
     cartridges 1, libraries 2, how-tos 1, content sheets (a .vla of
     src/content/) 1 since MENU.1 wrote the first, and courses (a folder of
     src/courses/) 0 until MISE.1's second slice writes its rule.
  2. The cartridge's second line is the pass's banner naming its source,
     word for word as alonzo::pass::banner writes it under the source's
     header line (check_spdx.ps1 holds that header): a cartridge written by
     hand, or expanded from another source, says so there.
  3. The source includes common.vla, then kitchen.vla, each by a line of its
     own as the pass reads an include, then each content sheet once, as
     content/<name>.vla; the manifest, (cartridge ...), is the source's first
     form, the cartridge's own, then every include, then the code (the
     owner's words of 2026-10-15, at MENU.1's hand test: the includes grouped
     as most languages group their imports, so that a file missing is refused
     at the top, and the manifest before them, the cartridge's metaform); and
     kitchen.vla includes nothing (MISE.1's decision 2): the pass resolves
     every include against the source's own folder, an included file's too,
     so a library that included the kit would name it by a path true at one
     depth only, and a frozen course stands two folders deeper. A content
     sheet the source does not include is words no game shows, and an
     include of one that is not there refuses the pass.
  4. The content rule (MENU.1, decisions 4 and 9, as amended at its hand
     test): a content sheet is named in lower-case letters, digits and
     hyphens, since its name is the page's block id, library-content-<name>
     (tools/build_web.ps1); it holds words alone, one form, a macro named
     after its file whose body is one call of the library's strings, so it
     includes nothing and defines nothing else, the code that writes the
     rows being the library's, AGPL-3.0-or-later, and the words
     CC-BY-SA-4.0, a line this rule keeps where the license map draws it; and
     the source calls that macro once, where the sheet's rows belong. The
     pass splices an include where it stands and takes the first form that
     is no include and no macro for the manifest, which is why a content
     file defines its rows and the source calls them: an include among the
     source's first forms defines macros alone.

-Control proves the check on a scratch copy of the six files: the clean
copy passes; a stray file under src/, the cartridge missing, a banner naming
another source, an include in kitchen.vla, the source's includes out of
order, a second content sheet past its pin, a course before its pin, a
content sheet the source does not include, a content sheet that includes, a
content sheet that defines a second macro, a content sheet with a capital
letter in its name, a content sheet whose macro bears another file's name,
the source's call of a content sheet's macro gone, an include among the
code, and an include before the manifest each fail by name. Each mutant
proves it applied before the check is asked.

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
# which includes nothing. 2026-10-15, MENU.1: the content sheets, a .vla each
# under src/content/, the first the Disclosures sheet's, and the how-to beside
# them. No course yet.
$sourceName = 'buffet.source.vla'
$cartridgeName = 'buffet.vla'
$includeOrder = @('common.vla', 'kitchen.vla')
$includesNothing = @('kitchen.vla')
$contentFolder = 'content'
$howTo = 'content/README.md'
# A content sheet's name: lower-case letters, digits and hyphens, the page's
# block id (library-content-<name>), which check_web_offline.ps1 reads as
# [a-z0-9_-]; case and all, since a name that differs only in case is
# another file to Linux, where CI builds.
$contentName = '^[a-z][a-z0-9-]*\.vla$'
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
    @{ Kind = 'how-to'; Plural = 'how-tos'; Pinned = 1; Names = @($howTo)
       Missing = 'the How to add a row beside the content sheets, for a stranger with five minutes (MENU.1)'
       Raise = 'a file added or removed moves its name and this pin together, in this file' },
    @{ Kind = 'content sheet'; Plural = 'content sheets'; Pinned = 1; Folder = $contentFolder; Unit = 'file'
       Raise = 'a sheet arrives with the dish that reads it (MENU.1''s decision 1), which raises this pin and adds its include to the source' },
    @{ Kind = 'course'; Plural = 'courses'; Pinned = 0; Folder = 'courses'; Unit = 'folder'
       Raise = 'the courses'' rule, MISE.1''s second slice (COURSE.1), raises this pin' }
)

# A file's path under src/ to its kind, case and all: a name that differs
# only in case is another file to Linux, where CI builds.
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
        if ($named.ContainsKey($rel)) {
            $kind = $named[$rel]
        } elseif ($parts.Count -gt 1) {
            foreach ($k in $kinds) {
                if (-not $k.Folder -or $parts[0] -cne $k.Folder) { continue }
                if ($k.Unit -eq 'file' -and $parts.Count -eq 2 -and $rel.EndsWith('.vla', [System.StringComparison]::Ordinal)) { $kind = $k.Kind }
                elseif ($k.Unit -eq 'folder' -and $parts.Count -ge 3) { $kind = $k.Kind; $unit = 'src/' + $parts[0] + '/' + $parts[1] + '/' }
            }
        }
        if ($null -eq $kind) {
            $failures.Add("stray: src/$rel is no file the layout names: the source, its cartridge, the two libraries, a content sheet (src/content/<name>.vla) and its how-to (src/$howTo), or a course (src/courses/<name>/)")
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

    # ---- 3. the source includes the kit, then the library, then each content sheet once; the library includes nothing ----
    $sheets = @($found['content sheet'] | ForEach-Object { $_.Substring('src/'.Length) })
    $sourcePath = Join-Path $srcDir $sourceName
    $sourceLines = @()
    if (Test-Path -LiteralPath $sourcePath -PathType Leaf) {
        $sourceLines = Get-LfLines $sourcePath
        # The source's forms by the house's layout, each opening a line in its first column, and
        # an include wherever it stands: the manifest first, then every include, then the code.
        $forms = New-Object System.Collections.Generic.List[object]
        for ($i = 0; $i -lt $sourceLines.Count; $i++) {
            $line = $sourceLines[$i]
            $isInclude = $null -ne (Read-Include $line)
            if ($line.StartsWith('(') -or $isInclude) { $forms.Add(@{ Line = $i + 1; Text = $line.Trim(); Include = $isInclude; Manifest = ($line.Trim() -imatch '^\(cartridge(\s|$)') }) }
        }
        if ($forms.Count -gt 0 -and -not $forms[0].Manifest) {
            $failures.Add("order: the first form of src/$sourceName, at line $($forms[0].Line), is '$($forms[0].Text)', where the manifest, (cartridge ...), stands first, the cartridge's own form, then every include, then the code (the owner's word of 2026-10-15)")
        }
        $k = 1
        while ($k -lt $forms.Count -and $forms[$k].Include) { $k++ }
        for ($j = $k; $j -lt $forms.Count; $j++) {
            if ($forms[$j].Include) { $failures.Add("order: line $($forms[$j].Line) of src/$sourceName, '$($forms[$j].Text)', stands among the code, after line $($forms[$k].Line)'s form: every include follows the manifest, before any other form (the owner's word of 2026-10-15)") }
        }
        $inc = Get-Includes $sourcePath
        $names = New-Object System.Collections.Generic.List[string]
        foreach ($r in $inc) {
            $names.Add($r.Name)
            if ($r.Name -eq '') { $failures.Add("include: line $($r.Line) of src/$sourceName, '$($r.Text)', is not one include as the pass reads one, a quoted name alone on its line") }
        }
        $libs = @($names | Select-Object -First $includeOrder.Count)
        if (($libs -join '|') -cne ($includeOrder -join '|')) {
            $failures.Add("includes: src/$sourceName includes $(Format-Names $names), where the layout wants $(Format-Names $includeOrder), each by a line of its own, then each content sheet once")
        }
        $seen = New-Object System.Collections.Generic.List[string]
        foreach ($n in @($names | Select-Object -Skip $includeOrder.Count)) {
            if ($n -eq '') { continue }
            if ($seen.Contains($n)) { $failures.Add("include: src/$sourceName includes $n twice"); continue }
            $seen.Add($n)
            if ($sheets -cnotcontains $n) { $failures.Add("include: src/$sourceName includes $n, which is no content sheet under src/$contentFolder/; the pass would refuse it") }
        }
        foreach ($s in $sheets) {
            if (-not $seen.Contains($s)) { $failures.Add("include: src/$s is a content sheet the source does not include, words no game shows; include it after the manifest with the others, as (include ""$s""), and call its macro where the sheet belongs") }
        }
    }
    foreach ($lib in $includesNothing) {
        $p = Join-Path $srcDir $lib
        if (-not (Test-Path -LiteralPath $p -PathType Leaf)) { continue }
        foreach ($r in (Get-Includes $p)) {
            $failures.Add("include: line $($r.Line) of src/$lib, '$($r.Text)': the library includes nothing (MISE.1's decision 2), the source including the kit and the library each by its own line")
        }
    }

    # ---- 4. the content rule: a sheet's name the page's block id, words alone, and its macro called ----
    foreach ($s in $sheets) {
        $file = $s.Substring($contentFolder.Length + 1)
        $base = [System.IO.Path]::GetFileNameWithoutExtension($file)
        if ($file -cnotmatch $contentName) {
            $failures.Add("name: src/$s is named in other than lower-case letters, digits and hyphens, which its block id in the page needs, library-content-<name> (MENU.1's decision 9)")
        }
        $p = Join-Path $srcDir $s
        foreach ($r in (Get-Includes $p)) {
            $failures.Add("include: line $($r.Line) of src/$s, '$($r.Text)': a content sheet includes nothing, its words one macro of the library's strings (MENU.1's decision 4)")
        }
        # Its forms, by the house's layout: a form at the top level opens in the line's first column.
        $lines = Get-LfLines $p
        $tops = @($lines | Where-Object { $_.StartsWith('(') })
        $defined = @($tops | Where-Object { $_ -match '^\(defmacro\s+\(([^\s()]+)\)' } | ForEach-Object { [regex]::Match($_, '^\(defmacro\s+\(([^\s()]+)\)').Groups[1].Value })
        $calls = @($lines | Where-Object { $_.Trim().StartsWith('(strings ', [System.StringComparison]::Ordinal) })
        if ($tops.Count -ne 1 -or $defined.Count -ne 1) {
            $failures.Add("macro: src/$s holds $($tops.Count) form(s) at its top level, $($defined.Count) of them a macro, where a content sheet holds one, (defmacro ($base) ...), its body one call of the library's strings (MENU.1's decision 4, as amended)")
        } elseif ($defined[0] -cne $base) {
            $failures.Add("macro: src/$s defines ($($defined[0])), where its file's name wants ($base), the call the source makes")
        }
        if ($calls.Count -ne 1) {
            $failures.Add("macro: src/$s calls the library's strings $($calls.Count) time(s), where its macro's body is one call of it")
        }
        $called = @($sourceLines | Where-Object { $_.Trim() -cmatch ('^\(' + [regex]::Escape($base) + '\)(\s|;|$)') })
        if ($called.Count -ne 1) {
            $failures.Add("call: src/$sourceName calls ($base) $($called.Count) time(s), where it calls each content sheet's macro once, after the manifest, where the sheet's rows belong")
        }
    }
    return @{ Failures = $failures; Found = $found; Files = $all.Count }
}

if ($Control) {
    $tmp = Join-Path ([System.IO.Path]::GetTempPath()) ('buffet_layout_' + [System.IO.Path]::GetRandomFileName())
    $tsrc = Join-Path $tmp 'src'
    New-Item -ItemType Directory -Path (Join-Path $tsrc $contentFolder) -Force | Out-Null
    try {
        $utf8 = New-Object System.Text.UTF8Encoding($false)
        $sheetName = 'disclosures.vla'
        $copied = @($sourceName, $cartridgeName) + $includeOrder + @($howTo, "$contentFolder/$sheetName")
        foreach ($n in $copied) {
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
        $contentC = Join-Path $tsrc $contentFolder
        $sheetC = Join-Path $contentC $sheetName
        $howToC = Join-Path $tsrc $howTo
        $source0 = [System.IO.File]::ReadAllBytes($sourceC)
        $cart0 = [System.IO.File]::ReadAllBytes($cartC)
        $kitchen0 = [System.IO.File]::ReadAllBytes($kitchenC)
        $sheet0 = [System.IO.File]::ReadAllBytes($sheetC)
        $howTo0 = [System.IO.File]::ReadAllBytes($howToC)
        $stray = Join-Path $tsrc 'notes.txt'
        $second = Join-Path $contentC 'jokes.vla'
        $capital = Join-Path $contentC 'Disclosures.vla'
        $course = Join-Path $tsrc 'courses/buffet0/buffet0.vla'
        $banner0 = Get-Banner $sourceName
        $banner1 = Get-Banner 'other.source.vla'
        $incKit = '(include "common.vla")'
        $incLib = '(include "kitchen.vla")'
        $incSheet = "(include ""$contentFolder/$sheetName"")"

        $verdicts = New-Object System.Collections.Generic.List[string]
        $ok = $true
        $clean = Get-Report $tmp
        if ($clean.Failures.Count -eq 0 -and $clean.Files -eq 6) {
            $verdicts.Add("the clean copy: passes, 6 files read ($(Format-Found $clean.Found))")
        } else {
            $ok = $false
            $verdicts.Add("the clean copy: $($clean.Failures.Count) problem(s), $($clean.Files) file(s) read; wanted 0 and 6: $($clean.Failures -join ' | ')")
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
            @{ Name = 'a second content sheet past its pin'
               Apply = { [System.IO.File]::WriteAllText($second, "; SPDX-License-Identifier: CC-BY-SA-4.0`r`n", $utf8) }
               Applied = { Test-Path -LiteralPath $second }
               Want = "count: content sheets, 2 under src/ (src/$contentFolder/$sheetName, src/$contentFolder/jokes.vla)" },
            @{ Name = 'a course before its pin is raised'
               Apply = { New-Item -ItemType Directory -Path (Split-Path -Parent $course) -Force | Out-Null; [System.IO.File]::WriteAllText($course, "; a frozen course`r`n", $utf8) }
               Applied = { Test-Path -LiteralPath $course }
               Want = 'count: courses, 1 under src/ (src/courses/buffet0/)' },
            @{ Name = 'a content sheet the source does not include'
               Apply = { $l = Get-LfLines $sourceC; $script:hits = @($l | Where-Object { $_ -ceq $incSheet }).Count; [System.IO.File]::WriteAllText($sourceC, ((@($l | Where-Object { $_ -cne $incSheet })) -join "`r`n"), $utf8) }
               Applied = { $l = Get-LfLines $sourceC; ($script:hits -eq 1) -and (@($l | Where-Object { $_ -ceq $incSheet }).Count -eq 0) }
               Want = "include: src/$contentFolder/$sheetName is a content sheet the source does not include" },
            @{ Name = 'a content sheet that includes'
               Apply = { $t = [System.IO.File]::ReadAllText($sheetC, $utf8); [System.IO.File]::WriteAllText($sheetC, $t.Insert($t.IndexOf("`n") + 1, $incKit + "`r`n"), $utf8) }
               Applied = { (Get-LfLines $sheetC)[1] -ceq $incKit }
               Want = "include: line 2 of src/$contentFolder/$sheetName" },
            @{ Name = 'a content sheet that defines a second macro'
               Apply = { $t = [System.IO.File]::ReadAllText($sheetC, $utf8); [System.IO.File]::WriteAllText($sheetC, $t + "(defmacro (aside) ""a macro in a content file"" (cell ""Disclosures"" ""H1"" 1))`r`n", $utf8) }
               Applied = { $l = Get-LfLines $sheetC; @($l | Where-Object { $_ -clike '(defmacro (aside)*' }).Count -eq 1 }
               Want = "macro: src/$contentFolder/$sheetName holds 2 form(s) at its top level, 2 of them a macro" },
            @{ Name = 'a content sheet with a capital letter in its name'
               Apply = { Move-Item -LiteralPath $sheetC -Destination ($sheetC + '.tmp'); Move-Item -LiteralPath ($sheetC + '.tmp') -Destination $capital }
               Applied = { @(Get-ChildItem -LiteralPath $contentC -File | Where-Object { $_.Name -ceq 'Disclosures.vla' }).Count -eq 1 }
               Want = "name: src/$contentFolder/Disclosures.vla" },
            @{ Name = 'a content sheet whose macro bears another file''s name'
               Apply = { $t = [System.IO.File]::ReadAllText($sheetC, $utf8); $script:hits = [regex]::Matches($t, [regex]::Escape('(defmacro (disclosures)')).Count; [System.IO.File]::WriteAllText($sheetC, $t.Replace('(defmacro (disclosures)', '(defmacro (jokes)'), $utf8) }
               Applied = { $l = Get-LfLines $sheetC; ($script:hits -eq 1) -and (@($l | Where-Object { $_ -clike '(defmacro (jokes)*' }).Count -eq 1) }
               Want = "macro: src/$contentFolder/$sheetName defines (jokes), where its file's name wants (disclosures)" },
            @{ Name = 'the source''s call of a content sheet''s macro gone'
               Apply = { $l = Get-LfLines $sourceC; $script:hits = @($l | Where-Object { $_.Trim().StartsWith('(disclosures)') }).Count; [System.IO.File]::WriteAllText($sourceC, ((@($l | Where-Object { -not $_.Trim().StartsWith('(disclosures)') })) -join "`r`n"), $utf8) }
               Applied = { $l = Get-LfLines $sourceC; ($script:hits -eq 1) -and (@($l | Where-Object { $_.Trim().StartsWith('(disclosures)') }).Count -eq 0) }
               Want = "call: src/$sourceName calls (disclosures) 0 time(s)" },
            @{ Name = 'an include among the code'
               Apply = { $l = Get-LfLines $sourceC; $l = @($l | Where-Object { $_ -cne $incSheet }); [System.IO.File]::WriteAllText($sourceC, (($l + @($incSheet, '')) -join "`r`n"), $utf8) }
               Applied = { $l = Get-LfLines $sourceC; $at = [Array]::IndexOf($l, $incSheet); $m = -1; for ($i = 0; $i -lt $l.Count; $i++) { if ($l[$i].StartsWith('(cartridge')) { $m = $i; break } }; ($at -gt $m) -and ($m -ge 0) }
               Want = "'$incSheet', stands among the code" },
            @{ Name = 'an include before the manifest'
               Apply = { $l = Get-LfLines $sourceC; $l = @($l | Where-Object { $_ -cne $incKit }); $m = -1; for ($i = 0; $i -lt $l.Count; $i++) { if ($l[$i].StartsWith('(cartridge')) { $m = $i; break } }; $out = New-Object System.Collections.Generic.List[string]; for ($i = 0; $i -lt $l.Count; $i++) { if ($i -eq $m) { $out.Add($incKit) }; $out.Add($l[$i]) }; [System.IO.File]::WriteAllText($sourceC, ($out -join "`r`n"), $utf8) }
               Applied = { $l = Get-LfLines $sourceC; $at = [Array]::IndexOf($l, $incKit); $m = -1; for ($i = 0; $i -lt $l.Count; $i++) { if ($l[$i].StartsWith('(cartridge')) { $m = $i; break } }; ($at -ge 0) -and ($m -ge 0) -and ($at -lt $m) }
               Want = "order: the first form of src/$sourceName" }
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
            # put the tree back: the four files, the content folder as copied, nothing else
            [System.IO.File]::WriteAllBytes($sourceC, $source0)
            [System.IO.File]::WriteAllBytes($cartC, $cart0)
            [System.IO.File]::WriteAllBytes($kitchenC, $kitchen0)
            foreach ($extra in @($stray, $contentC, (Join-Path $tsrc 'courses'))) {
                if (Test-Path -LiteralPath $extra) { Remove-Item -LiteralPath $extra -Recurse -Force }
            }
            New-Item -ItemType Directory -Path $contentC | Out-Null
            [System.IO.File]::WriteAllBytes($sheetC, $sheet0)
            [System.IO.File]::WriteAllBytes($howToC, $howTo0)
        }
        foreach ($v in $verdicts) { Write-Host ('control: ' + $v) }
        if ($ok) { Write-Host "OK: control: the clean copy passes and each of the $($mutants.Count) mutants fails for its own reason"; exit 0 }
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
$sheetCount = $report.Found['content sheet'].Count
Write-Host "OK: src/ holds exactly the files the layout names, each kind at its pin; the cartridge's second line is the banner naming $sourceName; the source includes $(Format-Names $includeOrder), then its $sheetCount content sheet(s), each once, every include after the manifest, its first form, and before the code; $($includesNothing -join ', ') and the content sheets include nothing; each content sheet is one macro named after its file, which the source calls once"
exit 0
