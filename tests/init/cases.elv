# The test cases for Elvish.
#
# Every shell must print the identical stdout and stderr; see expected.out and
# expected.err. Therefore, keep the cases of the shells in the same order, and
# do not print anything which depends on the shell.
#
# Elvish loads the script as a module; run.sh puts it into the lib directory
# before running this file.

use sibling

var cdnext~ = $sibling:cdnext~
var cdprev~ = $sibling:cdprev~
var cdfirst~ = $sibling:cdfirst~
var cdlast~ = $sibling:cdlast~
var cdrand~ = $sibling:cdrand~
var lsfirst~ = $sibling:lsfirst~
var sibling_peco~ = $sibling:sibling_peco~
var sibling_fzf~ = $sibling:sibling_fzf~

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
echo "pwd="$pwd
echo "== cdlast =="
cdlast
echo "== cdnext at the last =="
cdnext
echo "pwd="$pwd
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
echo "pwd="$pwd
echo "== no parent =="
cd /
cdnext
echo "pwd="$pwd
