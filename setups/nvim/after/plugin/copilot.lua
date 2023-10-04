require('copilot').setup({
    suggestion = { enabled = false },
    panel = { enabled = false },
    filetypes = {
        yaml = true,
        markdown = true,
        help = true,
        gitcommit = true,
        gitrebase = true,
        hgcommit = false,
        svn = false,
        cvs = false,
        ["."] = false,
    },
    copilot_node_command = 'node', -- Node.js version must be > 16.x
    server_opts_overrides = {},
})

function ToggleCopilot()
    local client = require("copilot.client")
    local command = require("copilot.command")

    if client.buf_is_attached(0) then
        command.detach()
        vim.notify('Copilot Disabled')
        return
    end

    vim.notify('Copilot Enabled')
    command.attach()
end

vim.keymap.set('n', '<leader>ai',
    '<cmd> lua ToggleCopilot()<cr>', {
        noremap = true,
    })
