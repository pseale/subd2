#!/usr/bin/env bash

set -e
set -o pipefail

# go to repo's root directory - the one containing the .env file
cd "$(dirname $0)/.."

if [[ ! $1 == "--username" ]]; then
    echo "ERROR: invalid arguments provided" 1>&2
    echo "Usage: $0 --username <yourusername>"
    exit 1
fi

if [[ -z $2 ]]; then
    echo "ERROR: invalid arguments provided" 1>&2
    echo "Usage: $0 --username <yourusername>"
    exit 1
fi

twitchusername=$2
twitchusernamelowercase=$(echo "$twitchusername" | awk '{print tolower($0)}')

scopes="chat:edit channel:read:subscriptions channel:read:redemptions"
scopes_urlencoded="$(echo $scopes | tr -d '\n' | jq -sRr @uri)" # https://stackoverflow.com/a/34407620 and https://stackoverflow.com/a/12524345

twitchclioutput=$(twitch token -u --scopes "$scopes" 2>&1)
useraccesstoken=$(echo "$twitchclioutput" | grep "User Access Token" | cut -d " " -f 6)
refreshtoken=$(echo "$twitchclioutput" | grep "Refresh Token" | cut -d " " -f 3)

twitchchannelid=$(twitch api get users -q "login=$twitchusername" | jq '.data[0].id' -r)
if [[ "$twitchchannelid" == "null" ]]; then
    echo "ERROR: no twitch user found with login '$twitchusername'" 1>&2
    exit 1
fi

sed -i .env  -e  "s/^.*export SUBD_TWITCH_BOT_USERNAME=.*$/export SUBD_TWITCH_BOT_USERNAME=$twitchusernamelowercase/"
sed -i .env  -e  "s/^.*export SUBD_TWITCH_BOT_CHANNEL_ID=.*$/export SUBD_TWITCH_BOT_CHANNEL_ID=$twitchchannelid/"
sed -i .env  -e  "s/^.*export SUBD_TWITCH_BOT_OAUTH=.*$/export SUBD_TWITCH_BOT_OAUTH=$useraccesstoken/"
sed -i .env  -e  "s/^.*export SUBD_TWITCH_BOT_REFRESH=.*$/export SUBD_TWITCH_BOT_REFRESH=$refreshtoken/"