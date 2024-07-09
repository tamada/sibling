#! /bin/sh

VERSION=$1

if [[ "$VERSION" == "" ]]; then
    echo "no version specified"
    exit 1
fi

result=0

PREV_VERSION=$(grep "^version = " Cargo.toml | sed -e 's/version = "\(.*\)"/\1/g')
if [[ "$PREV_VERSION" != "" && $VERSION == $PREV_VERSION ]]; then
    echo "already updated to $VERSION"
    exit 1
fi

for i in README.md ; do
    sed -e "s#Version-${PREV_VERSION}-information#Version-${VERSION//-/--}-information#g" -e "s#tag/v${PREV_VERSION}#tag/v${VERSION}#g" $i > a
    mv a $i
done

sed "s/^version= /version = "${VERSION}"/g" Cargo.toml > a && mv a Cargo.toml

echo "Replace version from \"${PREV_VERSION}\" to \"${VERSION}\""
