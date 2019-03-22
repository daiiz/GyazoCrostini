#! /bin/sh

if [ $$ != "`pgrep -fo $0`" ]
then
  echo "gyazo_slate is already running."
  exit 1;
fi

filepath="$(realpath ./target/release/gyazo_slate)"

echo "gyazo_slate running."
$filepath
echo "gyazo_slate done."
