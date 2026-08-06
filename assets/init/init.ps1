# The functions for traversing the sibling directories.
# Install them by putting the following line into your $PROFILE.
#
#     sibling --init powershell | Out-String | Invoke-Expression
#
# Each function receives the optional count of the traversing, such as
# "cdnext 3". A negative count traverses in the opposite direction.
# The count is ignored by cdfirst, cdlast, cdrand, lsfirst, lslast, and lsrand.

# Print the found sibling directory of the working directory.
# $LASTEXITCODE is the exit status of the sibling command; 0 means the directory
# was found, 1 means no more sibling directory, and the others mean an error.
function Find-SiblingDirectory {
    param([string]$Type, [int]$Count = 1)

    & sibling --type $Type --step $Count -- $($PWD.Path)
}

# Print the working directory with its position, such as "C:\path\to\c (3\26)".
function Show-SiblingPosition {
    & sibling --progress --type keep -- $($PWD.Path)
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
    param([string]$Type, [int]$Count = 1)

    $next = Find-SiblingDirectory $Type $Count
    $code = $LASTEXITCODE
    if ($code -ne 0 -or [string]::IsNullOrEmpty($next)) {
        Write-SiblingReport $code
        return
    }
    Set-Location -LiteralPath $next
    Show-SiblingPosition
}

# List the entries of the found sibling directory, without changing the
# working directory.
function Get-SiblingChildItem {
    param([string]$Type, [int]$Count = 1)

    $next = Find-SiblingDirectory $Type $Count
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
    param([string]$Filter)

    $selected = & sibling --format list --type keep -- $($PWD.Path) | & $Filter
    if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrEmpty($selected)) {
        return
    }
    # Each line of the list format consists of the index, the marker of the
    # current and the next directories, and the path; drop all but the path.
    $selected = $selected -replace '^ *[0-9]+ (\* |> |  )', ''
    Set-Location -LiteralPath $selected
    Show-SiblingPosition
}

function cdnext { param([int]$Count = 1) Set-SiblingLocation next $Count }

function cdprev { param([int]$Count = 1) Set-SiblingLocation previous $Count }

function cdfirst { param([int]$Count = 1) Set-SiblingLocation first $Count }

function cdlast { param([int]$Count = 1) Set-SiblingLocation last $Count }

function cdrand { param([int]$Count = 1) Set-SiblingLocation random $Count }

function lsnext { param([int]$Count = 1) Get-SiblingChildItem next $Count }

function lsprev { param([int]$Count = 1) Get-SiblingChildItem previous $Count }

function lsfirst { param([int]$Count = 1) Get-SiblingChildItem first $Count }

function lslast { param([int]$Count = 1) Get-SiblingChildItem last $Count }

function lsrand { param([int]$Count = 1) Get-SiblingChildItem random $Count }

function sibling_peco { Set-SiblingLocationWithFilter peco }

function sibling_fzf { Set-SiblingLocationWithFilter fzf }
