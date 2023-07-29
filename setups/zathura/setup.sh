sudo pacman -S zathura zathura-pdf-mupdf

owl add "$OWL_PATH/setups/zathura/zathurarc" "~/.config/zathura/zathurarc"

# Make it the default PDF viewer
xdg-mime default org.pwmt.zathura.desktop application/pdf

owl link