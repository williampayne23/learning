#!/bin/bash
CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"


language=$(ls $CURRENT_DIR/templates | gum filter --header "Select Language" --height=10)
year=$(gum input --value $(date +%Y) --header "Select year:")
day=$(gum input --value $(date +%d) --header "Select day:")
nicelyFormattedDay=$(printf "d%02d" $day)

cd $CURRENT_DIR
mkdir -p "$year/$language/$nicelyFormattedDay"
cp -r templates/$language/* "$year/$language/$nicelyFormattedDay"
find "$year/$language/$nicelyFormattedDay" -type f \( -name "*.toml" -o -name "*.rs" \) -exec sed -i '' s/DAYPLACEHOLDER/${nicelyFormattedDay}/g {} +
