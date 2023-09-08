# Open a file in the daily notes folder

# Get the current date
DATE=$(date +%Y-%m-%d)

base_dir="$HOME/docs/notes/daily"

mkdir -p "$base_dir"

nvim "$base_dir/$DATE.md"
