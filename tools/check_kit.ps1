<#
check_kit.ps1 - the kit's copy, held to the file it copies.

WHY: Buffet's source includes the kit, Alonzo's cartridges/common.vla, the
library of template macros every Alonzo cartridge is written in (docs/OMAKASE.md,
HOUSE.3). The engine comes from crates.io, and its package carries no
cartridges, so the kit cannot come with it: it is copied to src/common.vla,
beside the game's source, at the tag the pin names, byte for byte, under the
0BSD it came with, its name kept so that it diffs against Alonzo's by name.
A copy is worth what its last comparison was worth. An edit made here would
fork the kit silently, and every cartridge expanded over it would then hold
rows no Alonzo cartridge writes. So the copy is held to the SHA-256 of the
file at that tag, read off the tag once and written below. A pattern Buffet
wants in the kit goes to Alonzo by a pull request, and the copy and this
digest follow at the next pin, never ahead of it; a pattern of Buffet's own
goes in src/kitchen.vla.

WHAT IT HOLDS, two things, neither needing a host or the network:
  1. Each file the baseline names is there, today one, src/common.vla.
  2. Its SHA-256, over its text with CRLF read as LF, and its length in LF
     bytes, equal the ones pinned for it. A .vla is checked out CRLF
     (.gitattributes) and the tag stores it LF, so the digest is of the LF
     text, as a replay's digest of its cartridge is; the length says how
     far a copy that fails is from the file.

-Control proves the check on a scratch tree: the clean copy passes; a copy
with one byte changed fails; a copy missing fails. Each mutant proves it
applied before the check is asked.

House style (tools/check_*.ps1): PowerShell 5.1 and pwsh alike, host-free, a
hardcoded and reviewable baseline. Exit 0 clean, exit 1 with every failure
listed.

Usage:  powershell -File tools\check_kit.ps1
        powershell -File tools\check_kit.ps1 -Control
#>
param(
    [switch]$Control
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot

# ---- baseline: each file of the kit, the tag it was copied at, its digest ----
# 2026-10-14, HOUSE.3: Alonzo's cartridges/common.vla at v0.1.0 (fc6dc4f), the
# tag of alonzo 0.1.0, which Cargo.toml pins. The digest and the length are
# `git show v0.1.0:cartridges/common.vla | sha256sum` and `| wc -c` in
# Alonzo's tree, the blob being stored LF.
$baseline = @(
    @{ File = 'src/common.vla'; From = 'Alonzo cartridges/common.vla at v0.1.0'; Sha256 = '315D3611A9CFF6EB2CFAD61FAA57831AFED576AF0982ED89978E8F58BFFC16C9'; Bytes = 23787 }
)

function Get-LfDigest([string]$path) {
    $text = [System.IO.File]::ReadAllText($path, (New-Object System.Text.UTF8Encoding($false)))
    $bytes = (New-Object System.Text.UTF8Encoding($false)).GetBytes($text.Replace("`r`n", "`n"))
    $sha = [System.Security.Cryptography.SHA256]::Create()
    try { $hash = $sha.ComputeHash($bytes) } finally { $sha.Dispose() }
    return @{ Sha256 = (($hash | ForEach-Object { $_.ToString('X2') }) -join ''); Bytes = $bytes.Length }
}

function Get-Report([string]$root) {
    $failures = New-Object System.Collections.Generic.List[string]
    $read = 0
    foreach ($b in $baseline) {
        $p = Join-Path $root $b.File
        if (-not (Test-Path -LiteralPath $p)) { $failures.Add("missing: $($b.File), the copy of $($b.From)"); continue }
        $got = Get-LfDigest $p
        $read++
        if ($got.Sha256 -ne $b.Sha256) {
            $failures.Add("changed: $($b.File) is not $($b.From): SHA-256 $($got.Sha256) over $($got.Bytes) LF bytes, where the copy is $($b.Sha256) over $($b.Bytes); copy the file again from the tag, or move the pin and this baseline together")
        } elseif ($got.Bytes -ne $b.Bytes) {
            $failures.Add("changed: $($b.File) has the copy's digest and $($got.Bytes) LF bytes, where the baseline says $($b.Bytes); the baseline's two numbers disagree")
        }
    }
    return @{ Failures = $failures; Read = $read }
}

if ($Control) {
    $tmp = Join-Path ([System.IO.Path]::GetTempPath()) ('buffet_kit_' + [System.IO.Path]::GetRandomFileName())
    New-Item -ItemType Directory -Path (Join-Path $tmp 'src') | Out-Null
    try {
        $copy = Join-Path $tmp 'src/common.vla'
        Copy-Item -LiteralPath (Join-Path $repoRoot 'src/common.vla') -Destination $copy
        $copy0 = [System.IO.File]::ReadAllBytes($copy)

        $verdicts = New-Object System.Collections.Generic.List[string]
        $ok = $true
        $clean = Get-Report $tmp
        if ($clean.Failures.Count -eq 0 -and $clean.Read -eq 1) {
            $verdicts.Add('the clean copy: passes, 1 file read')
        } else {
            $ok = $false
            $verdicts.Add("the clean copy: $($clean.Failures.Count) problem(s), $($clean.Read) read; wanted 0 and 1: $($clean.Failures -join ' | ')")
        }

        $mutants = @(
            @{ Name = 'a copy with one byte changed'
               Apply = { $b = [byte[]]$copy0.Clone(); $i = [Array]::IndexOf($b, [byte][char]'0'); $b[$i] = [byte][char]'1'; [System.IO.File]::WriteAllBytes($copy, $b) }
               Applied = { $now = [System.IO.File]::ReadAllBytes($copy); ($now.Length -eq $copy0.Length) -and (@(for ($j = 0; $j -lt $now.Length; $j++) { if ($now[$j] -ne $copy0[$j]) { $j } }).Count -eq 1) }
               Want = 'changed: src/common.vla is not' },
            @{ Name = 'a copy missing'
               Apply = { Remove-Item -LiteralPath $copy -Force }
               Applied = { -not (Test-Path -LiteralPath $copy) }
               Want = 'missing: src/common.vla' }
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
            [System.IO.File]::WriteAllBytes($copy, $copy0)
        }
        foreach ($v in $verdicts) { Write-Host ('control: ' + $v) }
        if ($ok) { Write-Host 'OK: control: the clean copy passes and each of the two mutants fails for its own reason'; exit 0 }
        Write-Host 'FAIL: control: the check did not behave as the header says'
        exit 1
    } finally {
        Remove-Item -LiteralPath $tmp -Recurse -Force -ErrorAction SilentlyContinue
    }
}

$report = Get-Report $repoRoot
Write-Host "check_kit: $($report.Read) of $($baseline.Count) kit file(s) read against the digests pinned at the tag"
if ($report.Failures.Count -gt 0) {
    Write-Host "FAIL: $($report.Failures.Count) problem(s)"
    $report.Failures | ForEach-Object { Write-Host "  - $_" }
    exit 1
}
Write-Host 'OK: the kit is the file it copies, byte for byte at the tag the pin names'
exit 0
