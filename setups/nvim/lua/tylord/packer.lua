-- This file can be loaded by calling `lua require('plugins')` from your init.vim

-- Only required if you have packer configured as `opt`
vim.cmd [[packadd packer.nvim]]

return require('packer').startup(function(use)
    -- Packer can manage itself
    use 'wbthomason/packer.nvim'

    use {
        'nvim-telescope/telescope.nvim',
        requires = { { 'nvim-lua/plenary.nvim' } }
    }
    use "nvim-lua/plenary.nvim"

    -- Pretty UI
    use { "catppuccin/nvim", as = "catppuccin" }
    use { 'stevearc/dressing.nvim' }
    use { 'nvim-tree/nvim-web-devicons' }

    -- Treesitter
    use({ 'nvim-treesitter/nvim-treesitter', run = ':TSUpdate' })
    use 'nvim-treesitter/playground'
    use {
        'nvim-tree/nvim-tree.lua',
        requires = {
            'nvim-tree/nvim-web-devicons', -- optional
        },
    }
    use({
        "nvim-treesitter/nvim-treesitter-textobjects",
        after = "nvim-treesitter",
        requires = "nvim-treesitter/nvim-treesitter",
    })

    use 'mbbill/undotree'

    -- Harpoon
    use {
        "ThePrimeagen/harpoon",
        branch = "harpoon2",
        requires = { { "nvim-lua/plenary.nvim" } }
    }

    -- AI
    use { "zbirenbaum/copilot.lua" }
    use {
        "zbirenbaum/copilot-cmp",
        after = { "copilot.lua" },
        config = function()
            require("copilot_cmp").setup()
        end
    }
    use {
        "melbaldove/llm.nvim",
        requires = { "nvim-neotest/nvim-nio" },
    }
    -- use { 'huggingface/llm.nvim' }
    -- use 'github/copilot.vim'

    -- LSP
    use 'neovim/nvim-lspconfig'
    use 'williamboman/mason.nvim'
    use 'williamboman/mason-lspconfig.nvim'

    -- Lint
    use 'mfussenegger/nvim-lint'

    -- Formatting
    use { 'stevearc/conform.nvim' }

    -- Completions
    use 'hrsh7th/nvim-cmp'
    use 'hrsh7th/cmp-nvim-lsp'
    use 'hrsh7th/cmp-buffer'
    use 'hrsh7th/cmp-path'
    use 'hrsh7th/cmp-cmdline'
    use 'hrsh7th/cmp-vsnip' -- snippits
    use 'hrsh7th/vim-vsnip'

    use 'onsails/lspkind-nvim'

    use {
        'numToStr/Comment.nvim',
        config = function()
            require('Comment').setup()
        end
    }


    -- Folding
    use { 'kevinhwang91/nvim-ufo', requires = 'kevinhwang91/promise-async' }

    -- install without yarn or npm
    use({
        "iamcco/markdown-preview.nvim",
        run = function() vim.fn["mkdp#util#install"]() end,
    })

    use "mhinz/vim-startify"

    use "tpope/vim-surround"

    -- Git client
    use "tpope/vim-fugitive"

    -- Lean
    use "Julian/lean.nvim"

    -- Smooth Scrolling
    use 'karb94/neoscroll.nvim'

    use 'vim-airline/vim-airline'
end)
