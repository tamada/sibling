# shellcheck shell=bash
# The test cases for bash and zsh; TEST_SHELL tells which one is running.
#
# Every shell must print the identical stdout and stderr; see expected.out and
# expected.err. Therefore, keep the cases of the shells in the same order, and
# do not print anything which depends on the shell.

eval "$(sibling --init "${TEST_SHELL}")"

cd /work/testdata/basic/c || exit 1

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
cd "/work/testdata/worried/dir with spaces" || exit 1
cdnext
cdprev
echo "== filter =="
cd /work/testdata/basic/c || exit 1
sibling_peco
echo "== filter without selection =="
sibling_fzf
echo "pwd=$PWD"
echo "== no parent =="
cd / || exit 1
cdnext
echo "pwd=$PWD"
