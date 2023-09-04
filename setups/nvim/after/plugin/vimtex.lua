vim.g.vimtex_compiler_latexmk = { out_dir = 'out' }
vim.g.vimtex_view_method = 'zathura'
vim.cmd [[ autocmd FileType tex execute 'silent !mkdir -p out' ]]
