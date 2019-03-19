#! /bin/sh

if [ $$ != "`pgrep -fo $0`" ]
then
  echo "gyazo_slate is already running."
  exit 1;
fi

echo "gyazo_slate running."
cargo run
echo "gyazo_slate done."
