local lsp = require('lsp-zero').preset({})

lsp.on_attach(function(client, bufnr)
    lsp.default_keymaps({ buffer = bufnr })

    vim.keymap.set('n', '<leader>ca', '<cmd>lua vim.lsp.buf.code_action()<cr>')
    vim.keymap.set('n', 'K', '<cmd>lua vim.lsp.buf.hover()<cr>')
end)

lsp.ensure_installed({
    'tsserver',
    'eslint',
    'rust_analyzer',
    'lua_ls',
})

lsp.format_on_save({
    format_opts = {
        async = false,
        timeout_ms = 10000,
    },
    servers = {
        ['lua_ls'] = { 'lua' },
        ['rust_analyzer'] = { 'rust' },
    }
})

local lspconfig = require('lspconfig')

-- Configure lua language server for neovim
lspconfig.lua_ls.setup(lsp.nvim_lua_ls())

-- Run eslint format on save
lspconfig.eslint.setup({
    on_attach = function(client, bufnr)
        vim.api.nvim_create_autocmd("BufWritePre", {
            buffer = bufnr,
            command = "EslintFixAll",
        })
    end,
})

-- Latex setup
lspconfig.texlab.setup({
    settings = {
        texlab = {
            build = {
                onSave = true,
            },
            chktex = {
                onOpenAndSave = true,
            },
        }
    }
})


-- Typescript language server commands
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

lsp.setup()

-- You need to setup `cmp` after lsp-zero
local cmp = require('cmp')
local cmp_action = require('lsp-zero').cmp_action()

cmp.setup({
    mapping = {
        -- `Enter` key to confirm completion
        ['<CR>'] = cmp.mapping.confirm({ select = false }),

        -- Ctrl+Space to trigger completion menu
        ['<C-Space>'] = cmp.mapping.complete(),

        -- Navigate between snippet placeholder
        ['<C-f>'] = cmp_action.luasnip_jump_forward(),
        ['<C-b>'] = cmp_action.luasnip_jump_backward(),
    }
})
