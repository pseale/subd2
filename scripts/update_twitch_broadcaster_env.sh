#!/usr/bin/env bash

set -e
set -o pipefail

# go to repo's root directory - the one containing the .env file
cd "$(dirname "$0")/.."

if [[ ! $1 == "--username" ]] || [[ -z $2 ]]; then
    echo "ERROR: invalid arguments provided" 1>&2
    echo "Usage: $0 --username <yourusername>"
    exit 1
fi

cp --no-clobber .env.template .env

twitchusername=$2
twitchusernamelowercase=$(echo "$twitchusername" | awk '{print tolower($0)}')

scopes="chat:edit channel:read:subscriptions channel:read:redemptions"

twitchclioutput=$(twitch token -u --scopes "$scopes" 2>&1)
useraccesstoken=$(echo "$twitchclioutput" | grep "User Access Token" | cut -d " " -f 6)

sed -i .env  -e  "s/^.*SUBD2_BROADCASTER_USERNAME=.*$/SUBD2_BROADCASTER_USERNAME=$twitchusernamelowercase/"
sed -i .env  -e  "s/^.*SUBD2_BROADCASTER_OAUTH_TOKEN=.*$/SUBD2_BROADCASTER_OAUTH_TOKEN=$useraccesstoken/"
