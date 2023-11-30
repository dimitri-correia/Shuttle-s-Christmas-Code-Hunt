#!/bin/bash

# Set default URL
URL="http://localhost:8000/"

# Check for options
while getopts ":r" opt; do
  case $opt in
    r)
      URL="https://cch23-dcorreia.shuttleapp.rs/"
      ;;
    \?)
      echo "Invalid option: -$OPTARG. Use '-r' for remote."
      exit 1
      ;;
  esac
done


###  day -1 test

echo -e "### 1st endpoint"
curl -i -X GET "$URL"

echo -e "\n\n### 2nd endpoint"
curl -i -X GET "$URL-1/error"