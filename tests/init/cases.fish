# The test cases for fish.
#
# Every shell must print the identical stdout and stderr; see expected.out and
# expected.err. Therefore, keep the cases of the shells in the same order, and
# do not print anything which depends on the shell.

sibling --init fish | source

cd /work/testdata/basic/c

echo "== cdnext =="
cdnext
echo "== cdnext 3 =="
cdnext 3
echo "== cdprev 2 =="
cdprev 2
echo "== cdprev -1 =="
cdprev -1
echo "== cdfirst =="
cdfirst
echo "== cdprev at the first =="
cdprev
echo "pwd=$PWD"
echo "== cdlast =="
cdlast
echo "== cdnext at the last =="
cdnext
echo "pwd=$PWD"
echo "== lsfirst =="
lsfirst
echo "== cdrand then cdfirst =="
cdrand > /dev/null
cdfirst
echo "== spaces and multibyte =="
cd "/work/testdata/worried/dir with spaces"
cdnext
cdprev
echo "== filter =="
cd /work/testdata/basic/c
sibling_peco
echo "== filter without selection =="
sibling_fzf
echo "pwd=$PWD"
echo "== no parent =="
cd /
cdnext
echo "pwd=$PWD"
echo "== cdnext -f list =="
cdnext -f /work/testdata/basic/dirlist.txt
echo "== cdnext -f list again =="
cdnext -f /work/testdata/basic/dirlist.txt
echo "== lsnext -f list =="
lsnext -f /work/testdata/basic/dirlist.txt
echo "== cdnext -f list at the last =="
cdnext 5 -f /work/testdata/basic/dirlist.txt
echo "pwd=$PWD"
