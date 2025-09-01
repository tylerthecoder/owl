return {
    -- LSP Configuration
    {
        'neovim/nvim-lspconfig',
        dependencies = {
            'williamboman/mason.nvim',
            'williamboman/mason-lspconfig.nvim',
            'hrsh7th/cmp-nvim-lsp',
        },
        config = function()
            vim.wo.signcolumn = 'yes'

            vim.api.nvim_create_autocmd('LspAttach', {
                desc = 'LSP actions',
                callback = function(event)
                    vim.keymap.set('n', 'gd', function() vim.lsp.buf.definition() end)
                    vim.keymap.set('n', 'gD', '<cmd>lua vim.lsp.buf.declaration()<cr>')
                    vim.keymap.set('n', 'gi', '<cmd>Telescope lsp_implementations<cr>')
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
                    'ts_ls',
                    -- 'eslint', -- Disabled due to compatibility issues with Neovim 0.12-dev
                    'rust_analyzer',
                    'lua_ls',
                    'texlab',
                    'ltex',
                    'pyright',
                    'clangd'
                }
            })

            local lsp_capabilities = require('cmp_nvim_lsp').default_capabilities()

            -- Configure LSP servers using the new vim.lsp.config approach

            -- TypeScript/JavaScript
            vim.lsp.config('ts_ls', {
                capabilities = lsp_capabilities,
            })
            vim.lsp.enable('ts_ls')

            -- Tailwind CSS
            vim.lsp.config('tailwindcss', {
                capabilities = lsp_capabilities,
            })
            vim.lsp.enable('tailwindcss')

            -- Lua
            vim.lsp.config('lua_ls', {
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
            vim.lsp.enable('lua_ls')

            -- Rust
            vim.lsp.config('rust_analyzer', {
                capabilities = lsp_capabilities,
            })
            vim.lsp.enable('rust_analyzer')

            -- LaTeX
            vim.lsp.config('texlab', {
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
            vim.lsp.enable('texlab')

            -- Grammar checking (Markdown/LaTeX)
            vim.lsp.config('ltex', {
                capabilities = lsp_capabilities,
            })
            vim.lsp.enable('ltex')

            -- C/C++
            vim.lsp.config('clangd', {
                capabilities = lsp_capabilities,
            })
            vim.lsp.enable('clangd')

            -- Python
            vim.lsp.config('pyright', {
                capabilities = lsp_capabilities,
            })
            vim.lsp.enable('pyright')

            do
                local ok_lsp, lsp = pcall(require, "lspconfig")
                if ok_lsp and lsp.eslint and type(lsp.eslint.setup) == "function" then
                    local orig = lsp.eslint.setup
                    lsp.eslint.setup = function(...)
                        local src = debug.getinfo(2, "S").source
                        vim.schedule(function()
                            vim.notify("eslint.setup called from: " .. tostring(src), vim.log.levels.WARN)
                        end)
                        return orig(...)
                    end
                end
            end
        end,
    },
}
