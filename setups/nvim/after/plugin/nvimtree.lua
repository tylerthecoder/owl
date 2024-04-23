local function grep_at_current_tree_node()
    local node = require('nvim-tree.lib').get_node_at_cursor()
    if not node then return end
    require('telescope.builtin').live_grep({ search_dirs = { node.absolute_path } })
end

vim.keymap.set('n', '<leader>ft', '<cmd>NvimTreeToggle<CR>', { noremap = true, silent = true })
vim.keymap.set('n', '<leader>ff', '<cmd>NvimTreeFindFile<CR>', { noremap = true, silent = true })
vim.keymap.set('n', '<leader>fg', grep_at_current_tree_node, { noremap = true, silent = true })

require 'nvim-web-devicons'.setup {}

require('nvim-tree').setup({
    actions = {
        open_file = {
            quit_on_open = true,
        },
    },
})
