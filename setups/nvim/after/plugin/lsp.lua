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
    }
})


--=======================
-- Configure LSPS
--======================

local lspconfig = require('lspconfig')
local lsp_capabilities = require('cmp_nvim_lsp').default_capabilities()
local format_sync_grp = vim.api.nvim_create_augroup("Format", {})

-- Typescript setup
lspconfig.tsserver.setup {
    capabilities = lsp_capabilities,
}
local function rename_file()
    local source = vim.uri_from_bufnr(0)
    vim.ui.input({
        prompt = "Move to file: ",
        completion = "file",
        default = vim.uri_to_fname(source),
    }, function(path)
        print("You entered: " .. path)
        local params = {
            command = "_typescript.applyRenameFile",
            arguments = { {
                sourceUri = source,
                targetUri = vim.uri_from_fname(path),
            } },
        }
        vim.lsp.buf.execute_command(params)
    end)
end
vim.keymap.set('n', '<leader>oi', rename_file)

-- Run eslint format on save
lspconfig.eslint.setup({
    capabilities = lsp_capabilities,
    on_attach = function(client, bufnr)
        vim.api.nvim_create_autocmd("BufWritePre", {
            buffer = bufnr,
            command = "EslintFixAll",
        })
    end,
})

-- Lua setup
lspconfig.lua_ls.setup({
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
vim.api.nvim_create_autocmd("BufWritePre", {
    pattern = "*.lua",
    callback = function()
        vim.lsp.buf.format({ timeout_ms = 200 })
    end,
    group = format_sync_grp,
})

-- Rust setup
lspconfig.rust_analyzer.setup({
    capabilities = lsp_capabilities,
})
vim.api.nvim_create_autocmd("BufWritePre", {
    pattern = "*.rs",
    callback = function()
        vim.lsp.buf.format({ timeout_ms = 200 })
    end,
    group = format_sync_grp,
})


-- Latex setup
lspconfig.texlab.setup({
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
vim.api.nvim_create_autocmd("BufWritePre", {
    pattern = "*.tex",
    callback = function()
        vim.lsp.buf.format({ timeout_ms = 200 })
    end,
    group = format_sync_grp,
})

-- Markdown grammar
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

require('lint').linters_by_ft = {
    markdown = { 'vale', }
}
vim.api.nvim_create_autocmd({ "BufWritePost" }, {
    callback = function()
        require("lint").try_lint()
    end,
})
