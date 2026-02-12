LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | fold -w 8 | head -n 100 | xargs mkdir
\ls | shuf | head -5 > target.txt
