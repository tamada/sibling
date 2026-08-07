# The test cases for PowerShell.
#
# Every shell must print the identical stdout and stderr; see expected.out and
# expected.err. Therefore, keep the cases of the shells in the same order, and
# do not print anything which depends on the shell.

sibling --init powershell | Out-String | Invoke-Expression

Set-Location /work/testdata/basic/c

Write-Output "== cdnext =="
cdnext
Write-Output "== cdnext 3 =="
cdnext 3
Write-Output "== cdprev 2 =="
cdprev 2
Write-Output "== cdprev -1 =="
cdprev -1
Write-Output "== cdfirst =="
cdfirst
Write-Output "== cdprev at the first =="
cdprev
Write-Output "pwd=$($PWD.Path)"
Write-Output "== cdlast =="
cdlast
Write-Output "== cdnext at the last =="
cdnext
Write-Output "pwd=$($PWD.Path)"
Write-Output "== lsfirst =="
lsfirst
Write-Output "== cdrand then cdfirst =="
cdrand | Out-Null
cdfirst
Write-Output "== spaces and multibyte =="
Set-Location "/work/testdata/worried/dir with spaces"
cdnext
cdprev
Write-Output "== filter =="
Set-Location /work/testdata/basic/c
sibling_peco
Write-Output "== filter without selection =="
sibling_fzf
Write-Output "pwd=$($PWD.Path)"
Write-Output "== no parent =="
Set-Location /
cdnext
Write-Output "pwd=$($PWD.Path)"
Write-Output "== cdnext -f list =="
cdnext -f /work/testdata/basic/dirlist.txt
Write-Output "== cdnext -f list again =="
cdnext -f /work/testdata/basic/dirlist.txt
Write-Output "== lsnext -f list =="
lsnext -f /work/testdata/basic/dirlist.txt
Write-Output "== cdnext -f list at the last =="
cdnext 5 -f /work/testdata/basic/dirlist.txt
Write-Output "pwd=$($PWD.Path)"
Write-Output "== nextdir / prevdir =="
Set-Location /work/testdata/basic/c
nextdir
prevdir
Write-Output "== hook =="
sibling_hook_enable
Set-Location /work/testdata/basic/e
Write-Output "NEXTDIR=$env:NEXTDIR PREVDIR=$env:PREVDIR"
sibling_hook_disable
