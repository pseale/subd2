#!/usr/bin/env bash

set -e
set -o pipefail

# go to repo's root directory - the one containing the .env file
cd "$(dirname "$0")/.."

cp --no-clobber .env.template .env

twitch_cli_configfile_path_with_trailing_paren=$( twitch --help | grep "config file (default is" 2>&1 | cut -d " " -f 15)
twitch_cli_configfile_path=${twitch_cli_configfile_path_with_trailing_paren::-1}

client_id=$(cat "$twitch_cli_configfile_path" | grep CLIENTID 2>&1 | cut -d "=" -f 2)
sed -i .env  -e  "s/^.*SUBD2_TWITCH_APPLICATION_CLIENT_ID=.*$/SUBD2_TWITCH_APPLICATION_CLIENT_ID=$client_id/"

client_secret=$(cat "$twitch_cli_configfile_path" | grep CLIENTSECRET 2>&1 | cut -d "=" -f 2)
sed -i .env  -e  "s/^.*SUBD2_TWITCH_APPLICATION_CLIENT_SECRET=.*$/SUBD2_TWITCH_APPLICATION_CLIENT_SECRET=$client_secret/"
