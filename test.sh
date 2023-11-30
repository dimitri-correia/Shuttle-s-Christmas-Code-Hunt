#!/bin/bash

URL="http://localhost:8000"
DAY=0

# Check for options
while getopts ":r:d:" opt; do
  case $opt in
    r)
      URL="https://cch23-dcorreia.shuttleapp.rs"
      ;;
    d)
      DAY=$OPTARG
      ;;
    \?)
      echo "Invalid option: -$OPTARG. Use '-r' for remote."
      exit 1
      ;;
  esac
done

echo -e "Test endpoints for day $DAY\n"

# Run commands based on the specified day
if [ "$DAY" -eq 0 ]; then
  echo -e "### 1st endpoint"
  curl -i -X GET "$URL/"

  echo -e "\n\n### 2nd endpoint"
  curl -i -X GET "$URL/-1/error"
elif [ "$DAY" -eq 1 ]; then
  echo -e "### 1st endpoint"
  curl -i -X GET "$URL/okok"

  echo -e "\n\n### 2nd endpoint"
  curl -i -X GET "$URL/ko"
fi
