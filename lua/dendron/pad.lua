local dendron = require('namjul/dendron')
-- local cmd = require('namjul/dendron.cmd')
local telescope = require('namjul/dendron/_telescope')

dendron.setup({
  dendron_dir = '~/Dropbox/dendron',
  -- virtual_titles = true,
  -- mappings = true,
  -- run = nil, -- function to run when in neuron dir
  -- neuron_dir = '~/neuron', -- the directory of all of your notes, expanded by default (currently supports only one directory for notes, find a way to detect neuron.dhall to use any directory)
  -- leader = 'gz', -- the leader key to for all mappings, remember with 'go zettel'
})

-- cmd.lookup({ query = 'hello', vault = 'wiki' }, dendron.config.dendron_dir, function(data)
--   -- print(vim.inspect(data))
--   print(vim.inspect(vim.fn.keys(data)))
-- end)
-- cmd.delete({ query = 'hello', vault = 'wiki' }, dendron.config.dendron_dir, function(data)
--   print(data)
-- end)

-- telescope.lookup(require('telescope.themes').get_ivy())
