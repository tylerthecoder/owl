return {
  -- Harpoon
  {
    "ThePrimeagen/harpoon",
    branch = "harpoon2",
    dependencies = { "nvim-lua/plenary.nvim" },
    config = function()
      local harpoon = require("harpoon")
      harpoon:setup()

      vim.keymap.set("n", "<C-a>", function() harpoon:list():append() end)
      vim.keymap.set("n", "<C-h>", function() harpoon.ui:toggle_quick_menu(harpoon:list()) end)

      -- Toggle previous & next buffers stored within Harpoon list
      vim.keymap.set("n", "<C-P>", function() harpoon:list():prev() end)
      vim.keymap.set("n", "<C-N>", function() harpoon:list():next() end)
    end,
  },

  -- File tree
  {
    'nvim-tree/nvim-tree.lua',
    dependencies = { 'nvim-tree/nvim-web-devicons' },
    config = function()
      require("nvim-tree").setup()
      local function grep_at_current_tree_node()
          local node = require('nvim-tree.api').tree.get_node_under_cursor()
          if not node then return end
          require('telescope.builtin').live_grep({ search_dirs = { node.absolute_path } })
      end

      vim.keymap.set('n', '<leader>ft', '<cmd>NvimTreeToggle<CR>', { noremap = true, silent = true })
      vim.keymap.set('n', '<leader>ff', '<cmd>NvimTreeFindFile<CR>', { noremap = true, silent = true })
      vim.keymap.set('n', '<leader>fg', grep_at_current_tree_node, { noremap = true, silent = true })
    end,
  },

  -- Undotree
  {
    'mbbill/undotree',
    config = function()
      vim.keymap.set("n", "<leader>u", vim.cmd.UndotreeToggle)
    end,
  },
}