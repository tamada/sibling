# The functions for traversing the sibling directories.
# Install them by putting the following line into your $PROFILE.
#
#     sibling --init powershell | Out-String | Invoke-Expression
#
# Each function receives the optional count of the traversing, such as
# "cdnext 3". A negative count traverses in the opposite direction.
# The count is ignored by cdfirst, cdlast, cdrand, lsfirst, lslast, and lsrand.
#
# They also receive "-File FILE" ("-f" for short), which traverses the
# directories listed in the file, instead of the siblings of the working
# directory, such as "cdnext -f ~/projects.txt". Give the file in an absolute
# path, since the working directory changes.

# Return the target of the traversing; the given file, or the working directory.
function Get-SiblingTarget {
    param([string]$File)

    if ([string]::IsNullOrEmpty($File)) { $PWD.Path } else { $File }
}

# Print the found sibling directory of the working directory, or of the given
# list file. $LASTEXITCODE is the exit status of the sibling command; 0 means
# the directory was found, 1 means no more sibling directory, and the others
# mean an error.
function Find-SiblingDirectory {
    param([string]$Type, [int]$Count = 1, [string]$File)

    & sibling --type $Type --step $Count -- $(Get-SiblingTarget $File)
}

# Print the working directory with its position, such as "C:\path\to\c (3\26)".
function Show-SiblingPosition {
    param([string]$File)

    & sibling --progress --type keep -- $(Get-SiblingTarget $File)
}

# Tell the user why no directory was found.
function Write-SiblingReport {
    param([int]$Code)

    if ($Code -eq 1) {
        [Console]::Error.WriteLine('sibling: no more sibling directory')
    }
}

# Change the working directory to the found sibling directory.
function Set-SiblingLocation {
    param([string]$Type, [int]$Count = 1, [string]$File)

    $next = Find-SiblingDirectory $Type $Count $File
    $code = $LASTEXITCODE
    if ($code -ne 0 -or [string]::IsNullOrEmpty($next)) {
        Write-SiblingReport $code
        return
    }
    Set-Location -LiteralPath $next
    Show-SiblingPosition $File
}

# List the entries of the found sibling directory, without changing the
# working directory.
function Get-SiblingChildItem {
    param([string]$Type, [int]$Count = 1, [string]$File)

    $next = Find-SiblingDirectory $Type $Count $File
    $code = $LASTEXITCODE
    if ($code -ne 0 -or [string]::IsNullOrEmpty($next)) {
        Write-SiblingReport $code
        return
    }
    Write-Output $next
    Get-ChildItem -LiteralPath $next
}

# Choose a sibling directory with the filter command, such as peco and fzf,
# and change the working directory to it.
function Set-SiblingLocationWithFilter {
    param([string]$Filter, [string]$File)

    $selected = & sibling --format list --type keep -- $(Get-SiblingTarget $File) | & $Filter
    if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrEmpty($selected)) {
        return
    }
    # Each line of the list format consists of the index, the marker of the
    # current and the next directories, and the path; drop all but the path.
    $selected = $selected -replace '^ *[0-9]+ (\* |> |  )', ''
    Set-Location -LiteralPath $selected
    Show-SiblingPosition $File
}

# Return the found sibling directory, without changing the working directory.
# It tells the result by $LASTEXITCODE only, since the caller usually reads it
# by the sub expression, such as `Copy-Item file (nextdir)`.
function Get-SiblingDirectory {
    param([string]$Type, [int]$Count = 1, [string]$File)

    $next = Find-SiblingDirectory $Type $Count $File
    if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrEmpty($next)) {
        return
    }
    $next
}

# Set NEXTDIR and PREVDIR to the siblings of the working directory; they become
# empty when no such directory is found.
function Update-SiblingEnvironment {
    $next = Find-SiblingDirectory next 1 2> $null
    if ($LASTEXITCODE -ne 0) { $next = '' }
    $prev = Find-SiblingDirectory previous 1 2> $null
    if ($LASTEXITCODE -ne 0) { $prev = '' }
    $env:NEXTDIR = $next
    $env:PREVDIR = $prev
}

# Run the hook on every change of the working directory. It is not registered
# by default, since it runs the sibling command twice on every change; reading
# a directory of ten thousand entries costs about 30 milliseconds.
function sibling_hook_enable {
    $action = $ExecutionContext.SessionState.InvokeCommand.LocationChangedAction
    if (-not ($action -and $action.ToString() -match 'Update-SiblingEnvironment')) {
        # Keep the action of the other tools, such as zoxide, and call it.
        $global:SiblingPreviousLocationChangedAction = $action
        $ExecutionContext.SessionState.InvokeCommand.LocationChangedAction = {
            param($old, $new)
            if ($global:SiblingPreviousLocationChangedAction) {
                & $global:SiblingPreviousLocationChangedAction $old $new
            }
            Update-SiblingEnvironment
        }
    }
    Update-SiblingEnvironment
}

function sibling_hook_disable {
    $ExecutionContext.SessionState.InvokeCommand.LocationChangedAction = `
        $global:SiblingPreviousLocationChangedAction
    Remove-Item env:NEXTDIR, env:PREVDIR -ErrorAction SilentlyContinue
}

function nextdir { param([int]$Count = 1, [string]$File) Get-SiblingDirectory next $Count $File }

function prevdir { param([int]$Count = 1, [string]$File) Get-SiblingDirectory previous $Count $File }

function cdnext { param([int]$Count = 1, [string]$File) Set-SiblingLocation next $Count $File }

function cdprev { param([int]$Count = 1, [string]$File) Set-SiblingLocation previous $Count $File }

function cdfirst { param([int]$Count = 1, [string]$File) Set-SiblingLocation first $Count $File }

function cdlast { param([int]$Count = 1, [string]$File) Set-SiblingLocation last $Count $File }

function cdrand { param([int]$Count = 1, [string]$File) Set-SiblingLocation random $Count $File }

function lsnext { param([int]$Count = 1, [string]$File) Get-SiblingChildItem next $Count $File }

function lsprev { param([int]$Count = 1, [string]$File) Get-SiblingChildItem previous $Count $File }

function lsfirst { param([int]$Count = 1, [string]$File) Get-SiblingChildItem first $Count $File }

function lslast { param([int]$Count = 1, [string]$File) Get-SiblingChildItem last $Count $File }

function lsrand { param([int]$Count = 1, [string]$File) Get-SiblingChildItem random $Count $File }

function sibling_peco { param([string]$File) Set-SiblingLocationWithFilter peco $File }

function sibling_fzf { param([string]$File) Set-SiblingLocationWithFilter fzf $File }
