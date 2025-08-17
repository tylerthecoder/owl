-- disable netrw at the very start of your init.lua
vim.g.loaded_netrw = 1
vim.g.loaded_netrwPlugin = 1

-- set termguicolors to enable highlight groups
vim.opt.termguicolors = true

-- word wrap
vim.opt.linebreak = true

-- Bootstrap and setup lazy.nvim
require("config.lazy")

require("tylord")
vim.cmd.colorscheme "catppuccin"
