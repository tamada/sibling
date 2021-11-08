#! /bin/sh

VERSION=$1

if [[ "$VERSION" == "" ]]; then
    echo "no version specified"
    exit 1
fi

result=0

grep "$VERSION" Makefile 2>&1 > /dev/null || result=$?
if [[ $result -eq 0 ]]; then
    echo "already updated to $VERSION"
    exit 1
fi

prev=$(grep ^VERSION Makefile | tr -d 'VERSION := ')

for i in README.md docs/content/_index.md; do
    sed -e "s#Version-${prev}-green#Version-${VERSION//-/--}-green#g" -e "s#tag/v${prev}#tag/v${VERSION}#g" $i > a
    mv a $i
done

sed "s/VERSION := .*/VERSION := ${VERSION}/g" Makefile > a && mv a Makefile
sed "s/const VERSION = \".*\"/const VERSION = \"${VERSION}\"/g" cmd/sibling/main.go > a && mv a cmd/sibling/main.go

echo "Replace version to \"${VERSION}\""
