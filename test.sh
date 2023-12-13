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

  echo -e "\n\n### 3rd endpoint"
  curl -X GET "$URL/1/10"
  result=$(curl -i -X GET "$URL/1/10" 2>/dev/null)
  if [[ "$result" != *"1000"* ]]; then
    echo -e "[ERROR] ########## The result is supposed to be 1000 ########## "
  fi

  echo -e "\n\n### 4th endpoint"
  curl -X GET "$URL/1/4/5/8/10"
  result=$(curl -i -X GET "$URL/1/4/5/8/10" 2>/dev/null)
  if [[ "$result" != *"27"* ]]; then
    echo -e "[ERROR] ########## The result is supposed to be 27 ########## "
  fi

elif [ "$DAY" -eq 4 ]; then
  echo -e "### 1st endpoint"
  curl -X POST  "$URL/4/strength"   -H 'Content-Type: application/json'   -d '[
                                { "name": "Dasher", "strength": 5 },
                                { "name": "Dancer", "strength": 6 },
                                { "name": "Prancer", "strength": 4 },
                                { "name": "Vixen", "strength": 7 }
                              ]'


  result=$(curl -X POST  "$URL/4/strength"   -H 'Content-Type: application/json'   -d '[
                                           { "name": "Dasher", "strength": 5 },
                                           { "name": "Dancer", "strength": 6 },
                                           { "name": "Prancer", "strength": 4 },
                                           { "name": "Vixen", "strength": 7 }
                                         ]' 2>/dev/null)
  if [[ "$result" != *"22"* ]]; then
    echo -e "[ERROR] ########## The result is supposed to be 22 ########## "
  fi

  echo -e "\n\n### 2nd endpoint"
  curl -X POST \
    "$URL/4/contest" \
    -H 'Content-Type: application/json' \
    -d '[
      {
        "name": "Dasher",
        "strength": 5,
        "speed": 50.4,
        "height": 80,
        "antler_width": 36,
        "snow_magic_power": 9001,
        "favorite_food": "hay",
        "cAnD13s_3ATeN-yesT3rdAy": 2
      },
      {
        "name": "Dancer",
        "strength": 6,
        "speed": 48.2,
        "height": 65,
        "antler_width": 37,
        "snow_magic_power": 4004,
        "favorite_food": "grass",
        "cAnD13s_3ATeN-yesT3rdAy": 5
      }
    ]'
  result=$(curl -X POST \
               "$URL/4/contest" \
               -H 'Content-Type: application/json' \
               -d '[
                 {
                   "name": "Dasher",
                   "strength": 5,
                   "speed": 50.4,
                   "height": 80,
                   "antler_width": 36,
                   "snow_magic_power": 9001,
                   "favorite_food": "hay",
                   "cAnD13s_3ATeN-yesT3rdAy": 2
                 },
                 {
                   "name": "Dancer",
                   "strength": 6,
                   "speed": 48.2,
                   "height": 65,
                   "antler_width": 37,
                   "snow_magic_power": 4004,
                   "favorite_food": "grass",
                   "cAnD13s_3ATeN-yesT3rdAy": 5
                 }
               ]' 2>/dev/null)
  if [[ "$result" != *"216"* ]]; then
    echo -e "[ERROR] ########## The result is supposed to be 216 ########## "
  fi

fi



echo -e "\nFin test endpoints for day $DAY\n"