vim.wo.signcolumn = 'yes'

vim.api.nvim_create_autocmd('LspAttach', {
    desc = 'LSP actions',
    callback = function(event)
        vim.keymap.set('n', 'gd', '<cmd>Telescope lsp_definitions<cr>')
        vim.keymap.set('n', 'gD', '<cmd>lua vim.lsp.buf.declaration()<cr>')
        vim.keymap.set('n', 'gi', '<cmd>Telescope lsp_impementations<cr>')
        vim.keymap.set('n', 'go', '<cmd>Telescope lsp_type_definition<cr>')
        vim.keymap.set('n', 'gr', '<cmd>Telescope lsp_references<cr>')
        vim.keymap.set('n', 'gs', '<cmd>lua vim.lsp.buf.signature_help()<cr>')
        vim.keymap.set('n', '<F2>', '<cmd>lua vim.lsp.buf.rename()<cr>')
        vim.keymap.set('n', 'gl', '<cmd>lua vim.diagnostic.open_float()<cr>')
        vim.keymap.set('n', '[d', '<cmd>lua vim.diagnostic.goto_prev()<cr>')
        vim.keymap.set('n', ']d', '<cmd>lua vim.diagnostic.goto_next()<cr>')
        vim.keymap.set('n', '<leader>ca', '<cmd>lua vim.lsp.buf.code_action()<cr>')
        vim.keymap.set('n', 'K', '<cmd>lua vim.lsp.buf.hover()<cr>')
    end
})

require('mason').setup()
require('mason-lspconfig').setup({
    ensure_installed = {
        'tsserver',
        'eslint',
        'rust_analyzer',
        'lua_ls',
        'texlab',
        'ltex',
        'pyright',
        'clangd'
    }
})

--=======================
-- Configure LSPS
--======================

local lspconfig = require('lspconfig')
local lsp_capabilities = require('cmp_nvim_lsp').default_capabilities()

-- Typescript setup
lspconfig.tsserver.setup {
    capabilities = lsp_capabilities,
}

-- Tailwind setup
lspconfig.tailwindcss.setup({
    capabilities = lsp_capabilities,
})

-- Lua setup
lspconfig.lua_ls.setup({
    capabilities = lsp_capabilities,
    settings = {
        Lua = {
            runtime = {
                version = 'LuaJIT'
            },
            diagnostics = {
                globals = { 'vim' },
            },
            workspace = {
                library = {
                    vim.env.VIMRUNTIME,
                }
            }
        }
    }
})

-- Rust setup
lspconfig.rust_analyzer.setup({
    capabilities = lsp_capabilities,
})

-- Latex setup
lspconfig.texlab.setup({
    capabilities = lsp_capabilities,
    settings = {
        texlab = {
            build = {
                onSave = true,
                args = { "-outdir=out", "-pdf", "-interaction=nonstopmode", "-synctex=1", "%f" },
                auxDirectory = "./out",
                logDirectory = "./out",
                pdfDirectory = "./out",
            },
            forwardSearch = {
                executable = "zathura",
                args = { "--synctex-forward", "%l:1:%f", "%p" },
            },
            chktex = {
                onOpenAndSave = true,
            },
        }
    }
})

-- Markdown / LaTeX grammar
lspconfig.ltex.setup({
    capabilities = lsp_capabilities,
})

-- C setup
lspconfig.clangd.setup({
    capabilities = lsp_capabilities,
})

-- Python
lspconfig.pyright.setup({
    capabilities = lsp_capabilities,
})
