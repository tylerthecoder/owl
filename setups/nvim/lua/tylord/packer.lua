-- This file can be loaded by calling `lua require('plugins')` from your init.vim

-- Only required if you have packer configured as `opt`
vim.cmd [[packadd packer.nvim]]

return require('packer').startup(function(use)
    -- Packer can manage itself
    use 'wbthomason/packer.nvim'

    use {
        'nvim-telescope/telescope.nvim', tag = '0.1.2',
        requires = { { 'nvim-lua/plenary.nvim' } }
    }
    use "nvim-lua/plenary.nvim"

    use { "catppuccin/nvim", as = "catppuccin" }

    -- Treesitter
    use({ 'nvim-treesitter/nvim-treesitter', run = ':TSUpdate' })
    use 'nvim-treesitter/playground'

    use 'mbbill/undotree'

    use 'github/copilot.vim'

    -- LSP
    use 'neovim/nvim-lspconfig'
    use 'williamboman/mason.nvim'
    use 'williamboman/mason-lspconfig.nvim'

    -- Completions
    use 'hrsh7th/nvim-cmp'
    use 'hrsh7th/cmp-nvim-lsp'
    use 'hrsh7th/cmp-buffer'
    use 'hrsh7th/cmp-path'
    use 'hrsh7th/cmp-cmdline'
    use 'hrsh7th/cmp-vsnip' -- snippits
    use 'hrsh7th/vim-vsnip'

    use {
        'numToStr/Comment.nvim',
        config = function()
            require('Comment').setup()
        end
    }

    use {
        'nvim-tree/nvim-tree.lua',
        requires = {
            'nvim-tree/nvim-web-devicons', -- optional
        },
    }

    -- install without yarn or npm
    use({
        "iamcco/markdown-preview.nvim",
        run = function() vim.fn["mkdp#util#install"]() end,
    })

    use "mhinz/vim-startify"

    use "tpope/vim-surround"

    use "tpope/vim-fugitive"

    -- Latex
    use "lervag/vimtex"

    -- Lean
    use "Julian/lean.nvim"

    -- Smooth Scrolling
    use 'karb94/neoscroll.nvim'

    use 'vim-airline/vim-airline'

    -- Editor Config
    use 'editorconfig/editorconfig-vim'
end)
