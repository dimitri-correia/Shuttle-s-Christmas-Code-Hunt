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
  curl -i -X GET "$URL/1/4/8"

  result=$(curl -i -X GET "$URL/1/4/8" 2>/dev/null)
  if [[ "$result" != *"1728"* ]]; then
    echo -e "[ERROR] ########## The result is supposed to be 1728 ########## "
  fi

  echo -e "\n\n### 2nd endpoint"
  curl -X GET "$URL/1/4/8/10"
  result=$(curl -i -X GET "$URL/1/4/8/10" 2>/dev/null)
  if [[ "$result" != *"216"* ]]; then
    echo -e "[ERROR] ########## The result is supposed to be 216 ########## "
  fi
fi



echo -e "\nFin test endpoints for day $DAY\n"