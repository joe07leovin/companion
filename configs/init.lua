-- Plugin manager bootstrap (lazy.nvim)
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({ "git", "clone", "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git", "--branch=stable", lazypath })
end
vim.opt.rtp:prepend(lazypath)

-- The basics (your old settings, translated)
vim.g.mapleader = " "            -- leader key = space (for shortcuts)
vim.opt.number = true
vim.opt.relativenumber = true

-- Plugins: colorscheme, file explorer, statusline, syntax highlighting
require("lazy").setup({
  { "folke/tokyonight.nvim" },
  { "nvim-tree/nvim-tree.lua", dependencies = { "nvim-tree/nvim-web-devicons" } },
  { "nvim-lualine/lualine.nvim" },
  { "nvim-treesitter/nvim-treesitter", build = ":TSUpdate" },
})

require("nvim-tree").setup({})

vim.cmd.colorscheme("tokyonight")
require("lualine").setup()

-- Treesitter: real syntax highlighting
vim.g.mapleader = " "
vim.keymap.set("n", "<leader>e", ":NvimTreeToggle<CR>", { desc = "File explorer" })
