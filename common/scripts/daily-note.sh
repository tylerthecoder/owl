# Open a file in the daily notes folder

# Get the current date
DATE=$(date +%Y-%m-%d)

template_dir="$HOME/docs/mind/Templates"
base_dir="$HOME/docs/mind/Journal 📔"
mkdir -p "$base_dir"

daily_note="$base_dir/$DATE.md"
template_note="$template_dir/Daily Note.md"

# If the Daily Note template exists and there isn't already a file there, copy it over
if [ -f "$template_note" ] && [ ! -f "$daily_note" ]; then
    cp "$template_note" "$daily_note"
fi

nvim "$daily_note"
