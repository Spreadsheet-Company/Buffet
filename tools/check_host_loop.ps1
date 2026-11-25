<#
check_host_loop.ps1 - the game's page in a headless browser: its loop
oracle's cases run over Buffet's own blocks and judged by Buffet's own
floors, the owner's session among them, every frame held to the hand.

WHY: the page, web/index.template.html filled by tools/build_web.ps1, is
Alonzo's, vendored unchanged; its loop oracle (?loop=1) runs on a game's
page the twenty cases that need nothing of Alonzo's own files, clipboard and
file among them since alonzo 0.5.0, and says the fifteen others are not on it
(Alonzo's ENGINE.16). Alonzo's loop check holds
a game's page only for its own, a page of Kairos alone that it builds, and
its -Page judges any page it is given as Alonzo's own: over Buffet's it
printed every case OK and failed, "the oracle ran 15 case(s), not the 29
pinned here" (docs/SPIKE.md, HOUSE.4's second slice). So the page is held
here by a check of Buffet's own: Alonzo's tools/check_host_loop.ps1 at
v0.4.0, adapted by the same copyright holder under this repository's terms
(decision 6), its browser, its runs and its judgment carried, its floors
Buffet's and its mutants the session's. At the pin to alonzo 0.5.0 the case
list follows Alonzo's at v0.5.0, whose judgment is v0.4.0's unchanged.

WHAT IT HOLDS: the built page, web/index.html (or -Page), run headless at a
ratio of 1, every case, in four runs side by side: the replay case, the save
case, the keep case and the rest; and at a ratio of 1.5 every case but
those three, which draw nothing a ratio moves. Each run is judged: its first
line the loop oracle's header, its ratio the run's, each case it asked for
OK, each count at or above its floor, the counts that must be none none
(live allocations, misfrees, a fill past a sheet, a replay held alone), the
fifteen cases of Alonzo's own page each saying it is not on this one, and
its last line 'loop ok cases=N' at the run's count. The replay case's
floors are the session's own: its digests the standard's, one hand
registered, one replay, its golden and its round, none alone; its frames
held the replay's last, read from src/buffet.replay.vla, and the frames and
cells the hand held its table's, read from src/buffet.hand.js, so that a
session restamped or played again moves no number here. A page built
without the engine's module runs the double's cases alone, and the floors
that need the module (WithModule) ask nothing of it: the ratchets job's.

-Control builds five mutant pages from the real one, each one block of it
changed, and runs the replay case over each, the session to the frame its
mutant lands at where the golden and the round are not what is mutated:
the hand a frame late, failing at the first frame it holds where a cell
moved since the frame before; the hand's first fill a cent off, the firm's
cash at the frame the first order filled; the hand's block dropped, the
replay held alone; a cell of the golden changed; and a frame's only row cut
from the replay, its round failing, since the page's recorder writes no
frame that has no rows but the last. Each must fail by its own words.

The browser: -Browser, else BUFFET_BROWSER or ALONZO_BROWSER, else Google
Chrome, Microsoft Edge or Chromium at their known paths or on the PATH;
skipped, and said so, without a page or a browser. -Jobs, the browsers run
side by side: by default half the logical processors, at most a browser a
0.6 GB of the memory free, and at most four; 1 is one at a time.

House style (tools/check_*.ps1): PowerShell 5.1 and pwsh alike, host-free,
no network; exit 0 clean, exit 1 with every problem named. A headless run's
clock is virtual (--virtual-time-budget), so the counts are the page's own
and the same on every machine.

Usage:  powershell -File tools\check_host_loop.ps1
        powershell -File tools\check_host_loop.ps1 -Control
        powershell -File tools\check_host_loop.ps1 -Page C:\somewhere\index.html -Jobs 1
#>
param(
    [string]$Page = '',
    [string]$Browser = '',
    [switch]$Control,
    [int]$Jobs = 0
)
$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
if ($Page -eq '') { $Page = Join-Path $repoRoot 'web/index.html' }
elseif (-not [System.IO.Path]::IsPathRooted($Page)) { $Page = Join-Path $repoRoot $Page }
# Not $IsWindows: that name is a read-only automatic variable under PowerShell 7.
$onWindows = ($env:OS -eq 'Windows_NT')
function Get-FreeGB {
    try { if ($onWindows) { return [double](Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB } } catch { }
    try { $m = [regex]::Match([System.IO.File]::ReadAllText('/proc/meminfo'), 'MemAvailable:\s+(\d+) kB'); if ($m.Success) { return [double]$m.Groups[1].Value / 1MB } } catch { }
    return $null
}
if ($Jobs -le 0 -and $env:BUFFET_LOOP_JOBS) { $Jobs = [int]$env:BUFFET_LOOP_JOBS }
if ($Jobs -le 0) {
    $Jobs = [Math]::Max(1, [Math]::Min(4, [int][Math]::Floor([Environment]::ProcessorCount / 2)))
    $freeGB = Get-FreeGB
    if ($null -ne $freeGB) { $Jobs = [Math]::Max(1, [Math]::Min($Jobs, [int][Math]::Floor($freeGB / 0.6))) }
}

# --- the session's own numbers, read from its files ---
$replayPath = Join-Path $repoRoot 'src/buffet.replay.vla'
$handPath = Join-Path $repoRoot 'src/buffet.hand.js'
function Get-Session {
    $s = @{ Last = 0; Held = 0; Cells = 0; FirstFill = 0 }
    if (Test-Path -LiteralPath $replayPath) {
        $m = [regex]::Matches([System.IO.File]::ReadAllText($replayPath), '(?m)^\(frame (\d+)')
        if ($m.Count) { $s.Last = [int]$m[$m.Count - 1].Groups[1].Value }
    }
    if (Test-Path -LiteralPath $handPath) {
        $hand = [System.IO.File]::ReadAllText($handPath)
        $cells = [regex]::Match($hand, '(?s)var CELLS = \[(.*?)\];')
        if ($cells.Success) { $s.Cells = ([regex]::Matches($cells.Groups[1].Value, '"[^"]*"')).Count }
        $s.Held = ([regex]::Matches($hand, '(?m)^    \[\d+')).Count
    }
    return $s
}
$session = Get-Session

# --- the floors: each case's counts on Buffet's page, never lowered ---
# 2026-11-21, HOUSE.4's second slice: measured under headless Chrome over the page built at 73c9c26 and the
# session's files, at a ratio of 1, every case OK, the counts a function of the page and its blocks. The replay
# case's session counts are the files' own (above); every other floor is the count measured that day.
$floors = @(
    @{ Case = 'abi';      Min = @{ refused = 3 }; WithModule = @{ attached = 1 } },
    @{ Case = 'pass';     Min = @{}; WithModule = @{ sources = 1; same = 1 } },
    @{ Case = 'replay';   Min = @{ vectors = 5; hands = 1 }; WithModule = @{ replays = 1; golden = 1; round = 1; buffetFrames = $session.Last; buffetCells = $session.Held * $session.Cells; buffetHeld = $session.Held } },
    @{ Case = 'grid';     Min = @{}; WithModule = @{} },
    @{ Case = 'empty';    Min = @{ cartridges = 1; ordered = 1; titled = 1; libraries = 1; double = 1 }; WithModule = @{ engine = 1 } },
    @{ Case = 'still';    Min = @{}; WithModule = @{} },
    @{ Case = 'views';    Min = @{}; WithModule = @{} },
    @{ Case = 'changes';  Min = @{}; WithModule = @{} },
    @{ Case = 'continue'; Min = @{}; WithModule = @{} },
    @{ Case = 'palette';  Min = @{} },
    @{ Case = 'drawer';   Min = @{} },
    @{ Case = 'keys';     Min = @{} },
    @{ Case = 'commands'; Min = @{}; WithModule = @{} },
    @{ Case = 'settle';   Min = @{}; WithModule = @{} },
    @{ Case = 'widths';   Min = @{}; WithModule = @{} },
    @{ Case = 'modes';    Min = @{}; WithModule = @{} },
    @{ Case = 'save';     Min = @{}; WithModule = @{ saves = 1; opened = 1; seeds = 1; resumed = 1; owing = 1; owed = 1 } },
    @{ Case = 'keep';     Min = @{}; WithModule = @{ cartridges = 1; restores = 1; held = 1; owing = 1; owed = 1 } },
    @{ Case = 'clipboard'; Min = @{}; WithModule = @{} },
    @{ Case = 'file';     Min = @{}; WithModule = @{} }
)
# 2026-11-24, the pin to alonzo 0.5.0: twenty cases, clipboard and file over the page's own cards, Brickwork's cut and
# Afterlife's reel left to Alonzo's page (game 0, gate 0); and Alonzo's ENGINE.30's decision 13 over the session, each
# the floor of its first count: a second save where an order's derived writes wait, opened owing them as its run owes
# them (owing 1) and played on beside the run, every frame compared the same (owed 1), saves 2 in all over 66,537
# frames; and the keep case back, 2,081 restores held of 2,081, four of them owing the writes the run owed there
# (owing 4), and a keyframe taken where an order's writes wait, restored at the next frame at rest, held (owed 1).
# The counts no run may count: a live allocation or a misfree, a fill past its sheet, and a replay held to its golden
# and its recording alone, with no hand.
$zero = @('live', 'misfrees', 'filled', 'alone')
# Alonzo's own page's cases, each of which says it is not on a game's page.
$absent = @('rate30', 'rate0', 'yield', 'slice', 'pace', 'memory', 'refusals', 'scale', 'engine', 'parity', 'plane', 'input', 'audio', 'sound', 'tabs')
$expectedCases = 20
$scales = @('1', '1.5')
# The engine's artifact: a page built beside it must hold it.
$artifact = Join-Path $repoRoot 'target/wasm32-unknown-unknown/release/alonzo.wasm'

function Find-Browser([string]$named) {
    if ($named -ne '') { if (Test-Path -LiteralPath $named) { return $named }; return '' }
    foreach ($v in @($env:BUFFET_BROWSER, $env:ALONZO_BROWSER)) { if ($v) { if (Test-Path -LiteralPath $v) { return $v }; return '' } }
    if ($onWindows) {
        $candidates = @(
            (Join-Path $env:ProgramFiles 'Google\Chrome\Application\chrome.exe'),
            (Join-Path ${env:ProgramFiles(x86)} 'Google\Chrome\Application\chrome.exe'),
            (Join-Path ${env:ProgramFiles(x86)} 'Microsoft\Edge\Application\msedge.exe'),
            (Join-Path $env:ProgramFiles 'Microsoft\Edge\Application\msedge.exe'),
            (Join-Path $env:ProgramFiles 'Chromium\Application\chrome.exe')
        )
        foreach ($c in $candidates) { if ($c -and (Test-Path -LiteralPath $c)) { return $c } }
        return ''
    }
    foreach ($name in @('google-chrome', 'google-chrome-stable', 'chromium-browser', 'chromium', 'microsoft-edge', 'microsoft-edge-stable')) {
        $cmd = Get-Command $name -ErrorAction SilentlyContinue
        if ($cmd) { return $cmd.Source }
    }
    return ''
}
# A page's address with its cases: replay:buffet:N plays buffet's committed replay to its frame N and no further.
function Get-PageUrl([string]$path, [string]$cases = '') {
    $slashed = [System.IO.Path]::GetFullPath($path) -replace '\\', '/'
    $names = New-Object System.Collections.Generic.List[string]
    $replays = $null
    $upto = $null
    foreach ($c in @($cases -split ',' | Where-Object { $_ -ne '' })) {
        if ($c -notlike 'replay:*') { $names.Add($c); continue }
        $names.Add('replay')
        $parts = $c.Substring(7) -split ':'
        $replays = $parts[0]
        if ($parts.Count -gt 1) { $upto = $parts[1] }
    }
    $query = '?loop=1' + $(if ($names.Count) { '&cases=' + ($names -join ',') } else { '' }) + $(if ($null -ne $replays) { '&replays=' + $replays } else { '' }) + $(if ($null -ne $upto) { '&upto=' + $upto } else { '' })
    if ($slashed.StartsWith('/')) { return 'file://' + $slashed + $query }
    return 'file:///' + $slashed + $query
}
function Get-CaseNames([string]$cases) { return @($cases -split ',' | Where-Object { $_ -ne '' } | ForEach-Object { ($_ -split ':')[0] }) }
function Remove-LoopTemp([string]$dir) {
    for ($i = 0; $i -lt 100 -and (Test-Path -LiteralPath $dir); $i++) {
        Remove-Item -Recurse -Force $dir -ErrorAction SilentlyContinue
        if (Test-Path -LiteralPath $dir) { Start-Sleep -Milliseconds 50 }
    }
}
# A run past ten minutes is ended and said by its page and its cases (Alonzo's 0.3.0 CI); a third such stops the check.
$script:LoopRunLimit = 600
$script:LoopRunsOver = New-Object System.Collections.Generic.List[string]
# One headless run started: the browser over the page with its cases, its DOM dumped into a folder of its own, its
# profile there too, so that runs side by side share nothing.
function Start-LoopRun([string]$browser, [string]$page, [string]$scale, [string]$cases = '') {
    $tmp = Join-Path ([System.IO.Path]::GetTempPath()) ('buffet_loop_' + [System.IO.Path]::GetRandomFileName())
    New-Item -ItemType Directory -Path $tmp | Out-Null
    $dump = Join-Path $tmp 'dump.html'
    $err = Join-Path $tmp 'err.txt'
    $argList = @('--headless=new', '--disable-gpu', '--run-all-compositor-stages-before-draw', '--disable-frame-rate-limit', '--disable-gpu-vsync',
                 '--virtual-time-budget=60000', ('--force-device-scale-factor=' + $scale), '--window-size=1600,1200', '--no-first-run', '--no-default-browser-check',
                 ('--user-data-dir="' + (Join-Path $tmp 'profile') + '"'))
    if (-not $onWindows) { $argList += @('--no-sandbox', '--disable-dev-shm-usage') }
    $argList += @('--dump-dom', ('"' + (Get-PageUrl $page $cases) + '"'))
    $start = @{ FilePath = $browser; ArgumentList = $argList; RedirectStandardOutput = $dump; RedirectStandardError = $err; PassThru = $true }
    if ($onWindows) { $start['NoNewWindow'] = $true }
    $watch = [System.Diagnostics.Stopwatch]::StartNew()
    $proc = Start-Process @start
    return @{ Proc = $proc; Tmp = $tmp; Dump = $dump; Watch = $watch; Size = -1; Seconds = $null; Over = $false; Page = $page; Cases = $cases; Err = $err }
}
# A run ended: its browser exited, or its dump whole, ending in </html> and grown no more since the last look, the
# browser ended then (a browser beside another may wait on its shutdown long after its page is done).
function Test-LoopRunDone($run) {
    if ($null -ne $run.Seconds) { return $true }
    if (-not $run.Proc.HasExited -and $run.Watch.Elapsed.TotalSeconds -ge $script:LoopRunLimit) {
        $run.Over = $true
        try { $run.Proc.Kill() } catch {}
    }
    elseif (-not $run.Proc.HasExited) {
        if (-not (Test-Path -LiteralPath $run.Dump)) { return $false }
        $fs = $null
        try {
            $fs = New-Object System.IO.FileStream($run.Dump, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read, [System.IO.FileShare]::ReadWrite)
            $n = $fs.Length
            if ($n -lt 16 -or $n -ne $run.Size) { $run.Size = $n; return $false }
            $k = [int][Math]::Min(64, $n)
            $buf = New-Object byte[] $k
            $null = $fs.Seek($n - $k, [System.IO.SeekOrigin]::Begin)
            $null = $fs.Read($buf, 0, $k)
            if ([System.Text.Encoding]::UTF8.GetString($buf).TrimEnd() -notlike '*</html>') { return $false }
        } catch { return $false } finally { if ($fs) { $fs.Dispose() } }
        try { $run.Proc.Kill() } catch {}
    }
    $run.Seconds = $run.Watch.Elapsed.TotalSeconds
    $null = $run.Proc.WaitForExit(60000)
    return $true
}
# A started run's answer: the loop element's text, or '' with the reason, and its seconds.
function Complete-LoopRun($run, [switch]$Defer) {
    while (-not (Test-LoopRunDone $run)) { Start-Sleep -Milliseconds 50 }
    $seconds = $run.Seconds
    try {
        if ($run.Over) {
            $wrote = $(if (Test-Path -LiteralPath $run.Dump) { (Get-Item -LiteralPath $run.Dump).Length } else { 0 })
            $over = "the browser over {0} with the cases '{1}' had not ended after {2:N0} s, {3:N0} bytes of its dump written: ended by the run's limit" -f (Split-Path -Leaf $run.Page), $run.Cases, $seconds, $wrote
            $script:LoopRunsOver.Add($over)
            if ($script:LoopRunsOver.Count -ge 3) { throw ("three runs ended by the run's limit, so the check stops here: " + ($script:LoopRunsOver -join '; ')) }
            return @{ Text = ''; Reason = $over; Seconds = $seconds }
        }
        if (-not (Test-Path -LiteralPath $run.Dump)) { return @{ Text = ''; Reason = 'the browser wrote no dump'; Seconds = $seconds } }
        $fs = New-Object System.IO.FileStream($run.Dump, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read, [System.IO.FileShare]::ReadWrite)
        try { $html = (New-Object System.IO.StreamReader($fs, (New-Object System.Text.UTF8Encoding($false)))).ReadToEnd() } finally { $fs.Dispose() }
        $m = [regex]::Match($html, '(?s)<pre id="loop"[^>]*>(.*?)</pre>')
        if (-not $m.Success) { return @{ Text = ''; Reason = ("no loop block in the dump ({0:N0} characters)" -f $html.Length); Seconds = $seconds } }
        $text = [System.Net.WebUtility]::HtmlDecode($m.Groups[1].Value).Trim()
        if ($text -eq '') { return @{ Text = ''; Reason = 'the loop block is empty'; Seconds = $seconds } }
        return @{ Text = $text; Reason = ''; Seconds = $seconds }
    } finally {
        if (-not $Defer) { Remove-LoopTemp $run.Tmp }
    }
}
# Runs side by side: each run of $plan, @{ Key; Page; Scale; Cases }, started while fewer than $jobs run, each
# answered into $results by its key, their folders swept once every run has ended. $plan is typed an array: in
# Windows PowerShell 5.1, @($v) over a generic list held by a variable fails to compile.
function Invoke-LoopPool([object[]]$plan, [int]$jobs, $results) {
    $queue = New-Object System.Collections.Generic.Queue[object]
    foreach ($p in @($plan)) { $queue.Enqueue($p) }
    $running = New-Object System.Collections.Generic.List[object]
    $folders = New-Object System.Collections.Generic.List[string]
    try {
        while ($queue.Count -gt 0 -or $running.Count -gt 0) {
            while ($running.Count -lt $jobs -and $queue.Count -gt 0) {
                $p = $queue.Dequeue()
                $running.Add(@{ Plan = $p; Run = (Start-LoopRun $script:browser $p.Page $p.Scale $p.Cases) })
            }
            Start-Sleep -Milliseconds 25
            for ($i = $running.Count - 1; $i -ge 0; $i--) {
                $e = $running[$i]
                if (-not (Test-LoopRunDone $e.Run)) { continue }
                $results[$e.Plan.Key] = Complete-LoopRun $e.Run -Defer
                $folders.Add($e.Run.Tmp)
                $running.RemoveAt($i)
            }
        }
    } finally {
        foreach ($e in $running) { try { $e.Run.Proc.Kill() } catch {} }
    }
    foreach ($d in $folders) { Remove-LoopTemp $d }
}
# The judgment of one run's text at one scale: the problems, and the case lines to print. $only, the cases the run
# asked for, judges those cases' floors and counts them alone, and the header must name them.
function Test-Loop([string]$text, [string]$scale, [string[]]$only = @()) {
    $problems = New-Object System.Collections.Generic.List[string]
    $table = $floors
    $want = $expectedCases
    $lines = @($text -split "`n" | ForEach-Object { $_.TrimEnd("`r") })
    $header = $lines[0]
    if ($header -notlike 'loop: host *') { $problems.Add("the first line is not the loop oracle's header: $header") }
    if ($only.Count -gt 0) {
        $table = @($floors | Where-Object { $only -contains $_.Case })
        # the oracle's count is the cases it ran, Alonzo's own said absent apart
        $want = $table.Count
        if ($header -notlike ('*; asked=' + ($only -join ',') + ';*')) { $problems.Add("the page's header names no cases asked as " + ($only -join ',') + ": $header") }
    }
    $dm = [regex]::Match($header, 'dpr=([0-9.]+)')
    if (-not $dm.Success -or $dm.Groups[1].Value -ne $scale) { $problems.Add("the page saw a ratio of '$($dm.Groups[1].Value)', the run asked for $scale") }
    $cases = @{}
    $caseLines = New-Object System.Collections.Generic.List[string]
    $notHere = @{}
    foreach ($l in $lines) {
        $nh = [regex]::Match($l, '^case (\S+): not on this page')
        if ($nh.Success) { $notHere[$nh.Groups[1].Value] = $true; continue }
        $m = [regex]::Match($l, '^case (\S+): ([^:]*): (OK|FAIL.*)$')
        if (-not $m.Success) { continue }
        $counts = @{}
        foreach ($pair in ($m.Groups[2].Value -split ',\s*')) { $kv = [regex]::Match($pair.Trim(), '^(\w+) (\S+)$'); if ($kv.Success) { $counts[$kv.Groups[1].Value] = $kv.Groups[2].Value } }
        $cases[$m.Groups[1].Value] = @{ Counts = $counts; Verdict = $m.Groups[3].Value }
        $caseLines.Add(("  {0}: {1}" -f $m.Groups[1].Value, ($l -replace '^case \S+: ', '')))
    }
    $module = 0
    if ($cases.ContainsKey('abi') -and $cases['abi'].Counts.ContainsKey('module')) { $module = [double]$cases['abi'].Counts['module'] }
    if ($module -gt 0 -and (Test-Path -LiteralPath $artifact)) {
        $size = (Get-Item -LiteralPath $artifact).Length
        if ($module -ne $size) { $problems.Add("the page holds an engine module of $module bytes and $artifact is $size; a page built beside the artifact holds it, so build the page again (tools/build_web.ps1)") }
    }
    # Alonzo's own cases, each that the run asked for, say they are not on this page, and none of them runs.
    $askedAbsent = if ($only.Count -gt 0) { @($absent | Where-Object { $only -contains $_ }) } else { $absent }
    foreach ($name in $askedAbsent) {
        if (-not $notHere.ContainsKey($name)) { $problems.Add("the case $name, Alonzo's own, did not say it is not on this page") }
        if ($cases.ContainsKey($name)) { $problems.Add("the case $name, Alonzo's own, ran on Buffet's page") }
    }
    foreach ($name in @($notHere.Keys | Sort-Object)) { if ($absent -notcontains $name) { $problems.Add("the case $name said it is not on this page, where it runs") } }
    foreach ($f in $table) {
        $c = $cases[$f.Case]
        if ($null -eq $c) { $problems.Add("the case $($f.Case) was not run"); continue }
        if ($c.Verdict -ne 'OK') { $problems.Add("the case $($f.Case) at a ratio of $scale`: $($c.Verdict)") }
        $mins = @{}
        foreach ($k in $f.Min.Keys) { $mins[$k] = $f.Min[$k] }
        if ($f.ContainsKey('WithModule') -and $module -gt 0) { foreach ($k in $f.WithModule.Keys) { $mins[$k] = $f.WithModule[$k] } }
        foreach ($k in $mins.Keys) {
            if (-not $c.Counts.ContainsKey($k)) { $problems.Add("the case $($f.Case) printed no count of $k"); continue }
            if ([double]$c.Counts[$k] -lt $mins[$k]) { $problems.Add("the case $($f.Case) at a ratio of $scale counted $($c.Counts[$k]) $k, below its floor of $($mins[$k])") }
        }
        foreach ($k in $zero) { if ($c.Counts.ContainsKey($k) -and [double]$c.Counts[$k] -ne 0) { $problems.Add("the case $($f.Case) at a ratio of $scale counted $($c.Counts[$k]) $k, where it must count none") } }
    }
    $last = $lines[$lines.Count - 1]
    if ($last -notmatch '^loop ok cases=(\d+)') { $problems.Add("the last line is not 'loop ok': $last") }
    elseif ([int]$Matches[1] -ne $want) { $problems.Add("the oracle ran $($Matches[1]) case(s), not the $want pinned here") }
    return @{ Problems = $problems; Lines = $caseLines; Header = $header; Cases = $cases }
}

$script:browser = Find-Browser $Browser
if (-not (Test-Path -LiteralPath $Page)) { Write-Output ("  SKIPPED: no built page at {0} (tools/build_web.ps1 writes it)" -f $Page); exit 0 }
if ($script:browser -eq '') { Write-Output '  SKIPPED: no browser found (Google Chrome, Microsoft Edge or Chromium at their known paths or on the PATH; set BUFFET_BROWSER or ALONZO_BROWSER, or -Browser, to name one)'; exit 0 }
if ($floors.Count -ne $expectedCases) { Write-Output ("FAIL: the floors table holds {0} case(s), not the {1} pinned here" -f $floors.Count, $expectedCases); exit 1 }
$built = [System.IO.File]::ReadAllText($Page)
$hasModule = -not ([regex]::IsMatch($built, '<script type="text/plain" id="alonzo-wasm"></script>'))

if ($Control) {
    if (-not $hasModule) { Write-Output 'SKIPPED: the control''s mutants play the session, and the page holds no engine module (built with -NoWasm): they do not apply'; exit 0 }
    if ($session.Held -eq 0 -or $session.Last -eq 0) { Write-Output 'FAIL: the control needs the session: no src/buffet.replay.vla or src/buffet.hand.js to read'; exit 1 }
    $tmp = Join-Path ([System.IO.Path]::GetTempPath()) ('buffet_loop_control_' + [System.IO.Path]::GetRandomFileName())
    New-Item -ItemType Directory -Path $tmp | Out-Null
    try {
        $utf8 = New-Object System.Text.UTF8Encoding($false)
        # A block's text in the built page, and the page with it replaced: a text block or the hand's script block.
        function Get-BlockMatch([string]$id) {
            if ($id -like 'hand-*') { return [regex]::Match($built, '(<script id="' + [regex]::Escape($id) + '"[^>]*>)([\s\S]*?)(</script>)') }
            return [regex]::Match($built, '(<script type="text/plain" id="' + [regex]::Escape($id) + '"[^>]*>)([\s\S]*?)(</script>)')
        }
        function New-MutantPage([string]$name, [string]$html) {
            $out = Join-Path $tmp ($name + '.html')
            [System.IO.File]::WriteAllText($out, $html, $utf8)
            return $out
        }
        $hand = Get-BlockMatch 'hand-buffet'
        $golden = Get-BlockMatch 'golden-buffet'
        $replay = Get-BlockMatch 'replay-buffet'
        if (-not ($hand.Success -and $golden.Success -and $replay.Success)) { Write-Output 'FAIL: the page holds no hand-buffet, golden-buffet or replay-buffet block to mutate: build it again (tools/build_web.ps1)'; exit 1 }
        $handText = $hand.Groups[2].Value
        function Set-BlockText($m, [string]$text) { return $built.Substring(0, $m.Groups[2].Index) + $text + $built.Substring($m.Groups[2].Index + $m.Groups[2].Length) }
        # 1. the hand a frame late: every frame it holds one later, failing where a cell moved at that frame
        $late = [regex]::Replace($handText, '(?m)^(    \[)(\d+)', { param($m) $m.Groups[1].Value + ([int]$m.Groups[2].Value + 1) })
        # 2. the hand's first fill a cent off: the firm's cash, CELLS' Cash!B3, at the first frame it moved after the begin
        $names = @([regex]::Matches([regex]::Match($handText, '(?s)var CELLS = \[(.*?)\];').Groups[1].Value, '"([^"]*)"') | ForEach-Object { $_.Groups[1].Value })
        $cashAt = [Array]::IndexOf($names, 'Cash!B3')
        $begunAt = [Array]::IndexOf($names, 'Proctor!B41')
        $fillFrame = 0
        $centOff = $handText
        $begun = $false
        foreach ($r in [regex]::Matches($handText, '(?m)^    \[(\d+)(.*)\],?$')) {
            $pairs = @($r.Groups[2].Value -split ', ' | Where-Object { $_ -ne '' })
            for ($i = 0; $i + 1 -lt $pairs.Count; $i += 2) {
                if ([int]$pairs[$i] -eq $begunAt -and $pairs[$i + 1] -eq '1') { $begun = $true }
            }
            if (-not $begun -or $fillFrame -gt 0) { continue }
            for ($i = 0; $i + 1 -lt $pairs.Count; $i += 2) {
                if ([int]$pairs[$i] -eq $cashAt -and [regex]::IsMatch($r.Value, '^    \[\d+')) {
                    $line = $r.Value
                    $was = $pairs[$i + 1]
                    if ($was -match '^\d+$' -and $r.Groups[1].Value -ne '') {
                        $fillFrame = [int]$r.Groups[1].Value
                        $mutated = $line.Replace((', ' + $cashAt + ', ' + $was), (', ' + $cashAt + ', ' + ([long]$was - 1)))
                        $centOff = $handText.Replace($line, $mutated)
                    }
                }
            }
        }
        # 4. a cell of the golden changed: the last value row's number one more
        $goldenText = $golden.Groups[2].Value
        $gv = [regex]::Matches($goldenText, '(?m)^\(value "Report" "([A-Z]+\d+)" (-?\d+)\)\r?$')
        $goldenOff = $goldenText
        if ($gv.Count) { $g = $gv[$gv.Count - 1]; $goldenOff = $goldenText.Substring(0, $g.Groups[2].Index) + ([long]$g.Groups[2].Value + 1) + $goldenText.Substring($g.Groups[2].Index + $g.Groups[2].Length) }
        # 5. a frame's only row cut from the replay: the last frame with one row, its row gone
        $replayBody = $replay.Groups[2].Value
        $one = [regex]::Matches($replayBody, '(?m)^\(frame (\d+)\n  (\(cell [^\n]*\))\)$')
        $cutFrame = 0
        $replayCut = $replayBody
        if ($one.Count) { $c = $one[$one.Count - 1]; $cutFrame = [int]$c.Groups[1].Value; $replayCut = $replayBody.Substring(0, $c.Index) + ('(frame ' + $cutFrame + ')') + $replayBody.Substring($c.Index + $c.Length) }
        $mutants = @(
            @{ Name = 'the hand a frame late'; Applied = ($late -ne $handText); Page = (New-MutantPage 'late' (Set-BlockText $hand $late)); Cases = 'abi,replay:buffet'
               Want = "buffet's committed replay, played on a fresh host, is not its hand's at frame" },
            @{ Name = 'the hand''s first fill a cent off'; Applied = ($fillFrame -gt 0 -and $centOff -ne $handText); Page = (New-MutantPage 'cent' (Set-BlockText $hand $centOff)); Cases = ('abi,replay:buffet:' + ($fillFrame + 1))
               Want = ("buffet's committed replay, played on a fresh host, is not its hand's at frame {0}: Cash!B3" -f $fillFrame) },
            @{ Name = 'the hand''s block dropped'; Applied = $true; Page = (New-MutantPage 'nohand' ($built.Substring(0, $hand.Index) + $built.Substring($hand.Index + $hand.Length))); Cases = 'abi,replay:buffet:64'
               Want = 'counted 1 alone, where it must count none' },
            @{ Name = 'a cell of the golden changed'; Applied = ($goldenOff -ne $goldenText); Page = (New-MutantPage 'golden' (Set-BlockText $golden $goldenOff)); Cases = 'abi,replay:buffet'
               Want = 'that is not its golden' },
            @{ Name = 'a frame''s only row cut from the replay'; Applied = ($cutFrame -gt 0 -and $replayCut -ne $replayBody); Page = (New-MutantPage 'cut' (Set-BlockText $replay $replayCut)); Cases = 'abi,replay:buffet'
               Want = 'is not the replay it played' }
        )
        $plan = New-Object System.Collections.Generic.List[object]
        foreach ($mu in $mutants) { if ($mu.Applied) { $plan.Add(@{ Key = $mu.Name; Page = $mu.Page; Scale = '1'; Cases = $mu.Cases }) } }
        $answers = @{}
        Invoke-LoopPool $plan.ToArray() $Jobs $answers
        $ok = $true
        foreach ($mu in $mutants) {
            if (-not $mu.Applied) { $ok = $false; Write-Output ("control: {0}: the mutation did not apply" -f $mu.Name); continue }
            $r = $answers[$mu.Name]
            if ($r.Text -eq '') { $ok = $false; Write-Output ("control: {0}: {1}" -f $mu.Name, $r.Reason); continue }
            $j = Test-Loop $r.Text '1' @(Get-CaseNames $mu.Cases)
            $hit = @($j.Problems | Where-Object { $_ -like ('*' + $mu.Want + '*') })
            if ($hit.Count -ge 1) { Write-Output ("control: {0} ({1:N1} s): fails, as it should: {2}" -f $mu.Name, $r.Seconds, $hit[0]) }
            else { $ok = $false; Write-Output ("control: {0}: did not fail by '{1}': {2}" -f $mu.Name, $mu.Want, (@($j.Problems) -join ' | ')) }
        }
        if ($ok) { Write-Output ("OK: control: each of the {0} mutant pages fails for its own reason" -f $mutants.Count); exit 0 }
        Write-Output 'FAIL: control: the check did not behave as its header says'
        exit 1
    } finally {
        Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
    }
}

Write-Output ("  the browser: {0}" -f $script:browser)
Write-Output ("  the page: {0}{1}" -f $Page, $(if ($hasModule) { '' } else { ', built without the engine''s module: the double''s cases alone' }))
$names = @($floors | ForEach-Object { $_.Case })
# The keep case, withheld from HOUSE.4's second slice to the pin to alonzo 0.5.0 (2026-11-21 to 2026-11-24): two of its
# 2,079 restores of the session, at frames 4,544 and 36,032, were not the run there, the page's host keeping a
# keyframe at a frame's end while an order's derived rows waited and a restore losing them. Alonzo's ENGINE.30's
# decision 13, built at Buffet's ask and released in 0.5.0: a save loaded past frame 0 makes at its next frame the
# derived writes its run made, so the case runs again, a run of its own.
$runs = @(
    @{ Key = 'replay'; Label = 'ratio 1, the replay case'; Scale = $scales[0]; Cases = @('abi', 'replay') },
    @{ Key = 'save'; Label = 'ratio 1, the save case'; Scale = $scales[0]; Cases = @('abi', 'save') },
    @{ Key = 'keep'; Label = 'ratio 1, the keep case'; Scale = $scales[0]; Cases = @('abi', 'keep') },
    @{ Key = 'rest'; Label = 'ratio 1, the other cases, and Alonzo''s own fifteen said absent'; Scale = $scales[0]; Cases = @(@($names | Where-Object { @('replay', 'save', 'keep') -notcontains $_ }) + $absent) },
    @{ Key = 'second'; Label = 'ratio 1.5, every case but replay, save and keep'; Scale = $scales[1]; Cases = @($names | Where-Object { @('replay', 'save', 'keep') -notcontains $_ }) }
)
$failed = New-Object System.Collections.Generic.List[string]
$plan = New-Object System.Collections.Generic.List[object]
foreach ($run in $runs) { $plan.Add(@{ Key = $run.Key; Page = $Page; Scale = $run.Scale; Cases = ($run.Cases -join ',') }) }
$answers = @{}
Invoke-LoopPool $plan.ToArray() $Jobs $answers
foreach ($run in $runs) {
    $r = $answers[$run.Key]
    if ($r.Text -eq '') { $failed.Add("$($run.Label): $($r.Reason)"); Write-Output ("  {0}: {1}  FAIL" -f $run.Label, $r.Reason); continue }
    $j = Test-Loop $r.Text $run.Scale $run.Cases
    Write-Output ("  {0} ({1:N1} s): {2}" -f $run.Label, $r.Seconds, $j.Header)
    foreach ($l in $j.Lines) { Write-Output ('  ' + $l) }
    foreach ($p in $j.Problems) { $failed.Add("$($run.Label): $p"); Write-Output ("    {0}  FAIL" -f $p) }
}
Write-Output ''
if ($failed.Count -eq 0) {
    $held = if ($hasModule) { "the session's {0:N0} frames held to its golden, its round and its hand at {1:N0} frames, {2:N0} cells" -f $session.Last, $session.Held, ($session.Held * $session.Cells) } else { 'the page without the engine''s module, so the session played nowhere: the double''s cases alone' }
    Write-Output ("=== CHECK: clean - every case OK on Buffet's page, the replay, save and keep cases at the first ratio and every other at both, every count at or above its floor; {0} ===" -f $held)
    exit 0
}
Write-Output "=== CHECK: $($failed.Count) problem(s) ==="
$failed | ForEach-Object { Write-Output "  $_" }
exit 1
