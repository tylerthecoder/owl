return {
  -- GitHub Copilot
  {
    "zbirenbaum/copilot.lua",
    config = function()
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

      vim.keymap.set('n', '<leader>cp',
        '<cmd> lua ToggleCopilot()<cr>', {
          noremap = true,
        })
    end,
  },

  -- Copilot CMP integration
  {
    "zbirenbaum/copilot-cmp",
    dependencies = { "copilot.lua" },
    config = function()
      require("copilot_cmp").setup()
    end
  },

  -- LLM Integration
  {
    "melbaldove/llm.nvim",
    dependencies = { "nvim-neotest/nvim-nio" },
    config = function()
      local system_prompt =
        'You should replace the code that you are sent, only following the comments. Do not talk at all. Only output valid code. Do not provide any backticks that surround the code. Never ever output backticks like this ```. Any comment that is asking you for something should be removed after you satisfy them. Other comments should left alone. Do not output backticks'
      local helpful_prompt =
        'You are a helpful assistant. What I have sent are my notes so far. You are very curt, yet helpful.'

      require('llm').setup {
        timeout_ms = 3000,
        services = {
          groq = {
            url = 'https://api.groq.com/openai/v1/chat/completions',
            model = 'llama3-70b-8192',
            api_key_name = 'GROQ_API_KEY',
            system_prompt = system_prompt,
          },
          groq_help = {
            url = 'https://api.groq.com/openai/v1/chat/completions',
            model = 'llama3-70b-8192',
            api_key_name = 'GROQ_API_KEY',
            system_prompt = helpful_prompt,
          },
          openai = {
            url = 'https://api.openai.com/v1/chat/completions',
            model = 'gpt-4o',
            api_key_name = 'OPENAI_API_KEY',
          },
          openai_help = {
            url = 'https://api.openai.com/v1/chat/completions',
            model = 'gpt-4o',
            api_key_name = 'OPENAI_API_KEY',
          },
          claude = {
            url = 'https://api.anthropic.com/v1/messages',
            model = 'claude-3-5-sonnet-20240620',
            api_key_name = 'ANTHROPIC_API_KEY',
            system_prompt = system_prompt,
          },
          claude_help = {
            url = 'https://api.anthropic.com/v1/messages',
            model = 'claude-3-5-sonnet-20240620',
            api_key_name = 'ANTHROPIC_API_KEY',
            system_prompt = helpful_prompt,
          },
        },
      }

      vim.keymap.set('v', '<leader>k', function()
        require('llm').prompt { replace = true, service = 'groq' }
      end, { desc = 'Prompt with groq (replace = true)' })

      vim.keymap.set('v', '<leader>K', function()
        require('llm').prompt { replace = false, service = 'groq_help' }
      end, { desc = 'Prompt with groq (replace = false)' })

      vim.keymap.set('v', '<leader>L', function()
        require('llm').prompt { replace = false, service = 'openai_help' }
      end, { desc = 'Prompt with openai (replace = false)' })

      vim.keymap.set('v', '<leader>l', function()
        require('llm').prompt { replace = true, service = 'openai' }
      end, { desc = 'Prompt with openai (replace = true)' })

      vim.keymap.set('n', '<leader>I', function()
        require('llm').prompt { replace = false, service = 'anthropic' }
      end, { desc = 'Prompt with anthropic (replace = false)' })

      vim.keymap.set('n', '<leader>i', function()
        require('llm').prompt { replace = true, service = 'anthropic_help' }
      end, { desc = 'Prompt with anthropic (replace = true)' })
    end,
  },
}